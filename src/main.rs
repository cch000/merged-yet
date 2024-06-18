mod pr_finder;

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
    #[arg(default_value_t = 5)]
    ///Each page is one request. If the pr was not found, try increasing this value.
    max_pages: u8,

    #[arg(short, long)]
    ///Whether to output script-friendly values
    scripting: bool,

    pr_number: u32,
}

fn main() {
    let args = Args::parse();

    let branch = args.branch;
    let pr_number = args.pr_number;
    let max_pages = args.max_pages;

    let pr_found = pr_finder::find_pr(pr_number, max_pages, &branch);

    let found_str = match args.scripting {
        true => String::from("true"),
        false => format!(
            "pr #{} has been merged into {}",
            pr_number, branch
        ),
    };

    let not_found_str = match args.scripting {
        true => String::from("false"),
        false => format!(
            "pr #{} has not been merged yet into {}",
            pr_number, branch
        ),
    };

    if pr_found {
        println!("{}", found_str);
    } else {
        println!("{}", not_found_str);
    };
}
