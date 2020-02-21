extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("bb-cli")
        .about("A CLI app for Bitbucket")
        .version("0.1.0")
        .author("npeter@atlassian.com")
        .subcommand(
            App::new("pr").about("pull requests").arg(
                Arg::with_name("open")
                    .long("open")
                    .short("o")
                    .help("Open the pull request created from the current branch")
                    .required(true),
            ),
        )
        .get_matches();


    match matches.subcommand() {
        ("pr", Some(pr_matches)) => {
            // Now we have a reference to clone's matches
            println!("Cloning {}", pr_matches.value_of("open").unwrap());
        }
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    }
}