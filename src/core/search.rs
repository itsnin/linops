// fuzzy search engine
// uses subsequence matching not substring matching
// query chars must appear in target in order but not necessarily contiguous
//
// scoring factors higher is better
// contiguous matches score higher than scattered
// matches at word boundaries score higher than mid word
// earlier matches score higher than later
// shorter targets with the same match score higher density bonus
//
// case insensitive by default
// the query is lowercased before matching
// the target is lowercased during matching without modifying the original

pub struct SearchItem {
    pub task_id: String,
    pub label: String,
    pub description: String,
}

// returns some score if query matches target as a subsequence
// returns none if query is not a subsequence of target
// empty query matches everything with score 0
pub fn fuzzy_match(query: &str, target: &str) -> Option<u32> {
    if query.is_empty() {
        return Some(0);
    }

    let query: Vec<char> = query.to_lowercase().chars().collect();
    let target: Vec<char> = target.to_lowercase().chars().collect();

    if query.len() > target.len() {
        return None;
    }

    let mut score: u32 = 0;
    let mut qi = 0;
    let mut prev_match_pos: Option<usize> = None;

    for (ti, &tch) in target.iter().enumerate() {
        if qi >= query.len() {
            break;
        }

        if tch == query[qi] {
            // base score for a match
            score += 10;

            // bonus for contiguous matches
            if let Some(prev) = prev_match_pos {
                if ti == prev + 1 {
                    score += 15;
                }
            }

            // bonus for word boundary matches
            // a word boundary is start of string or after a space hyphen underscore
            if ti == 0 || matches!(target.get(ti - 1), Some(' ') | Some('-') | Some('_')) {
                score += 20;
            }

            // earlier matches score higher
            score += (target.len() - ti) as u32;

            prev_match_pos = Some(ti);
            qi += 1;
        }
    }

    if qi == query.len() {
        // density bonus shorter targets with full match score higher
        score += (1000 / target.len() as u32).max(1);
        Some(score)
    } else {
        None
    }
}

// takes a query and a list of search item references
// returns items that match sorted by score descending
// ties broken alphabetically by label
pub fn search_results<'a>(query: &str, items: &[&'a SearchItem]) -> Vec<(&'a SearchItem, u32)> {
    let mut results: Vec<(&SearchItem, u32)> = items
        .iter()
        .filter_map(|item| {
            // match against both label and description
            let label_score = fuzzy_match(query, &item.label)?;
            let desc_score = fuzzy_match(query, &item.description).unwrap_or(0);
            let combined = label_score + desc_score / 2;
            Some((*item, combined))
        })
        .collect();

    // sort by score descending then alphabetically by label
    results.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.label.cmp(&b.0.label)));

    results
}
