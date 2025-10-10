use crate::models::Episode;

/// Filters episodes to return only those with multiple media sources (duplicates)
pub fn filter_duplicate_episodes(episodes: Vec<Episode>) -> Vec<Episode> {
    episodes
        .into_iter()
        .filter(|ep| has_multiple_versions(ep))
        .collect()
}

fn has_multiple_versions(episode: &Episode) -> bool {
    if let Some(media_sources) = &episode.media_sources {
        media_sources.len() > 1
    } else {
        false
    }
}
