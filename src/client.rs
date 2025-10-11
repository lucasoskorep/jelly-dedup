use crate::models::{Episode, EpisodesResponse, Item, ItemsResponse, Movie, MoviesResponse};
use std::error::Error;

pub struct JellyfinClient {
    base_url: String,
    api_key: String,
    client: reqwest::Client,
}

impl JellyfinClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_all_shows(&self) -> Result<Vec<Item>, Box<dyn Error>> {
        let url = format!(
            "{}/Items?IncludeItemTypes=Series&Recursive=true&api_key={}",
            self.base_url, self.api_key
        );

        let response = self.client.get(&url).send().await?;
        let items_response: ItemsResponse = response.json().await?;

        Ok(items_response.items)
    }

    pub async fn get_episodes_for_show(&self, show_id: &str) -> Result<Vec<Episode>, Box<dyn Error>> {
        let url = format!(
            "{}/Shows/{}/Episodes?Fields=Path,MediaSources&api_key={}",
            self.base_url, show_id, self.api_key
        );

        let response = self.client.get(&url).send().await?;
        let episodes_response: EpisodesResponse = response.json().await?;

        Ok(episodes_response.items)
    }

    pub async fn get_all_movies(&self) -> Result<Vec<Movie>, Box<dyn Error>> {
        let url = format!(
            "{}/Items?IncludeItemTypes=Movie&Recursive=true&Fields=Path,MediaSources,ProductionYear&api_key={}",
            self.base_url, self.api_key
        );

        let response = self.client.get(&url).send().await?;
        let movies_response: MoviesResponse = response.json().await?;

        Ok(movies_response.items)
    }
}
