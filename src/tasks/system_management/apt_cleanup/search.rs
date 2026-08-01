pub fn items(_state: &super::state::State) -> Vec<crate::core::search::SearchItem> {
    vec![
        crate::core::search::SearchItem {
            task_id: "apt_cleanup".to_string(),
            label: "mark manual".to_string(),
            description: "mark critical packages as manually installed".to_string(),
        },
        crate::core::search::SearchItem {
            task_id: "apt_cleanup".to_string(),
            label: "drop gnome-core".to_string(),
            description: "remove the gnome-core metapackage".to_string(),
        },
        crate::core::search::SearchItem {
            task_id: "apt_cleanup".to_string(),
            label: "autoremove purge".to_string(),
            description: "remove orphaned packages".to_string(),
        },
    ]
}
