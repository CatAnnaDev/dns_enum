mod thread_pool;
mod cmd_line_parser;

use std::fs::read_to_string;
use std::io::Write;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use clap::Parser;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};

fn main() -> anyhow::Result<()> {
    let new_args = cmd_line_parser::Args::parse();

    let url = if !new_args.domain.is_empty(){
        new_args.domain
    }else { println!("no domain name found"); exit(0); };
    let path = if !new_args.word_list_path.is_empty(){
        new_args.word_list_path
    }else { println!("no wordlist found"); exit(0); };
    let thread_count = if new_args.thread > 5 || new_args.thread <= 0{
        5
    }else { new_args.thread };

    let x = read_to_string(path)?;
    let mut t = vec![];
    for x in x.lines() {
        t.push(format!("{x}.{url}"))
    }

    let total_requests = t.len();

    let mut pool = thread_pool::ThreadPool::new(thread_count, total_requests);
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

            match resolver.txt_lookup(domain.clone()) {
                Err(_) => {},
                Ok(_) => {
                    println!("https://{}", &domain);
                }
            }
            let mut completed_requests = completed_requests.lock().unwrap();
            *completed_requests += 1;
            let mut active_jobs = active_jobs.lock().unwrap();
            *active_jobs -= 1;
        });
    }

    loop {
        let elapsed = start_time.elapsed().as_secs_f64();
        let completed = *completed_requests.lock().unwrap();
        let remaining = total_requests - completed;

        if elapsed > 0.0 {
            let rps = completed as f64 / elapsed;
            print!("\r{rps:.2} / s | {completed}/{total_requests} ");
            std::io::stdout().flush()?;
        }

        if remaining == 0 {
            break;
        }
    }

    println!("\nAll tasks are completed.");
    Ok(())
}