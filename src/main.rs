use clap::{App, ArgGroup};
use std::process::Command;

fn main() {
    let matches = App::new("bb-cli")
        .subcommand(
            App::new("pr")
                .about("Pull requests commands")
                .args_from_usage(
                    "--open             'opens the pull request in the browser'
                            --status           'displays the status of the pull request'
                            --create           'opens create pull request screen in the browser")
                .group(ArgGroup::with_name("pr-args")
                    .args(&["open", "status"])
                    .required(true))
        )
        .get_matches();

    // Pull request subcommand
    if let Some(ref matches) = matches.subcommand_matches("pr") {
        if matches.is_present("open") {
            // 1. find the current branch name
            let current_branch= git_current_branch();
            // 2. check if you have corresponding pull request

            // 3. if yes open the pull request
            println!("can you open that pr??");
        }
    }
}

fn git_current_branch() -> String {
    let result = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("git command failed to start");

    let branch = String::from_utf8_lossy(&result.stdout);
    return branch.to_string();
}

//fn get_pr_link(&current_branch: &str) {
//
//}