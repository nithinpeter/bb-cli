mod bitbucket;

use clap::{App, ArgGroup, SubCommand};
use bitbucket::pr::*;

fn main() {
    let matches = App::new("bb-cli")
        .subcommand(
            App::new("pr")
                .about("Pull requests commands")
                .subcommands(
                    vec![
                        SubCommand::with_name("open").about("Opens the pull request in the browser"),
                        SubCommand::with_name("status").about("Displays the status of the pull request"),
                        SubCommand::with_name("create").about("Opens create pull request screen in the browser"),
                    ]
                )
        )
        .get_matches();


    if let Some(ref prMatches) = matches.subcommand_matches("pr") {
        match prMatches.subcommand_name() {
            Some("open") => open_pr(),
            Some("status") => status_pr(),
            Some("create") => create_pr(),
            _ => {}
        };
    }
}


