// dev_toolchain search items
pub fn items(_state: &super::state::State) -> Vec<crate::core::search::SearchItem> {
    super::state::State::names()
        .iter()
        .map(|(name, desc, _)| crate::core::search::SearchItem {
            task_id: "dev_toolchain".to_string(),
            label: name.to_string(),
            description: desc.to_string(),
        })
        .collect()
}
