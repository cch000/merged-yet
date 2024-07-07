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

pub fn find_pr(pr_number: u32, max_pages: u16, branch: &str, api_key: Option<String>) -> bool {
    let mut headers = HeaderMap::new();

    headers.insert("User-agent", HeaderValue::from_static("curl"));

    if api_key.is_some() {
        let val = format!("Bearer {}", api_key.unwrap());

        headers.insert("Authorization", HeaderValue::from_str(&val).unwrap());
    }

    let client: Client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .expect("error building client");

    let base_url = format!(
        "https://api.github.com/repos/NixOS/nixpkgs/commits?sha={branch}&per_page=100&page="
    );

    let mut next = true;

    let mut pr_found = false;

    let mut page_number = 1;

    while next && !pr_found {
        let url = format!("{}{}", base_url, page_number);

        let res = client.get(url).send().expect("error getting response");

        let res_link_headers = res
            .headers()
            .get("Link")
            .expect("Github API rate limit was exceeded or Branch not found")
            .to_str()
            .unwrap();

        if res_link_headers.contains("next") && page_number <= max_pages {
            page_number += 1;
        } else {
            next = false;
        }

        let res_json: Vec<Object> = res.json().unwrap();

        for object in res_json {
            let message = object.commit.message;

            if message.contains(&pr_number.to_string()) {
                pr_found = true;
                break;
            }
        }
    }
    pr_found
}
