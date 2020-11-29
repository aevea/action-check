use github_rs::{
    client::{Executor, Github},
    errors::Error,
};
use serde_json::Value;

/// get_pr_files gets the files from a given PR from a given organization repo
pub fn get_pr_files(
    client: Github,
    owner: &str,
    repo: &str,
    pr_id: &str,
) -> Result<Option<Value>, Error> {
    let pr_data = client
        .get()
        .repos()
        .owner(owner)
        .repo(repo)
        .pulls()
        .number(pr_id)
        .files()
        .execute::<Value>();

    match pr_data {
        Ok((_headers, _status, json)) => Ok(json),
        Err(e) => Err(e),
    }
}
