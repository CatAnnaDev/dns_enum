mod thread_pool;

use std::env;
use std::error::Error;
use std::fs::read_to_string;
use std::io::Write;
use std::ops::Index;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::error::ResolveResult;
use trust_dns_resolver::lookup::TxtLookup;

fn main() -> Result<(), Box<dyn Error>> {
    let arg = env::args().collect::<Vec<String>>();
    let url = arg.index(1).to_string();
    let path = arg.index(2).to_string();

    let x = read_to_string(path)?;
    let mut t = vec![];
    for x in x.lines() {
        let url = url.clone();
        t.push(format!("{x}.{url}"))
    }

    let total_requests = t.len();
    let mut pool = thread_pool::ThreadPool::new(2, total_requests);
    let active_jobs = Arc::clone(&pool.active_jobs);
    let completed_requests = Arc::new(Mutex::new(0));
    let start_time = Instant::now();

    for domain in t {
        let active_jobs = Arc::clone(&active_jobs);
        let completed_requests = Arc::clone(&completed_requests);
        pool.execute(move || {
            let resolver = Resolver::new(
                ResolverConfig::default(),
                ResolverOpts::default()
            ).unwrap();

            display_txt(&*domain.clone(), &resolver.txt_lookup(domain.clone()));

            let mut completed_requests = completed_requests.lock().unwrap();
            *completed_requests += 1;
            let mut active_jobs = active_jobs.lock().unwrap();
            *active_jobs -= 1;
        });
        //sleep(Duration::from_secs(1));
    }

    loop {
        let elapsed = start_time.elapsed().as_secs_f64();
        let completed = *completed_requests.lock().unwrap();
        let remaining = total_requests - completed;

        if elapsed > 0.0 {
            let rps = completed as f64 / elapsed;
            print!("\r{:.2} / s | {completed}/{remaining} ",rps);
            std::io::stdout().flush().unwrap();
        }

        if remaining == 0 {
            break;
        }
    }

    println!("\nAll tasks are completed.");
    Ok(())
}

fn display_txt(query: &str, txt_response: &ResolveResult<TxtLookup>) {
    match txt_response {
        Err(_) => {  },
        Ok(txt_response) => {
            println!("{}", &query);
            for record in txt_response.iter() {
                if !record.to_string().starts_with("v=spf1") {
                    println!("{}", &record.to_string());
                }
            }
        }
    }
}
