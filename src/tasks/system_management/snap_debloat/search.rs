pub fn items(_state: &super::state::State) -> Vec<crate::core::search::SearchItem> {
    vec![
        crate::core::search::SearchItem {
            task_id: "snap_debloat".to_string(),
            label: "remove snapd".to_string(),
            description: "purge snapd and all snap packages".to_string(),
        },
        crate::core::search::SearchItem {
            task_id: "snap_debloat".to_string(),
            label: "pin snapd".to_string(),
            description: "block snapd from reinstalling".to_string(),
        },
    ]
}
