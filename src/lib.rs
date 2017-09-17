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
    let url = format!(
        "https://api.github.com/repos/{}/{}/branches",
        ctx.owner,
        ctx.repo
    );

    let client = get_client();
    let mut res = get_request(&client, &ctx.token, &url).send().unwrap();

    let mut content = String::new();
    res.read_to_string(&mut content).unwrap();

    let link_value = get_link_value(&res.headers, RelationType::Next);
    let link_value2 = get_link_value(&res.headers, RelationType::Alternate);

    println!("{:?}", link_value);
    println!("{:?}", link_value2);
    let data: Vec<Branch> = serde_json::from_str(&content).unwrap();

    data
}

/// Extract link={rel_type} values from the header collection
///
/// Returns Err if there's no link header or no link header whose rel={rel_type}
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
