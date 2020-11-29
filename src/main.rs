mod github;
use github::{extract_pr_id, get_pr_files, parse_repo_name};
use github_rs::client::Github;
use std::env;

fn main() {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is required");
    let repo_string =
        env::var("GITHUB_REPO").expect("GITHUB_REPO is required in format owner/name");

    let reference =
        env::var("GITHUB_REF").expect("GITHUB_REF is required in format refs/pull/123/merge");

    let repo_info = parse_repo_name(repo_string);

    let client = Github::new(token).unwrap();

    let pr_data = get_pr_files(
        client,
        &*repo_info.owner,
        &*repo_info.name,
        &*extract_pr_id(reference),
    );

    println!("{:?}", pr_data)
}
