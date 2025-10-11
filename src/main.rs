mod analyzer;
mod client;
mod display;
mod models;
mod selector;

use clap::{Parser, ValueEnum};
use client::JellyfinClient;
use display::FileToDelete;
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, Clone, ValueEnum)]
enum MediaType {
    /// Process TV shows only
    Tv,
    /// Process movies only
    Movies,
    /// Process both TV shows and movies
    Both,
}

/// A tool to find and manage duplicate episodes and movies in Jellyfin
#[derive(Parser, Debug)]
#[command(name = "jelly-dedup")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Jellyfin server URL
    #[arg(short, long, env = "JELLYFIN_URL", default_value = "http://localhost:8096")]
    jellyfin_url: String,

    /// Jellyfin API key
    #[arg(short, long, env = "JELLYFIN_API_KEY")]
    api_key: String,

    /// Path prefix to remove from displayed file paths
    #[arg(short, long, env = "PATH_PREFIX_TO_REMOVE")]
    path_prefix_to_remove: Option<String>,

    /// Type of media to process
    #[arg(short = 't', long, value_enum, default_value = "both")]
    media_type: MediaType,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let args = Args::parse();

    let client = JellyfinClient::new(args.jellyfin_url, args.api_key);

    let config = Config {
        path_prefix_to_remove: args.path_prefix_to_remove,
        media_type: args.media_type,
    };

    process_media(&client, &config).await?;

    Ok(())
}

struct Config {
    path_prefix_to_remove: Option<String>,
    media_type: MediaType,
}

struct Statistics {
    total_duplicate_episodes: usize,
    total_duplicate_movies: usize,
    total_duplicate_files: usize,
    files_to_delete: HashSet<FileToDelete>,
}

async fn process_media(client: &JellyfinClient, config: &Config) -> Result<(), Box<dyn Error>> {
    let mut stats = Statistics {
        total_duplicate_episodes: 0,
        total_duplicate_movies: 0,
        total_duplicate_files: 0,
        files_to_delete: HashSet::new(),
    };

    match config.media_type {
        MediaType::Tv => {
            process_all_shows(client, &mut stats).await?;
        }
        MediaType::Movies => {
            process_all_movies(client, &mut stats).await?;
        }
        MediaType::Both => {
            process_all_shows(client, &mut stats).await?;
            process_all_movies(client, &mut stats).await?;
        }
    }

    print_summary(&stats, config.path_prefix_to_remove.as_deref(), &config.media_type);

    Ok(())
}

async fn process_all_shows(client: &JellyfinClient, stats: &mut Statistics) -> Result<(), Box<dyn Error>> {
    println!("Fetching all TV shows from Jellyfin...\n");
    let shows = client.get_all_shows().await?;

    println!("Found {} TV shows\n", shows.len());
    println!("{}", "=".repeat(80));

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

    Ok(())
}

async fn process_all_movies(client: &JellyfinClient, stats: &mut Statistics) -> Result<(), Box<dyn Error>> {
    println!("\nFetching all movies from Jellyfin...\n");
    let movies = client.get_all_movies().await?;

    println!("Found {} movies\n", movies.len());
    println!("{}", "=".repeat(80));

    let duplicate_movie_groups = analyzer::filter_duplicate_movies(movies);

    if !duplicate_movie_groups.is_empty() {
        let movie_count = duplicate_movie_groups.len();
        let files_to_delete = display::print_duplicate_movies(duplicate_movie_groups);
        let file_count = files_to_delete.len();

        stats.total_duplicate_movies += movie_count;
        stats.total_duplicate_files += file_count;
        stats.files_to_delete.extend(files_to_delete);
    }

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

fn print_summary(stats: &Statistics, path_prefix_to_remove: Option<&str>, media_type: &MediaType) {
    // Files are already deduplicated in the HashSet
    let mut sorted_files: Vec<&FileToDelete> = stats.files_to_delete.iter().collect();
    sorted_files.sort_by(|a, b| a.path.cmp(&b.path));

    // Calculate total space to be freed
    let total_space_bytes: i64 = stats.files_to_delete.iter().map(|f| f.size).sum();
    let total_space_gb = total_space_bytes as f64 / 1_073_741_824.0;

    println!("\n{}", "=".repeat(80));
    println!("Summary:");

    match media_type {
        MediaType::Tv => {
            println!("   Total episodes with duplicates: {}", stats.total_duplicate_episodes);
        }
        MediaType::Movies => {
            println!("   Total movies with duplicates: {}", stats.total_duplicate_movies);
        }
        MediaType::Both => {
            println!("   Total episodes with duplicates: {}", stats.total_duplicate_episodes);
            println!("   Total movies with duplicates: {}", stats.total_duplicate_movies);
        }
    }

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
            display_path.to_owned().insert_str(0, ".");
            let escaped_path = shell_escape::escape(display_path.into());
            println!("rm .{}", escaped_path);
        }
        println!("{}", "=".repeat(80));
        println!("Total files to delete: {}", sorted_files.len());
        println!("Total space to free: {:.2} GB", total_space_gb);
    }
}
