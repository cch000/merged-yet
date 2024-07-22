mod pr_finder;

use std::thread;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
#[command(
    about = "Simple tool to find out if a pr was merged into a nixpkgs branch. Uses the github api",
    long_about = None
)]
struct Args {
    #[arg(short, long)]
    #[arg(default_value_t = String::from("nixos-unstable"))]
    ///Branch in which to look for the pull request
    branch: String,

    #[arg(short, long)]
    ///Each page is one request [default: if no key was provided 5, else 100]
    max_pages: Option<u32>,

    #[arg(short, long)]
    ///Whether to output script-friendly values
    scripting: bool,

    #[arg(short, long)]
    /// Github api key
    api_key: Option<String>,

    #[arg(short, long)]
    #[arg(
        default_value_t = thread::available_parallelism()
        .expect("could not get number of available threads")
        .get()
        .try_into()
        .unwrap()
    )]
    ///Number of threads
    threads: u32,

    pr_number: u32,
}

fn main() {
    let args = Args::parse();

    let branch = args.branch;
    let pr_number = args.pr_number;
    let key = args.api_key;
    let threads = args.threads;

    //Set defaults or use the provided max_pages
    let max_pages = if args.max_pages.is_none() {
        //More requests by default if an api key was provided
        if key.is_some() {
            100
        } else {
            5
        }
    } else {
        args.max_pages.unwrap()
    };

    let pr_found = pr_finder::find_pr(pr_number, max_pages, &branch, &key, threads);

    let found_str = match args.scripting {
        true => String::from("true"),
        false => format!("pr #{} has been merged into {}", pr_number, branch),
    };

    let not_found_str = match args.scripting {
        true => String::from("false"),
        false => format!(
            "pr #{} could not be found in {}, try increasing the number of requests",
            pr_number, branch
        ),
    };

    if pr_found {
        println!("{}", found_str);
    } else {
        println!("{}", not_found_str);
    };
}
