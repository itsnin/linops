// system_update search items
pub fn items(_state: &super::state::State) -> Vec<crate::core::search::SearchItem> {
    vec![
        crate::core::search::SearchItem {
            task_id: "system_update".to_string(),
            label: "apt update".to_string(),
            description: "fetch latest package lists".to_string(),
        },
        crate::core::search::SearchItem {
            task_id: "system_update".to_string(),
            label: "apt upgrade".to_string(),
            description: "upgrade all installed packages".to_string(),
        },
    ]
}
