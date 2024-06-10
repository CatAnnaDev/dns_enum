use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "PsykoDev", version, about, long_about = None)]
pub struct Args {
    #[arg(
        short = 'd',
        long,
        help = "domain name \"exemple.com\""
    )]
    pub domain: String,

    #[arg(
        short = 'f',
        long,
        help = "wordlist \"wordlist/subdomain.txt\""
    )]
    pub word_list_path: String,

    #[arg(
        short = 't',
        long,
        default_value_t = 1,
        help = "Thread use to scan max 5"
    )]
    pub thread: usize,
}