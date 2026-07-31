pub fn items(_state: &super::state::State) -> Vec<crate::core::search::SearchItem> {
    vec![
        crate::core::search::SearchItem {
            task_id: "gnome_debloat".to_string(),
            label: "remove gnome apps".to_string(),
            description: "remove 23 gnome utility applications".to_string(),
        },
        crate::core::search::SearchItem {
            task_id: "gnome_debloat".to_string(),
            label: "remove ptyxis".to_string(),
            description: "remove ubuntus custom terminal".to_string(),
        },
    ]
}
