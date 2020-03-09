extern crate reqwest;
extern crate opener;

use url::Url;
use reqwest::*;
use serde::{Deserialize, Serialize};

pub fn get_pr_link() {
    println!("get_pr_link begings");

//    let current_branch = super::git::git_current_branch();
    let current_branch = "gh-pages";
    if let Some(base_url) = get_api_base_url() {
        let pr_api_url = base_url + "pullrequests?q=source.branch.name~\"" + current_branch.trim() + "\"";
        let pr_link = get_current_pr(pr_api_url).unwrap();
        opener::open(pr_link);
    }
}

pub fn get_api_base_url() -> std::option::Option<String> {
    let remote_url = super::git::git_remote();
    if let Ok(parsed) = Url::parse(&remote_url) {
        let slug = parsed.path().split(".git").collect::<Vec<_>>().join("");
        let username = "xxxxx";
        let password = "xxxxx";
        let url = format!("{}://{}:{}@api.{}/2.0/repositories{}/",
                          parsed.scheme().to_owned(),
                          username,
                          password,
                          parsed.host_str().unwrap(),
                          &slug
        );
        return Some(url);
    } else {
        return None;
    }
}

pub fn get_current_pr(url: String) -> std::result::Result<String, reqwest::Error> {
    let response: PullRequestsResponse = reqwest::blocking::get(url.as_str())?.json()?;

    println!("body: {:#?}", response);


    if let pr = &response.values[0] {
        let pr_link = pr.links.html.href.clone();
        return Ok(pr_link);
    }

    panic!("Failed to find a pr link")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestsResponse {
    values: Vec<PullRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    description: String,
    links: PullRequestLinks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestLinks {
    #[serde(rename = "self")]
    self_link: Link,
    html: Link,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    href: String,
}

