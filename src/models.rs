use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeURL {
    pub id: Option<i64>,
    pub title: String,
    pub url: String,
    pub user: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub youtube_playlist_url: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistItem {
    pub id: Option<i64>,
    pub playlist_id: i64,
    pub title: String,
    pub url: String,
    pub user: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedVideo {
    pub id: Option<i64>,
    pub video_id: String,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeSearchResult {
    pub title: String,
    pub url: String,
    pub video_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSongRequest {
    pub title: String,
    pub user: String,
    pub url: Option<String>, // Optional for direct URL addition
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSongRequest {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlaylistRequest {
    pub name: String,
    pub description: Option<String>,
    pub youtube_playlist_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddToPlaylistRequest {
    pub playlist_id: i64,
    pub title: String,
    pub url: String,
    pub user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistProgress {
    pub id: Option<i64>,
    pub playlist_id: i64,
    pub video_id: String,
    pub title: String,
    pub url: String,
    pub played_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistQueueItem {
    pub id: Option<i64>,
    pub title: String,
    pub url: String,
    pub video_id: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddPlaylistSongsRequest {
    pub playlist_url: String,
}
