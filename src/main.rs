mod analyzer;
mod client;
mod display;
mod models;
mod selector;

use clap::Parser;
use client::JellyfinClient;
use display::FileToDelete;
use std::collections::HashSet;
use std::error::Error;

/// A tool to find and manage duplicate episodes in Jellyfin
#[derive(Parser, Debug)]
#[command(name = "jelly-dedup")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Jellyfin server URL
    #[arg(short, long, env("JELLYFIN_URL"), default_value = "http://localhost:8096")]
    jellyfin_url: String,

    /// Jellyfin API key
    #[arg(short, long, env("JELLYFIN_API_KEY"))]
    api_key: String,

    /// Path prefix to remove from displayed file paths
    #[arg(short, long, env("PATH_PREFIX_TO_REMOVE"))]
    path_prefix_to_remove: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let args = Args::parse();

    let config = Config {
        jellyfin_url: args.jellyfin_url,
        api_key: args.api_key,
        path_prefix_to_remove: args.path_prefix_to_remove,
    };

    let client = JellyfinClient::new(config.jellyfin_url, config.api_key);

    process_all_shows(&client, config.path_prefix_to_remove).await?;

    Ok(())
}

struct Config {
    jellyfin_url: String,
    api_key: String,
    path_prefix_to_remove: Option<String>,
}

struct Statistics {
    total_duplicate_episodes: usize,
    total_duplicate_files: usize,
    files_to_delete: HashSet<FileToDelete>,
}

async fn process_all_shows(client: &JellyfinClient, path_prefix_to_remove: Option<String>) -> Result<(), Box<dyn Error>> {
    println!("Fetching all TV shows from Jellyfin...\n");
    let shows = client.get_all_shows().await?;

    println!("Found {} TV shows\n", shows.len());
    println!("{}", "=".repeat(80));

    let mut stats = Statistics {
        total_duplicate_episodes: 0,
        total_duplicate_files: 0,
        files_to_delete: HashSet::new(),
    };

    for show in shows {
        match process_show(client, &show).await {
            Ok((episode_count, file_count, files_to_delete)) => {
                stats.total_duplicate_episodes += episode_count;
                stats.total_duplicate_files += file_count;
                stats.files_to_delete.extend(files_to_delete);
            }
            Err(e) => {
                eprintln!("   ❌ Error processing {}: {}", show.name, e);
            }
        }
    }

    print_summary(&stats, path_prefix_to_remove.as_deref());

    Ok(())
}

async fn process_show(
    client: &JellyfinClient,
    show: &models::Item,
) -> Result<(usize, usize, Vec<FileToDelete>), Box<dyn Error>> {
    let episodes = client.get_episodes_for_show(&show.id).await?;
    let duplicate_episodes = analyzer::filter_duplicate_episodes(episodes);

    let episode_count = duplicate_episodes.len();

    let files_to_delete = if !duplicate_episodes.is_empty() {
        display::print_duplicate_episodes(&show.name, duplicate_episodes)
    } else {
        Vec::new()
    };

    let file_count = files_to_delete.len();

    Ok((episode_count, file_count, files_to_delete))
}

fn print_summary(stats: &Statistics, path_prefix_to_remove: Option<&str>) {
    // Files are already deduplicated in the HashSet
    let mut sorted_files: Vec<&FileToDelete> = stats.files_to_delete.iter().collect();
    sorted_files.sort_by(|a, b| a.path.cmp(&b.path));

    // Calculate total space to be freed
    let total_space_bytes: i64 = stats.files_to_delete.iter().map(|f| f.size).sum();
    let total_space_gb = total_space_bytes as f64 / 1_073_741_824.0;

    println!("\n{}", "=".repeat(80));
    println!("Summary:");
    println!("   Total episodes with duplicates: {}", stats.total_duplicate_episodes);
    println!("   Total files to delete: {}", sorted_files.len());
    println!("   Estimated space savings: {:.2} GB", total_space_gb);
    println!("{}", "=".repeat(80));

    if !sorted_files.is_empty() {
        println!("\nFiles marked for deletion:");
        println!("{}", "=".repeat(80));
        for file in &sorted_files {
            let display_path = if let Some(prefix) = path_prefix_to_remove {
                file.path.strip_prefix(prefix).unwrap_or(&file.path)
            } else {
                &file.path
            };
            // Properly escape the path for bash
            let escaped_path = shell_escape::escape(display_path.into());
            println!("rm {}", escaped_path);
        }
        println!("{}", "=".repeat(80));
        println!("Total files to delete: {}", sorted_files.len());
        println!("Total space to free: {:.2} GB", total_space_gb);
    }
}
