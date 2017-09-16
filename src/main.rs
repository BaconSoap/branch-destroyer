extern crate branch_destroyer;
extern crate clap;

use branch_destroyer::*;
use clap::{App, Arg};

fn main() {
    let matches = get_args();

    let days_ago = match matches.value_of("days").unwrap().parse::<u32>() {
        Ok(x) => x,
        Err(e) => {
            println!("unable to parse --days because {}", e);
            panic!(e);
        }
    };

    let mut ctx = Context {
        token: matches.value_of("token").unwrap().to_string(),
        owner: matches.value_of("owner").unwrap().to_string(),
        repo: matches.value_of("repo").unwrap().to_string(),
        repo_id: 0,
        default_branch: "Develop".to_string(),
        days_ago
    };

    println!("Context: {:?}", ctx);

    get_repository(&mut ctx);
    let branches = get_branches(&ctx);

    let branches_info: Vec<BranchInfo> = branches.into_iter().map(get_full_branch_info).collect();

    print_branch_info(&branches_info);
    print_branch_info(&branches_info);
}

fn get_args<'a>() -> clap::ArgMatches<'a> {
    App::new("branch-destroyer")
        .version("1.0")
        .author("Andrew Varnerin <andrew@varnerin.info>")
        .about(
            "Hunts down and destroys useless branches with little remorse",
        )
        .arg(
            Arg::with_name("token")
                .short("t")
                .long("token")
                .value_name("TOKEN")
                .takes_value(true)
                .required(true)
                .help(
                    "Personal OAuth token with permissions to read/write repo info",
                ),
        )
        .arg(
            Arg::with_name("owner")
            .short("o")
            .long("owner")
            .value_name("OWNER")
            .takes_value(true)
            .required(true)
            .help(
                "Owner of the repository to run the branch destroyer against"
            )
        )
        .arg(
            Arg::with_name("repo")
            .short("r")
            .long("repo")
            .value_name("REPOSITORY")
            .takes_value(true)
            .required(true)
            .help(
                "Repository to run the branch destroyer against. You must have read/write permissions to this repository"
            )
        )
        .arg(
            Arg::with_name("days")
            .short("d")
            .long("days")
            .value_name("DAYS")
            .takes_value(true)
            .default_value("7")
            .help(
                "Filters to branches whose last commit was created at least this many days ago"
            )
        )
        .get_matches()
}
