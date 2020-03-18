extern crate opener;
extern crate colored;

use url::Url;
use reqwest::*;
use serde::{Deserialize, Serialize};
use std::env;
use colored::*;


pub fn open_pr() {
    let pr = get_current_pr().unwrap();
    let pr_link = pr.links.html.href.clone();
    opener::open(pr_link);
}

pub fn status_pr() {
    let pr = get_current_pr().unwrap();
    print_pr(&pr);
}

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

pub fn list_pr() {
    println!("{}", "Your open pull requests".bold());
    println!();
    let prs = get_current_user_prs().unwrap();
    for pr in prs.iter() {
        print_pr(pr);
        println!();
    }
}

fn print_pr(pr: &PullRequest) {
    let pr_link = pr.links.html.href.clone();
    println!("{}", pr.title.bold().underline());
    println!("{} -> {}", pr.source.branch.name, pr.destination.branch.name);
//    println!("{}: {}", "Description".bold(), pr.description);
    println!("{}: {}", "State".bold(), pr.state);
    println!("{}  {}", "🔗".bold(), pr_link);
}

fn print_line() {
    println!("--------------------------------");
}

fn get_api_base_url() -> std::option::Option<String> {
    let remote_url = super::git::git_remote();
    if let Ok(parsed) = Url::parse(&remote_url) {
        let username = env::var("BB_CLI_USERNAME").unwrap();
        let password = env::var("BB_CLI_PASSWORD").unwrap();

        let url = format!("{}://{}:{}@api.{}",
                          parsed.scheme().to_owned(),
                          username,
                          password,
                          parsed.host_str().unwrap());
        return Some(url);
    } else {
        return None;
    }
}

fn get_current_pr() -> std::result::Result<PullRequest, reqwest::Error> {
    let current_branch = super::git::git_current_branch();
    let remote_url = super::git::git_remote();

    if let Some(base_url) = get_api_base_url() {
        if let Ok(parsed) = Url::parse(&remote_url) {
            let slug = parsed.path().split(".git").collect::<Vec<_>>().join("");

            let pr_search_api_url = base_url
                + "/2.0/repositories"
                + &slug
                + "/pullrequests?q=source.branch.name~\""
                + current_branch.trim() + "\"";

            let response: PullRequestsResponse = reqwest::blocking::get(pr_search_api_url.as_str())?.json()?;

            if response.values.len() > 0 {
                let pr = response.values.first().unwrap().to_owned();
                return Ok(pr);
            }
        }
    }


    panic!("Failed to find the PR link")
}

fn get_current_user_prs() -> std::result::Result<Vec<PullRequest>, reqwest::Error> {
    if let Some(base_url) = get_api_base_url() {
        let username = env::var("BB_CLI_USERNAME").unwrap();
        let full_url = format!("{}/2.0/pullrequests/{}?state=OPEN", base_url, username);

        let response: PullRequestsResponse = reqwest::blocking::get(full_url.as_str())?.json()?;

        return Ok(response.values);
    }

    panic!("Failed to find the PR link")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PullRequestsResponse {
    values: Vec<PullRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PullRequest {
    title: String,
    description: String,
    state: String,
    links: PullRequestLinks,
    source: PullRequestBranch,
    destination: PullRequestBranch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PullRequestLinks {
    #[serde(rename = "self")]
    self_link: Link,
    html: Link,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Link {
    href: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Branch {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Repository {
    full_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PullRequestBranch {
    branch: Branch,
    repository: Repository,

}

