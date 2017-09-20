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
        days_ago,
        for_real: matches.is_present("for-real")
    };

    get_repository(&mut ctx);

    let branches = get_branches(&ctx);
    let total_branches = branches.len();

    let branches_info: Vec<BranchInfo> = branches
        .into_iter()
        .filter(|x| x.name != "master")
        .filter(|x| x.name != "develop")        
        //.take(20)
        .map(|x| get_branch_compare_info(&ctx, x))
        .collect();

    print_branch_info(&branches_info, &ctx);

    let branches_to_delete = branches_info.into_iter().filter(|x| will_delete(x, &ctx)).collect();

    print_summary(&ctx, &branches_to_delete, total_branches);

    print_branch_info(&branches_to_delete, &ctx);
    
    if ctx.for_real {
        for branch in branches_to_delete {
            let branch_name = branch.branch.name.clone();

            match delete_branch(&ctx, branch) {
                true => println!("Deleted {}", branch_name),
                false => println!("Failed to delete {}", branch_name)
            }
            
            std::thread::sleep(std::time::Duration::from_millis(100));

        }
    }
}

fn print_summary(ctx: &Context, branches: &Vec<BranchInfo>, total_branches: usize) {
    println!();
    println!("Found {} branches to delete out of {} branches total for {}/{}. SHAME SHAME SHAME", branches.len(), total_branches, ctx.owner, ctx.repo )
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
        .arg(
            Arg::with_name("for-real")
            .long("for-real")
            .help(
                "When set will really (for real!) delete branches. Be sure you want to do this."
            )
        )
        .get_matches()
}
