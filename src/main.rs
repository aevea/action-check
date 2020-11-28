mod github;
use github::{extract_pr_id, parse_repo_name};
use github_rs::client::{Executor, Github};
use serde_json::Value;
use std::env;

fn main() {
    let token = env::var("GITHUB_TOKEN").unwrap();
    let repo_string = env::var("GITHUB_REPO").unwrap();

    let reference = env::var("GITHUB_REF").unwrap();

    let repo_info = parse_repo_name(repo_string);

    let client = Github::new(token).unwrap();

    let pr_data = client
        .get()
        .repos()
        .owner(&*repo_info.owner)
        .repo(&*repo_info.name)
        .pulls()
        .number(&*extract_pr_id(reference))
        .files()
        .execute::<Value>();
    match pr_data {
        Ok((headers, status, json)) => {
            println!("{:#?}", headers);
            println!("{}", status);
            if let Some(json) = json {
                println!("{}", json);
            }
        }
        Err(e) => println!("{}", e),
    }
}
