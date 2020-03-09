extern crate reqwest;
extern crate tokio;
extern crate futures;

use url::Url;
use git::*;
use reqwest::*;
use futures::executor::block_on;

pub async fn get_pr_link() -> Result<String> {
    println!("get_pr_link begings");

    let current_branch = super::git::git_current_branch();
    if let Some(base_url) = get_api_base_url() {
        let pr_url = base_url + "pullrequests?q=destination.branch.name~\"" + current_branch.trim() + "\"";
        println!("{}", pr_url);

        println!("async begings");

        let res = block_on(get_current_pr(pr_url));
        println!("async ends");
    }

    panic!("Failed to fetch");
}

pub fn get_api_base_url() -> std::option::Option<String> {
    let remote_url = super::git::git_remote();
    if let Ok(parsed) = Url::parse(&remote_url) {
        let slug = parsed.path().split(".git").collect::<Vec<_>>().join("");
        let username = "xxxxx";
        let password = "xxxxx";
//        ?q=target.branch.name~"master"
        let url = format!("{}://{}:{}@api.{}/2.0/repositories/{}",
                          parsed.scheme().to_owned(),
                          username,
                          password,
                          parsed.host_str().unwrap(),
                          &slug
        );
        return Some(url);
//        return Some(parsed.scheme().to_owned() + "://api." + parsed.host_str().unwrap() + "/2.0/repositories" + &slug);
    } else {
        return None;
    }
}

pub async fn get_current_pr(url: String) -> Result<String> {
    let body = reqwest::get(url.as_str())
        .await?
        .text()
        .await?;

    println!("body: {:?}", body);
    println!("body");

    return Ok(body);
}

