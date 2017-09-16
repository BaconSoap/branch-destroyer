extern crate branch_destroyer;

use branch_destroyer::*;

fn main() {
    let mut ctx = Context {
        token: "Hi".to_string(),
        owner: "BaconSoap".to_string(),
        repo: "branch-destroyer".to_string(),
        repo_id: 0,
        default_branch: "Develop".to_string(),
    };

    get_repository(&mut ctx);
    let branches = get_branches(&ctx);

    let branches_info: Vec<BranchInfo> = branches.into_iter().map(get_full_branch_info).collect();

    print_branch_info(&branches_info);
    print_branch_info(&branches_info);
}
