extern crate colored;
extern crate opener;

use anyhow::{anyhow, Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use std::env;
use url::Url;

pub fn view_pr() -> Result<()> {
    let pr = get_current_pr()?;
    let pr_link = pr.links.html.href.clone();

    opener::open(pr_link).context("Failed to open PR link")
}

pub fn status_pr() -> Result<()> {
    let pr = get_current_pr()?;
    print_pr(&pr);

    Ok(())
}

pub fn create_pr() -> Result<()> {
    let remote_url = super::git::git_remote();
    let parsed = Url::parse(&remote_url)?;
    let slug = parsed.path().split(".git").collect::<Vec<_>>().join("");
    let current_branch = super::git::git_current_branch();
    let create_pr_url = format!(
        "{}://{}{}/pull-requests/new?source={}",
        parsed.scheme().to_owned(),
        parsed.host_str().unwrap(),
        &slug,
        current_branch
    );

    opener::open(create_pr_url).context("Failed to open create PR link")
}

pub fn list_pr() -> Result<()> {
    println!("{}", "Your open pull requests".bold());
    println!();
    let prs = get_current_user_prs()?;
    for pr in prs.iter() {
        print_pr(pr);
        println!();
    }

    Ok(())
}

fn print_pr(pr: &PullRequest) {
    let pr_link = pr.links.html.href.clone();
    println!("{}", pr.title.bold().underline());
    println!(
        "{} -> {}",
        pr.source.branch.name, pr.destination.branch.name
    );
    //    println!("{}: {}", "Description".bold(), pr.description);
    println!("{}: {}", "State".bold(), pr.state);
    println!("{}  {}", "🔗".bold(), pr_link);
}

fn print_line() {
    println!("--------------------------------");
}

fn get_api_base_url() -> Result<String> {
    let remote_url = super::git::git_remote();
    let parsed = Url::parse(&remote_url).context("Failed to parse git remote url")?;
    let username = env::var("BB_CLI_USERNAME").context("BB_CLI_USERNAME environment isn't set")?;
    let password =
        env::var("BB_CLI_PASSWORD").context("BB_CLI_PASSWORD environment variable isn't set")?;

    let url = format!(
        "{}://{}:{}@api.{}",
        parsed.scheme().to_owned(),
        username,
        password,
        parsed.host_str().unwrap()
    );
    Ok(url)
}

fn get_current_pr() -> Result<PullRequest> {
    let current_branch = super::git::git_current_branch();
    let remote_url = super::git::git_remote();

    let base_url = get_api_base_url()?;
    let parsed = Url::parse(&remote_url)?;
    let slug = parsed.path().split(".git").collect::<Vec<_>>().join("");

    let pr_search_api_url = base_url
        + "/2.0/repositories"
        + &slug
        + "/pullrequests?q=source.branch.name~\""
        + current_branch.trim()
        + "\"";

    let response: PullRequestsResponse =
        reqwest::blocking::get(pr_search_api_url.as_str())?.json()?;

    response
        .values
        .first()
        .cloned()
        .ok_or(anyhow!("No current PR found"))
}

fn get_current_user_prs() -> Result<Vec<PullRequest>> {
    let base_url = get_api_base_url()?;
    let username = env::var("BB_CLI_USERNAME")?;
    let full_url = format!("{}/2.0/pullrequests/{}?state=OPEN", base_url, username);

    let response: PullRequestsResponse = reqwest::blocking::get(full_url.as_str())?.json()?;

    Ok(response.values)
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
