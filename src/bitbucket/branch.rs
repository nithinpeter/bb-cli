extern crate opener;

use anyhow::{Context, Result};
use url::Url;

pub fn view_branch() -> Result<()> {
    let branch = super::git::git_current_branch();
    let remote_url = super::git::git_remote();

    let parsed = Url::parse(&remote_url)?;
    let slug = parsed.path().split(".git").collect::<Vec<_>>().join("");

    let url = format!(
        "{}://{}{}/branch/{}",
        parsed.scheme().to_owned(),
        parsed.host_str().unwrap(),
        &slug,
        branch
    );

    opener::open(url).context("Failed to open view branch url")
}
