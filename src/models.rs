use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ItemsResponse {
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MediaStream {
    #[serde(rename = "Type")]
    pub stream_type: Option<String>,
    #[serde(rename = "Height")]
    pub height: Option<i32>,
    #[serde(rename = "Codec")]
    pub codec: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MediaSource {
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "Container")]
    pub container: Option<String>,
    #[serde(rename = "Size")]
    pub size: Option<i64>,
    #[serde(rename = "Bitrate")]
    pub bitrate: Option<i64>,
    #[serde(rename = "Height")]
    pub height: Option<i32>,
    #[serde(rename = "MediaStreams")]
    pub media_streams: Option<Vec<MediaStream>>,
}

#[derive(Debug, Deserialize)]
pub struct EpisodesResponse {
    #[serde(rename = "Items")]
    pub items: Vec<Episode>,
}

#[derive(Debug, Deserialize)]
pub struct Episode {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "IndexNumber")]
    pub episode_number: Option<u32>,
    #[serde(rename = "ParentIndexNumber")]
    pub season_number: Option<u32>,
    #[serde(rename = "MediaSources")]
    pub media_sources: Option<Vec<MediaSource>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Movie {
    #[serde(rename = "Id")]
    pub _id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ProductionYear")]
    pub year: Option<u32>,
    #[serde(rename = "MediaSources")]
    pub media_sources: Option<Vec<MediaSource>>,
}

#[derive(Debug, Deserialize)]
pub struct MoviesResponse {
    #[serde(rename = "Items")]
    pub items: Vec<Movie>,
}
