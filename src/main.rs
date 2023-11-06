use std::env;
use reqwest;
use std::collections::VecDeque;
use std::sync::{Arc};
use tokio::time::Instant;
use tokio::sync::Mutex;
use tokio::spawn;
use std::fs::read_to_string;
use std::process::exit;
use std::time::Duration;

const WORDLIST: &str = "wordlist/subdomain.txt";

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: ./dns_enum domain.com");
        exit(0);
    }

    let mut wordlist = Vec::new();
    for line in read_to_string(WORDLIST).unwrap().lines() {
        let x = format!("https://{}.{}", line.trim().to_string(), args.last().unwrap());
        wordlist.push(x.clone());
    }


    let max_threads = 8;
    let url_queue = Arc::new(Mutex::new(VecDeque::from(wordlist)));

    for _ in 0..max_threads {
        let url_queue = Arc::clone(&url_queue);
        let handle = spawn(async move {
            while let Some(url) = url_queue.lock().await.pop_front() {
                let start_time = Instant::now();
                let result = fetch_url(&url).await;
                let elapsed_time = start_time.elapsed();
                match result {
                    Ok(response) => {
                            println!("URL: {}, Status: {:?}, Time: {:?}", url, response.status(), elapsed_time);
                    }
                    Err(_) => {
                    }
                }
            }
        });
        handle.await.expect("Erreur lors de l'exécution du thread");
    }

    loop {

    }

}

async fn fetch_url(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36")
        .timeout(Duration::from_secs(2))
        .send()
        .await?;

    Ok(res)
}