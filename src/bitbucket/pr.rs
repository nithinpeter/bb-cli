extern crate opener;

use url::Url;
use reqwest::*;
use serde::{Deserialize, Serialize};
use std::env;

pub fn open_pr() {
    let current_branch = super::git::git_current_branch();
    if let Some(base_url) = get_api_base_url() {
        let pr_search_api_url = base_url + "pullrequests?q=source.branch.name~\"" + current_branch.trim() + "\"";
        let pr_link = get_current_pr(pr_search_api_url).unwrap();
        opener::open(pr_link);
    }
}

pub fn status_pr() {}

pub fn create_pr() {
    let remote_url = super::git::git_remote();
    if let Ok(parsed) = Url::parse(&remote_url) {
        let slug = parsed.path().split(".git").collect::<Vec<_>>().join("");
        let current_branch = super::git::git_current_branch();
        let create_pr_url = format!("{}://{}{}/pull-requests/new?source={}",
                                    parsed.scheme().to_owned(),
                                    parsed.host_str().unwrap(),
                                    &slug,
                                    current_branch);

        opener::open(create_pr_url);
    }
}

fn get_api_base_url() -> std::option::Option<String> {
    let remote_url = super::git::git_remote();
    if let Ok(parsed) = Url::parse(&remote_url) {
        let slug = parsed.path().split(".git").collect::<Vec<_>>().join("");
        let username = env::var("BB_CLI_USERNAME").unwrap();
        let password = env::var("BB_CLI_PASSWORD").unwrap();

        let url = format!("{}://{}:{}@api.{}/2.0/repositories{}/",
                          parsed.scheme().to_owned(),
                          username,
                          password,
                          parsed.host_str().unwrap(),
                          &slug);
        return Some(url);
    } else {
        return None;
    }
}

fn get_current_pr(url: String) -> std::result::Result<String, reqwest::Error> {
    let response: PullRequestsResponse = reqwest::blocking::get(url.as_str())?.json()?;

    if response.values.len() > 0 {
        let pr = &response.values[0];
        let pr_link = pr.links.html.href.clone();
        return Ok(pr_link);
    }


    panic!("Failed to find the PR link")
}

#[derive(Debug, Serialize, Deserialize)]
struct PullRequestsResponse {
    values: Vec<PullRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PullRequest {
    description: String,
    links: PullRequestLinks,
}

#[derive(Debug, Serialize, Deserialize)]
struct PullRequestLinks {
    #[serde(rename = "self")]
    self_link: Link,
    html: Link,
}

#[derive(Debug, Serialize, Deserialize)]
struct Link {
    href: String,
}

