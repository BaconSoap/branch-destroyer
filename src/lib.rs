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

use hyper::header::{Authorization, Link, LinkValue, UserAgent};
use hyper::header::RelationType;

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
        let branch_name: String = branch.branch.name.chars().take(60).collect();

        println!(
            "{0:<60} | {1:<10} | {2:<10} | {3:<10}",
            branch_name,
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
    let first_url = format!(
        "https://api.github.com/repos/{}/{}/branches?per_page=100",
        ctx.owner,
        ctx.repo
    );

    let mut next_url = Some(first_url);
    let mut all_branches: Vec<Branch> = vec![];

    let mut i = 0;
    while let Some(url) = next_url {
        let results = get_branches_and_next(url, &ctx);
        let mut results_branches = results.1;
        all_branches.append(&mut results_branches);
        next_url = results.0;

        i = i + 1;
        if i > 100 {
            panic!("way too many branch iterations!");
        }
    }

    println!(
        "found {} branches, taking {} iterations",
        all_branches.len(),
        i
    );

    all_branches
}

fn get_branches_and_next(url: String, ctx: &Context) -> (Option<String>, Vec<Branch>) {
    let client = get_client();
    let mut res = get_request(&client, &ctx.token, &url).send().unwrap();

    let mut content = String::new();
    res.read_to_string(&mut content).unwrap();

    let link_value = get_link_value(&res.headers, RelationType::Next);

    let data: Vec<Branch> = serde_json::from_str(&content).unwrap();

    (
        link_value.ok().and_then(|x| Some(x.link().to_string())),
        data,
    )
}

/// Extract link={rel_type} values from the header collection
///
/// Returns GetLinkErr if there's no link header or no link header whose rel={rel_type}
fn get_link_value<'a>(
    headers: &hyper::header::Headers,
    rel_type: RelationType,
) -> Result<LinkValue, GetLinkErr> {
    let link = match headers.get::<Link>() {
        Some(x) => Ok(x),
        None => Err(GetLinkErr::NoLinkHeader),
    };

    let next: Result<LinkValue, GetLinkErr> = link.and_then(|x| {
        let a = x.values()
            .into_iter()
            .filter(|x| match x.rel() {
                Some(x) => match x[0] {
                    ref r if (r == &rel_type) => true,
                    _ => false,
                },
                _ => false,
            })
            .next();

        match a {
            Some(l) => Ok(l.clone()),
            None => Err(GetLinkErr::NoMatchingRel(rel_type)),
        }
    });

    next
}

#[derive(Debug)]
pub enum GetLinkErr {
    NoLinkHeader,
    NoMatchingRel(RelationType),
}

#[cfg(test)]
mod tests {

    use super::hyper::header::{Headers, Link, LinkValue, RelationType};
    use super::{get_link_value, GetLinkErr};

    #[test]
    pub fn get_link_value_works() {
        let next_link = LinkValue::new("https://google.com").push_rel(RelationType::Next);
        let prev_link = LinkValue::new("https://reddit.com").push_rel(RelationType::Prev);

        let mut headers = Headers::new();
        headers.set(Link::new(vec![next_link, prev_link]));

        let next_res = get_link_value(&headers, RelationType::Next);
        let prev_res = get_link_value(&headers, RelationType::Prev);
        let alt_res = get_link_value(&headers, RelationType::Alternate);

        assert!(
            next_res.is_ok(),
            "we should be able to fetch rel=next because it is in the collection"
        );

        assert!(
            prev_res.is_ok(),
            "we should be able to fetch rel=prev because it is in the collection"
        );

        match alt_res {
            Ok(_) => assert!(false, "we should not be able to fetch a missing rel"),
            Err(GetLinkErr::NoLinkHeader) => assert!(
                false,
                "we should not see NoLinkHeader when the collection has a link"
            ),
            Err(GetLinkErr::NoMatchingRel(_)) => assert!(true),
        }
    }

    #[test]
    pub fn get_link_value_returns_err_for_headers_without_link() {
        let headers = Headers::new();

        let res = get_link_value(&headers, RelationType::Next);

        match res {
            Ok(_) => assert!(
                false,
                "we should not be able to fetch any link header if there are no link headers"
            ),
            Err(GetLinkErr::NoMatchingRel(_)) => assert!(
                false,
                "we should not get this error if there are no link headers"
            ),
            Err(GetLinkErr::NoLinkHeader) => assert!(true),
        }
    }
}


/*

fn build_url(days_old: u32) {}

fn delete_branch() {}

fn format_branch_info() {}

fn get_comparison(head: String, base: String, context: Context) {}

*/
