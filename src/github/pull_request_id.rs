/// extract_pr_id splits the expected ref format until only the number id remains
pub fn extract_pr_id(raw_string: String) -> String {
    raw_string
        .trim_start_matches("refs/pull/")
        .trim_end_matches("/merge")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_pr_id() {
        let result = extract_pr_id("refs/pull/123/merge".to_string());
        assert_eq!(result, "123");
    }
}
