use github_rs::{
    client::{Executor, Github},
    errors::Error,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct File {
    pub filename: String,
}

/// get_pr_files gets the files from a given PR from a given organization repo
pub fn get_pr_files(
    client: Github,
    owner: &str,
    repo: &str,
    pr_id: &str,
) -> Result<Option<Vec<File>>, Error> {
    let pr_data = client
        .get()
        .repos()
        .owner(owner)
        .repo(repo)
        .pulls()
        .number(pr_id)
        .files()
        .execute::<Vec<File>>();

    match pr_data {
        Ok((_headers, _status, json)) => Ok(json),
        Err(e) => Err(e),
    }
}
