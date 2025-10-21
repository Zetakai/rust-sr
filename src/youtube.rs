use reqwest::Client;
use serde::Deserialize;
use anyhow::Result;
use crate::models::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct YouTubeAPI {
    client: Client,
    api_key: String,
}

#[derive(Debug, Deserialize)]
struct YouTubeSearchResponse {
    items: Vec<YouTubeSearchItem>,
}

#[derive(Debug, Deserialize)]
struct YouTubeSearchItem {
    id: YouTubeVideoId,
    snippet: YouTubeSnippet,
}

#[derive(Debug, Deserialize)]
struct YouTubeVideoId {
    #[serde(rename = "videoId")]
    video_id: String,
}

#[derive(Debug, Deserialize)]
struct YouTubeSnippet {
    title: String,
    description: String,
    #[serde(rename = "channelTitle")]
    channel_title: String,
}

#[derive(Debug, Deserialize)]
struct YouTubePlaylistResponse {
    items: Vec<YouTubePlaylistItem>,
    #[serde(rename = "nextPageToken")]
    next_page_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct YouTubePlaylistItem {
    snippet: YouTubePlaylistSnippet,
}

#[derive(Debug, Deserialize)]
struct YouTubePlaylistSnippet {
    title: String,
    #[serde(rename = "resourceId")]
    resource_id: YouTubeResourceId,
}

#[derive(Debug, Deserialize)]
struct YouTubeResourceId {
    #[serde(rename = "videoId")]
    video_id: String,
    #[serde(rename = "kind")]
    kind: Option<String>,
}

impl YouTubeAPI {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn search_song(&self, query: &str) -> Result<YouTubeSearchResult> {
        let url = "https://www.googleapis.com/youtube/v3/search";
        let params = [
            ("part", "snippet"),
            ("q", query),
            ("type", "video"),
            ("maxResults", "1"),
            ("key", &self.api_key),
        ];

        let response = self.client.get(url).query(&params).send().await?;
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            eprintln!("YouTube API error: Status {}, Body: {}", status, error_text);
            anyhow::bail!("YouTube API request failed with status: {}", status);
        }
        let search_response: YouTubeSearchResponse = response.json().await?;

        if let Some(item) = search_response.items.first() {
            Ok(YouTubeSearchResult {
                title: item.snippet.title.clone(),
                url: format!("https://www.youtube.com/watch?v={}", item.id.video_id),
                video_id: item.id.video_id.clone(),
            })
        } else {
            anyhow::bail!("No matching videos found");
        }
    }

    pub async fn get_recommendation(&self, recent_recommendations: &[String]) -> Result<YouTubeSearchResult> {
        let excluded_keywords = [
            "compilation", "playlist", "mix", "mashup", "megamix",
            "collection", "best of", "top 10", "top 20", "medley",
            "hits of", "greatest hits", "hour", "complete album",
            "songs", "tracks", "compilation", "non stop", "nonstop",
            "back to back", "b2b", "music collection", "jukeboxes", "jukebox",
            "all songs", "audio songs", "video songs", "chart", "songs",
        ];

        let excluded_content_keywords = [
            "indian", "hindi", "bollywood", "tamil", "telugu", "punjabi",
            "bhangra", "desi", "carnatic", "bharatanatyam",
        ];

        let search_queries = ["official music"];
        let query = search_queries[0]; // For now, use the first query

        let url = "https://www.googleapis.com/youtube/v3/search";
        let params = [
            ("part", "snippet"),
            ("q", query),
            ("type", "video"),
            ("videoCategoryId", "10"), // Music category
            ("maxResults", "25"),
            ("key", &self.api_key),
            ("relevanceLanguage", "en"),
            ("videoDuration", "short"),
        ];

        let response = self.client.get(url).query(&params).send().await?;
        let search_response: YouTubeSearchResponse = response.json().await?;

        let recent_set: HashSet<String> = recent_recommendations.iter().cloned().collect();

        // Filter out compilations and recently recommended videos
        let filtered_items: Vec<&YouTubeSearchItem> = search_response
            .items
            .iter()
            .filter(|item| {
                let title_lower = item.snippet.title.to_lowercase();
                let desc_lower = item.snippet.description.to_lowercase();
                let channel_lower = item.snippet.channel_title.to_lowercase();

                // Check for compilation keywords
                let is_compilation = excluded_keywords.iter().any(|keyword| {
                    title_lower.contains(keyword) || desc_lower.contains(keyword)
                });

                // Check for excluded content
                let is_excluded_content = excluded_content_keywords.iter().any(|keyword| {
                    title_lower.contains(keyword) || 
                    channel_lower.contains(keyword) || 
                    desc_lower.contains(keyword)
                });

                // Check for duration indicators
                let has_duration = title_lower.contains("minute") || 
                                 title_lower.contains("hour") ||
                                 regex::Regex::new(r"\d+\s*min").unwrap().is_match(&title_lower);

                // Check if recently recommended
                let is_recently_recommended = recent_set.contains(&item.id.video_id);

                // Check title length (very long titles often indicate compilations)
                let is_too_long = title_lower.len() > 70 && title_lower.split_whitespace().count() > 10;

                !is_compilation && !is_excluded_content && !has_duration && !is_recently_recommended && !is_too_long
            })
            .collect();

        if filtered_items.is_empty() {
            // If no filtered results, use all items but avoid recent ones
            let available_items: Vec<&YouTubeSearchItem> = search_response
                .items
                .iter()
                .filter(|item| !recent_set.contains(&item.id.video_id))
                .collect();

            if let Some(item) = available_items.first() {
                Ok(YouTubeSearchResult {
                    title: item.snippet.title.clone(),
                    url: format!("https://www.youtube.com/watch?v={}", item.id.video_id),
                    video_id: item.id.video_id.clone(),
                })
            } else {
                // If all items are recently recommended, pick the first one
                let item = &search_response.items[0];
                Ok(YouTubeSearchResult {
                    title: item.snippet.title.clone(),
                    url: format!("https://www.youtube.com/watch?v={}", item.id.video_id),
                    video_id: item.id.video_id.clone(),
                })
            }
        } else {
            // Pick a random item from filtered results
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            let item = filtered_items.choose(&mut rng).unwrap();
            
            Ok(YouTubeSearchResult {
                title: item.snippet.title.clone(),
                url: format!("https://www.youtube.com/watch?v={}", item.id.video_id),
                video_id: item.id.video_id.clone(),
            })
        }
    }

    pub async fn get_playlist_songs(&self, playlist_url: &str, played_songs: &[String]) -> Result<Vec<YouTubeSearchResult>> {
        let playlist_id = Self::extract_playlist_id(playlist_url)
            .ok_or_else(|| anyhow::anyhow!("Invalid playlist URL"))?;
        
        eprintln!("Extracted playlist ID: {}", playlist_id);
        
        let mut all_songs = Vec::new();
        let mut next_page_token: Option<String> = None;
        let mut total_fetched = 0;
        
        loop {
            let url = "https://www.googleapis.com/youtube/v3/playlistItems";
            let mut params = vec![
                ("part", "snippet"),
                ("playlistId", &playlist_id),
                ("maxResults", "50"),
                ("key", &self.api_key),
            ];
            
            if let Some(ref token) = next_page_token {
                params.push(("pageToken", token));
            }

            let response = self.client.get(url).query(&params).send().await?;
            let status = response.status();
            if !status.is_success() {
                let error_text = response.text().await.unwrap_or_default();
                eprintln!("YouTube Playlist API error: Status {}, Body: {}", status, error_text);
                eprintln!("Request URL: {}", url);
                eprintln!("Request params: {:?}", params);
                anyhow::bail!("YouTube Playlist API request failed with status: {}", status);
            }
            
            let response_text = response.text().await?;
            eprintln!("YouTube API Response (page {}): {} songs", total_fetched / 50 + 1, response_text.len());
            
            // Parse as generic JSON first to see the structure
            let json_value: serde_json::Value = serde_json::from_str(&response_text)
                .map_err(|e| {
                    eprintln!("JSON parsing error: {}", e);
                    e
                })?;
            
            let playlist_response: YouTubePlaylistResponse = serde_json::from_value(json_value)
                .map_err(|e| {
                    eprintln!("Struct parsing error: {}", e);
                    e
                })?;
            
            let played_set: std::collections::HashSet<String> = played_songs.iter().cloned().collect();
            
            let items_count = playlist_response.items.len();
            let page_songs: Vec<YouTubeSearchResult> = playlist_response
                .items
                .into_iter()
                .filter(|item| !played_set.contains(&item.snippet.resource_id.video_id))
                .map(|item| YouTubeSearchResult {
                    title: item.snippet.title,
                    url: format!("https://www.youtube.com/watch?v={}", item.snippet.resource_id.video_id),
                    video_id: item.snippet.resource_id.video_id,
                })
                .collect();
            
            all_songs.extend(page_songs);
            total_fetched += items_count;
            
            // Check if there are more pages
            if let Some(next_token) = playlist_response.next_page_token {
                next_page_token = Some(next_token);
                eprintln!("Fetched {} songs so far, continuing to next page...", total_fetched);
            } else {
                eprintln!("No more pages, total songs fetched: {}", total_fetched);
                break;
            }
            
            // Safety limit to prevent infinite loops
            if total_fetched > 1000 {
                eprintln!("Reached safety limit of 1000 songs, stopping");
                break;
            }
        }

        eprintln!("Total songs fetched: {}", all_songs.len());
        Ok(all_songs)
    }

    pub fn extract_playlist_id(url: &str) -> Option<String> {
        if url.contains("list=") {
            if let Some(parts) = url.split("list=").nth(1) {
                return Some(parts.split('&').next().unwrap_or(parts).to_string());
            }
        }
        None
    }

    pub fn extract_video_id(url: &str) -> Option<String> {
        if url.contains("v=") {
            if let Some(parts) = url.split("v=").nth(1) {
                return Some(parts.split('&').next().unwrap_or(parts).to_string());
            }
        } else if url.contains("youtu.be/") {
            if let Some(parts) = url.split("youtu.be/").nth(1) {
                return Some(parts.split('?').next().unwrap_or(parts).to_string());
            }
        }
        None
    }
}
