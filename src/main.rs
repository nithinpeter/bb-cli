mod bitbucket;

use clap::{App, SubCommand};
use bitbucket::*;

fn main() {
    let matches = App::new("bb-cli")
        .subcommand(
            App::new("pr")
                .about("Pull request commands")
                .subcommands(
                    vec![
                        SubCommand::with_name("view").about("Opens the pull request in the browser"),
                        SubCommand::with_name("status").about("Displays the status of the pull request"),
                        SubCommand::with_name("create").about("Opens create pull request screen in the browser"),
                        SubCommand::with_name("list").about("List pull requests authored by you"),
                    ]
                )
        )
        .subcommand(
            App::new("branch")
                .about("Branch commands")
                .subcommands(
                    vec![
                        SubCommand::with_name("view").about("Opens the branch in the browser"),
                    ]
                )
        )
        .subcommand(
            App::new("commit")
                .about("Commit commands")
                .subcommands(
                    vec![
                        SubCommand::with_name("view").about("Opens the commit in the browser"),
                    ]
                )
        )
        .subcommand(
            App::new("repo")
                .about("Repo commands")
                .subcommands(
                    vec![
                        SubCommand::with_name("view").about("Opens the repo in the browser"),
                    ]
                )
        )
        .get_matches();


    if let Some(ref prMatches) = matches.subcommand_matches("pr") {
        match prMatches.subcommand_name() {
            Some("view") => pr::view_pr(),
            Some("status") => pr::status_pr(),
            Some("create") => pr::create_pr(),
            Some("list") => pr::list_pr(),
            _ => {}
        };
    } else if let Some(ref prMatches) = matches.subcommand_matches("branch") {
        match prMatches.subcommand_name() {
            Some("view") => branch::view_branch(),
            _ => {}
        };
    }
}

