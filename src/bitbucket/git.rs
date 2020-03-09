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

//    let remote = String::from_utf8_lossy(&result.stdout);
//    return remote.to_string();

    let remote = "https://bitbucket.org/nithinpeterk/react-fork.git";

    return remote.to_string();
}