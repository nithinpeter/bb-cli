extern crate opener;

use url::Url;

pub fn view_commit() {
    let remote_url = super::git::git_remote();
    let commit_sha = super::git::git_commit_sha();

    if let Ok(parsed) = Url::parse(&remote_url) {
        let slug = parsed.path().split(".git").collect::<Vec<_>>().join("");

        let url = format!("{}://{}{}/commits/{}",
                          parsed.scheme().to_owned(),
                          parsed.host_str().unwrap(),
                          &slug, &commit_sha);

        opener::open(url);
    }
}