use crate::models::{Episode, MediaSource};
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
