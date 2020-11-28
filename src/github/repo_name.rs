pub struct Repo {
    pub owner: String,
    pub name: String,
}

/// parse_repo_name converts a repo name from the Github format into a Repo struct
/// ```
/// let input = "owner/name";
/// let result = action_check::github::parse_repo_name(input.to_string());
/// assert_eq!(result.owner, "owner");
/// assert_eq!(result.name, "name");
/// ```
pub fn parse_repo_name(raw_string: String) -> Repo {
    let parsed_string: Vec<_> = raw_string.split('/').collect();

    Repo {
        owner: parsed_string[0].to_string(),
        name: parsed_string[1].to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_repo_name() {
        let result = parse_repo_name("owner/name".to_string());
        assert_eq!(result.owner, "owner");
        assert_eq!(result.name, "name");
    }
}
