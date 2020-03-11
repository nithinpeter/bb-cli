mod bitbucket;

use clap::{App, ArgGroup};
use bitbucket::pr::*;

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
                    .args(&["open", "status", "create"])
                    .required(true))
        )
        .get_matches();

    // Pull request subcommand
    if let Some(ref matches) = matches.subcommand_matches("pr") {
        if matches.is_present("open") {
            open_pr();
        } else if matches.is_present("status") {
            status_pr();
        } else if matches.is_present("create") {
            create_pr();
        }
    }
}


