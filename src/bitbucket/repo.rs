extern crate opener;

use url::Url;

pub fn view_repo() {
    let remote_url = super::git::git_remote();

    if let Ok(parsed) = Url::parse(&remote_url) {
        let slug = parsed.path().split(".git").collect::<Vec<_>>().join("");

        let url = format!("{}://{}{}",
                          parsed.scheme().to_owned(),
                          parsed.host_str().unwrap(),
                          &slug);

        opener::open(url);
    }
}