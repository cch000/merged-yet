mod pr_finder;

use std::{process::ExitCode, thread};

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
    ///Whether to output if the pr was first merged into master
    full: bool,

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

fn main() -> ExitCode {
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

    if pr_number < 300000 {
        println!("pr is too old");
        return ExitCode::from(1);
    }

    //Program output
    if args.scripting {
        let pr_found = pr_finder::find_pr(pr_number, max_pages, &branch, &key, threads);

        if pr_found {
            ExitCode::from(0)
        } else {
            ExitCode::from(1)
        }
    } else {
        println!("#{}", pr_number);

        if args.full {
            let pr_master = pr_finder::find_pr(pr_number, max_pages, "master", &key, threads);

            if pr_master {
                println!("├ ✅ master");
            } else {
                println!("├ ❌ master");
            }
        }

        let pr_found = pr_finder::find_pr(pr_number, max_pages, &branch, &key, threads);

        if pr_found {
            println!("├ ✅ {}", branch);
        } else {
            println!("├ ❌ {}", branch);
        };

        ExitCode::from(0)
    }
}
