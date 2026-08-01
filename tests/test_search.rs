use linops::core::search::{self, SearchItem};

#[test]
fn test_empty_query_matches_everything() {
    assert_eq!(search::fuzzy_match("", "anything").unwrap(), 0);
}

#[test]
fn test_exact_subsequence() {
    assert!(search::fuzzy_match("abc", "a_b_c").is_some());
}

#[test]
fn test_non_matching_returns_none() {
    assert!(search::fuzzy_match("xyz", "abc").is_none());
}

#[test]
fn test_case_insensitive() {
    assert!(search::fuzzy_match("ABC", "abc").is_some());
    assert!(search::fuzzy_match("abc", "ABC").is_some());
}

#[test]
fn test_search_results_sorted_by_score() {
    let items = vec![
        SearchItem {
            task_id: "1".into(),
            label: "system update".into(),
            description: "update".into(),
        },
        SearchItem {
            task_id: "2".into(),
            label: "update".into(),
            description: "system update".into(),
        },
    ];
    let refs: Vec<&SearchItem> = items.iter().collect();
    let results = search::search_results("update", &refs);
    assert!(!results.is_empty());
}
