pub fn items(_state: &super::state::State) -> Vec<crate::core::search::SearchItem> {
    vec![crate::core::search::SearchItem {
        task_id: "apt_pins".to_string(),
        label: "write apt pins".to_string(),
        description: "block removed packages from reinstalling".to_string(),
    }]
}
