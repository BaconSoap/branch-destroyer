// get repository & default branch
// get list of branches
//

#[derive(Debug)]
struct Context {
    token: String,
    owner: String,
    repo: String,
    repo_id: u32,
    default_branch: String,
}

#[derive(Debug)]
struct Branch {
    name: String,
}

#[derive(Debug)]
struct BranchInfo {
    branch: Branch,
    ahead: u32,
    behind: u32,
}

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

fn get_full_branch_info(branch: Branch) -> BranchInfo {
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

fn print_branch_info(branches_info: &Vec<BranchInfo>) {
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

fn get_repository(ctx: &mut Context) {
    ctx.repo_id = ctx.repo_id + 1;
}

fn get_branches(ctx: &Context) -> Vec<Branch> {
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

fn build_url(days_old: u32) {}

fn delete_branch() {}

fn format_branch_info() {}

fn get_comparison(head: String, base: String, context: Context) {}
