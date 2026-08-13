use crate::models::{Episode, Movie};
use std::collections::HashMap;

/// Groups episodes covering the same season/episode number, returning only the groups
/// that hold more than one file. Duplicates show up two different ways depending on how
/// Jellyfin matched the files: merged into one item with several media sources, or as
/// separate items sharing a season/episode number.
pub fn filter_duplicate_episodes(episodes: Vec<Episode>) -> Vec<Vec<Episode>> {
    let mut episode_map: HashMap<(u32, u32), Vec<Episode>> = HashMap::new();
    let mut unnumbered: Vec<Vec<Episode>> = Vec::new();

    for episode in episodes {
        match (episode.season_number, episode.episode_number) {
            (Some(season), Some(number)) => {
                episode_map.entry((season, number)).or_default().push(episode);
            }
            // Without a season/episode number there is nothing safe to match on, so these
            // are kept apart and only checked for multiple media sources. Grouping them
            // together would coalesce unrelated specials into one bogus duplicate set.
            _ => unnumbered.push(vec![episode]),
        }
    }

    let mut groups: Vec<Vec<Episode>> = episode_map
        .into_values()
        .chain(unnumbered)
        .filter(|group| group.len() > 1 || has_multiple_versions_episode(&group[0]))
        .collect();

    // HashMap iteration order is arbitrary; sort so output is stable between runs
    groups.sort_by_key(|group| {
        (
            group[0].season_number.unwrap_or(u32::MAX),
            group[0].episode_number.unwrap_or(u32::MAX),
        )
    });

    groups
}

fn has_multiple_versions_episode(episode: &Episode) -> bool {
    if let Some(media_sources) = &episode.media_sources {
        media_sources.len() > 1
    } else {
        false
    }
}

/// Filters movies to return only those with duplicate titles (same name and year)
pub fn filter_duplicate_movies(movies: Vec<Movie>) -> Vec<Vec<Movie>> {
    let mut movie_map: HashMap<String, Vec<Movie>> = HashMap::new();

    // Group movies by title and year
    for movie in movies {
        let key = format!("{}-{}", movie.name, movie.year.unwrap_or(0));
        movie_map.entry(key).or_insert_with(Vec::new).push(movie);
    }

    // Return only groups with multiple movies or movies with multiple media sources
    movie_map
        .into_values()
        .filter(|group| group.len() > 1 || (group.len() == 1 && has_multiple_versions_movie(&group[0])))
        .collect()
}

fn has_multiple_versions_movie(movie: &Movie) -> bool {
    if let Some(media_sources) = &movie.media_sources {
        media_sources.len() > 1
    } else {
        false
    }
}
