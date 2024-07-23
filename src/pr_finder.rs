use std::sync::atomic::AtomicBool;
use std::{sync::atomic::Ordering, thread};

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Commit {
    message: String,
}
#[derive(Deserialize)]
struct Object {
    commit: Commit,
}

pub fn find_pr(
    pr_number: u32,
    max_pages: u32,
    branch: &str,
    api_key: &Option<String>,
    threads: u32,
) -> bool {
    let mut headers = HeaderMap::new();

    headers.insert("User-agent", HeaderValue::from_static("curl"));

    // If we have an api key, add it to the requests
    if let Some(api_key) = api_key {
        let val = format!("Bearer {api_key}");

        headers.insert("Authorization", HeaderValue::from_str(&val).unwrap());
    }

    let client: Client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .expect("error building client");

    // Atomic bool used as a stop flag
    let pr_found = &AtomicBool::new(false);

    thread::scope(|scope| {
        for i in 0..threads {
            let client = client.clone();
            scope.spawn(move || {
                let mut first_iter = true;
                let mut page_number = 1;

                //Loop that asigns different pages to each of the threads
                while !pr_found.load(Ordering::Relaxed) && page_number <= max_pages {
                    if first_iter {
                        page_number += i;
                    }

                    if send_req(&client, pr_number, page_number, branch) {
                        pr_found.store(true, Ordering::Relaxed);
                    }

                    page_number += threads;
                    first_iter = false;
                }
            });
        }
    });

    pr_found.load(Ordering::Relaxed)
}

fn send_req(client: &Client, pr_number: u32, page_number: u32, branch: &str) -> bool {
    let base_url = format!(
        "https://api.github.com/repos/NixOS/nixpkgs/commits?sha={branch}&per_page=100&page="
    );

    let url = format!("{}{}", base_url, page_number);

    let res = client.get(url).send().expect("error getting response");

    let res_json: Vec<Object> = res.json().unwrap();

    //Iterator that returns true if it finds a commit that contains the pr number
    res_json.iter().any(|o| {
        o.commit
            .message
            .contains(&format!("#{}", &pr_number.to_string()))
    })
}
