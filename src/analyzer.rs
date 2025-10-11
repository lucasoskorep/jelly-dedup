use crate::models::{Episode, Movie};
use std::collections::HashMap;

/// Filters episodes to return only those with multiple media sources (duplicates)
pub fn filter_duplicate_episodes(episodes: Vec<Episode>) -> Vec<Episode> {
    episodes
        .into_iter()
        .filter(|ep| has_multiple_versions_episode(ep))
        .collect()
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
