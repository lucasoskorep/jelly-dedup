use crate::models::{Episode, MediaSource, Movie};
use crate::selector;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FileToDelete {
    pub path: String,
    pub size: i64,
}

pub fn print_duplicate_episodes(show_name: &str, episodes: Vec<Episode>) -> Vec<FileToDelete> {
    println!("\n📺 Show: {}", show_name);
    println!("{}", "-".repeat(80));
    println!("   Episodes with multiple versions: {}\n", episodes.len());

    let mut files_to_delete = Vec::new();

    for episode in episodes {
        let to_delete = print_episode_with_versions(episode);
        files_to_delete.extend(to_delete);
    }

    println!("{}", "=".repeat(80));

    files_to_delete
}

fn print_episode_with_versions(episode: Episode) -> Vec<FileToDelete> {
    let season = episode.season_number.unwrap_or(0);
    let ep_num = episode.episode_number.unwrap_or(0);

    let version_count = episode
        .media_sources
        .as_ref()
        .map(|ms| ms.len())
        .unwrap_or(0);

    println!(
        "   S{:02}E{:02} - {} ({} versions)",
        season, ep_num, episode.name, version_count
    );

    let mut files_to_delete = Vec::new();

    if let Some(media_sources) = episode.media_sources {
        // Select the best source
        if let Some(best_idx) = selector::select_best_source(&media_sources) {
            // Print selected file
            println!("      [SELECTED]");
            print_media_source(&media_sources[best_idx]);

            // Print non-selected files
            if media_sources.len() > 1 {
                println!("      [TO DELETE]");
                for (idx, source) in media_sources.iter().enumerate() {
                    if idx != best_idx {
                        print_media_source(source);
                        if let Some(path) = &source.path {
                            let size = source.size.unwrap_or(0);
                            files_to_delete.push(FileToDelete {
                                path: path.clone(),
                                size,
                            });
                        }
                    }
                }
            }
        }
    }

    println!();

    files_to_delete
}

fn print_media_source(source: &MediaSource) {
    let bitrate_str = format_bitrate(source.bitrate);
    let resolution_str = format_resolution(source);
    let size_str = format_size(source.size);
    let codec_str = format_codec(source);
    let container_str = format_container(&source.container);
    let path_str = format_path(&source.path);

    println!(
        "         {} | {} | {} | {} | {} | {}",
        bitrate_str, resolution_str, size_str, codec_str, container_str, path_str
    );
}

fn format_bitrate(bitrate: Option<i64>) -> String {
    bitrate
        .map(|b| format!("{:.2} Mbps", b as f64 / 1_000_000.0))
        .unwrap_or_else(|| "Unknown".to_string())
}

fn format_resolution(source: &MediaSource) -> String {
    source
        .height
        .map(|h| format!("{}p", h))
        .or_else(|| {
            // Look for resolution in MediaStreams (video stream)
            source.media_streams.as_ref().and_then(|streams| {
                streams
                    .iter()
                    .find(|s| s.stream_type.as_deref() == Some("Video"))
                    .and_then(|s| s.height.map(|h| format!("{}p", h)))
            })
        })
        .unwrap_or_else(|| "Unknown".to_string())
}

fn format_container(container: &Option<String>) -> &str {
    container.as_ref().map(|s| s.as_str()).unwrap_or("Unknown")
}

fn format_path(path: &Option<String>) -> &str {
    path.as_ref().map(|s| s.as_str()).unwrap_or("Unknown")
}

fn format_size(size: Option<i64>) -> String {
    size.map(|s| format!("{:.2} GB", s as f64 / 1_073_741_824.0))
        .unwrap_or_else(|| "Unknown".to_string())
}

fn format_codec(source: &MediaSource) -> String {
    // Look for video codec in MediaStreams
    source
        .media_streams
        .as_ref()
        .and_then(|streams| {
            streams
                .iter()
                .find(|s| s.stream_type.as_deref() == Some("Video"))
                .and_then(|s| s.codec.as_ref().map(|c| c.to_string()))
        })
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn print_duplicate_movies(movie_groups: Vec<Vec<Movie>>) -> Vec<FileToDelete> {
    let mut files_to_delete = Vec::new();

    for movie_group in movie_groups {
        if movie_group.is_empty() {
            continue;
        }

        // If there's only one movie in the group, it must have multiple media sources
        if movie_group.len() == 1 {
            let movie = &movie_group[0];
            println!("\n🎬 Movie: {}", format_movie_title(movie));
            println!("{}", "-".repeat(80));

            if let Some(media_sources) = &movie.media_sources {
                println!("   Multiple versions found: {}\n", media_sources.len());
                let to_delete = print_movie_versions(&movie.name, media_sources);
                files_to_delete.extend(to_delete);
            }
        } else {
            // Multiple movies with same name/year - treat each as a separate version
            let first_movie = &movie_group[0];
            println!("\n🎬 Movie: {}", format_movie_title(first_movie));
            println!("{}", "-".repeat(80));
            println!("   Multiple copies found: {}\n", movie_group.len());

            // Collect all media sources from all movies
            let mut all_sources: Vec<MediaSource> = Vec::new();
            for movie in &movie_group {
                if let Some(media_sources) = &movie.media_sources {
                    for source in media_sources {
                        all_sources.push(source.clone());
                    }
                }
            }

            if !all_sources.is_empty() {
                let to_delete = print_movie_versions(&first_movie.name, &all_sources);
                files_to_delete.extend(to_delete);
            }
        }

        println!("{}", "=".repeat(80));
    }

    files_to_delete
}

fn format_movie_title(movie: &Movie) -> String {
    if let Some(year) = movie.year {
        format!("{} ({})", movie.name, year)
    } else {
        movie.name.clone()
    }
}

fn print_movie_versions(_movie_name: &str, media_sources: &Vec<MediaSource>) -> Vec<FileToDelete> {
    let mut files_to_delete = Vec::new();

    if let Some(best_idx) = selector::select_best_source(media_sources) {
        // Print selected file
        println!("      [SELECTED]");
        print_media_source(&media_sources[best_idx]);

        // Print non-selected files
        if media_sources.len() > 1 {
            println!("      [TO DELETE]");
            for (idx, source) in media_sources.iter().enumerate() {
                if idx != best_idx {
                    print_media_source(source);
                    if let Some(path) = &source.path {
                        let size = source.size.unwrap_or(0);
                        files_to_delete.push(FileToDelete {
                            path: path.clone(),
                            size,
                        });
                    }
                }
            }
        }
    }

    println!();

    files_to_delete
}
