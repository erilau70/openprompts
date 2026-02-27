use crate::models::prompt::PromptMetadata;

/// Search prompts. Empty query returns recency-sorted. Non-empty does fuzzy matching.
pub fn search_prompts(prompts: &[PromptMetadata], query: &str) -> Vec<PromptMetadata> {
    if query.trim().is_empty() {
        return sort_by_recency(prompts);
    }

    let query_lower = query.to_lowercase();
    let mut scored: Vec<(f64, &PromptMetadata)> = prompts
        .iter()
        .filter_map(|p| {
            let score = compute_score(p, &query_lower);
            if score > 0.0 {
                Some((score, p))
            } else {
                None
            }
        })
        .collect();

    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    scored.into_iter().map(|(_, p)| p.clone()).collect()
}

fn sort_by_recency(prompts: &[PromptMetadata]) -> Vec<PromptMetadata> {
    let mut sorted: Vec<PromptMetadata> = prompts.to_vec();
    sorted.sort_by(|a, b| {
        // Sort by last_used desc (Some > None), then by updated desc
        match (&b.last_used, &a.last_used) {
            (Some(b_used), Some(a_used)) => b_used.cmp(a_used),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => b.updated.cmp(&a.updated),
        }
    });
    sorted
}

fn compute_score(prompt: &PromptMetadata, query: &str) -> f64 {
    let name_score = fuzzy_score(&prompt.name.to_lowercase(), query) * 3.0;
    let desc_score = fuzzy_score(&prompt.description.to_lowercase(), query) * 2.0;
    let folder_score = fuzzy_score(&prompt.folder.to_lowercase(), query) * 1.0;

    let max = name_score.max(desc_score).max(folder_score);
    if max <= 0.0 {
        return 0.0;
    }

    // Bonus for starts-with match on name
    let starts_bonus = if prompt.name.to_lowercase().starts_with(query) {
        10.0
    } else {
        0.0
    };

    max + starts_bonus
}

/// Simple subsequence fuzzy scoring. Returns 0.0 if no match.
/// Higher score = better match (consecutive matches and position matter).
fn fuzzy_score(text: &str, query: &str) -> f64 {
    let text_chars: Vec<char> = text.chars().collect();
    let query_chars: Vec<char> = query.chars().collect();

    if query_chars.is_empty() {
        return 1.0;
    }
    if query_chars.len() > text_chars.len() {
        return 0.0;
    }

    let mut score = 0.0;
    let mut qi = 0;
    let mut prev_match_idx: Option<usize> = None;

    for (ti, &tc) in text_chars.iter().enumerate() {
        if qi < query_chars.len() && tc == query_chars[qi] {
            score += 1.0;

            // Consecutive match bonus
            if let Some(prev) = prev_match_idx {
                if ti == prev + 1 {
                    score += 2.0;
                }
            }

            // Position bonus (earlier matches score higher)
            score += 1.0 / (ti as f64 + 1.0);

            prev_match_idx = Some(ti);
            qi += 1;
        }
    }

    if qi == query_chars.len() {
        score
    } else {
        0.0 // Not all query chars matched
    }
}
