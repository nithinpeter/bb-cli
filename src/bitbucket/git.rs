use std::process::Command;

pub fn git_current_branch() -> String {
    let result = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("git command failed to start");

    let branch = String::from_utf8_lossy(&result.stdout);
    return branch.to_string();
}

pub fn git_remote() -> String {
    let result = Command::new("git")
        .args(&["config", "--get", "remote.origin.url"])
        .output()
        .expect("git command failed to start");

    let remote = String::from_utf8_lossy(&result.stdout);
    return remote.to_string();
}

pub fn git_commit_sha() -> String {
    let result = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("git command failed to start");

    let branch = String::from_utf8_lossy(&result.stdout);
    return branch.to_string().trim().to_owned();
}