#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate hyper_native_tls;
extern crate serde_json;

pub mod types;
pub use types::*;

use std::io::Read;
use hyper::Client;
use hyper::client::RequestBuilder;

use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use hyper::header::{Authorization, UserAgent};

pub fn get_full_branch_info(branch: Branch) -> BranchInfo {
    let branch_name = branch.name.clone();
    BranchInfo {
        branch,
        ahead: if branch_name == "feature/do-a-bunch-of-junk-and-stuff" {
            0
        } else {
            1
        },
        behind: 2,
    }
}

pub fn print_branch_info(branches_info: &Vec<BranchInfo>) {
    println!(
        "{0:<60} | {1:<10} | {2:<10} | {3:<10}",
        "Branch",
        "Ahead",
        "Behind",
        "Will Delete"
    );

    for branch in branches_info {
        println!(
            "{0:<60} | {1:<10} | {2:<10} | {3:<10}",
            branch.branch.name,
            branch.ahead,
            branch.behind,
            will_delete(branch)
        );
    }
}

fn will_delete(branch: &BranchInfo) -> bool {
    branch.ahead == 0
}

pub fn get_request<'a>(client: &'a Client, token: &'a str, url: &'a str) -> RequestBuilder<'a> {
    let auth = format!("token {}", token.clone());

    let req = client
        .get(url)
        .header(UserAgent("branch-destroyer 1.0".to_string()))
        .header(Authorization(auth));

    req
}

pub fn get_client() -> Client {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);

    Client::with_connector(connector)
}

pub fn get_repository(ctx: &mut Context) {
    let url = format!("https://api.github.com/repos/{}/{}", ctx.owner, ctx.repo);

    let client = get_client();
    let mut res = get_request(&client, &ctx.token, &url).send().unwrap();

    let mut content = String::new();
    res.read_to_string(&mut content).unwrap();

    let repo: Repository = serde_json::from_str(&content).unwrap();

    println!("{}", res.status);
    println!("{:?}", repo);
    ctx.repo_id = ctx.repo_id + 1;
}


pub fn get_branches(ctx: &Context) -> Vec<Branch> {
    vec![
        Branch {
            name: "master".to_string(),
        },
        Branch {
            name: "develop".to_string(),
        },
        Branch {
            name: "feature/do-a-bunch-of-junk-and-stuff".to_string(),
        },
    ]
}


/*

fn build_url(days_old: u32) {}

fn delete_branch() {}

fn format_branch_info() {}

fn get_comparison(head: String, base: String, context: Context) {}

*/
