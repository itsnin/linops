pub fn items(_state: &super::state::State) -> Vec<crate::core::search::SearchItem> {
    vec![crate::core::search::SearchItem {
        task_id: "kdump_removal".to_string(),
        label: "remove kdump-tools".to_string(),
        description: "free 512mb of reserved kernel memory".to_string(),
    }]
}
