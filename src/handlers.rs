use warp::reply::{json, with_status};
use warp::{Rejection, Reply};
use crate::database::Database;
use crate::youtube::YouTubeAPI;
use crate::models::*;
use anyhow::Result;

pub async fn requester_handler() -> Result<impl Reply, Rejection> {
    match std::fs::read_to_string("static/requester.html") {
        Ok(content) => Ok(warp::reply::html(content)),
        Err(_) => Ok(warp::reply::html("Error loading requester page".to_string())),
    }
}

pub async fn host_handler() -> Result<impl Reply, Rejection> {
    match std::fs::read_to_string("static/host.html") {
        Ok(content) => Ok(warp::reply::html(content)),
        Err(_) => Ok(warp::reply::html("Error loading host page".to_string())),
    }
}

pub async fn add_song(
    db: Database,
    youtube_api: YouTubeAPI,
    request: AddSongRequest,
) -> Result<impl Reply, Rejection> {
    let song = if let Some(url) = request.url {
        // Direct URL addition
        let _video_id = YouTubeAPI::extract_video_id(&url)
            .ok_or_else(|| warp::reject::custom(Error::InvalidUrl))?;
        
        YouTubeURL {
            id: None,
            title: request.title,
            url,
            user: request.user,
            created_at: None,
        }
    } else {
        // Search for song
        let search_result = youtube_api.search_song(&request.title).await
            .map_err(|e| {
                eprintln!("YouTube search error: {:?}", e);
                warp::reject::custom(Error::YouTubeSearchFailed)
            })?;
        
        YouTubeURL {
            id: None,
            title: search_result.title,
            url: search_result.url,
            user: request.user,
            created_at: None,
        }
    };

    match db.add_song(&song).await {
        Ok(_) => Ok(warp::reply::with_status(
            json(&serde_json::json!({
                "message": format!("Song added successfully: {} by {}", song.title, song.user)
            })),
            warp::http::StatusCode::CREATED,
        )),
        Err(_) => Ok(warp::reply::with_status(
            json(&serde_json::json!({
                "error": "URL already exists or error inserting URL"
            })),
            warp::http::StatusCode::CONFLICT,
        )),
    }
}

pub async fn delete_song(
    db: Database,
    request: DeleteSongRequest,
) -> Result<impl Reply, Rejection> {
    match db.delete_song_by_url(&request.url).await {
        Ok(true) => Ok(warp::reply::with_status(
            json(&serde_json::json!({
                "message": "URL deleted successfully"
            })),
            warp::http::StatusCode::OK,
        )),
        Ok(false) => Ok(warp::reply::with_status(
            json(&serde_json::json!({
                "error": "URL not found"
            })),
            warp::http::StatusCode::NOT_FOUND,
        )),
        Err(_) => Ok(warp::reply::with_status(
            json(&serde_json::json!({
                "error": "Error deleting URL"
            })),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

pub async fn get_oldest_song(db: Database, youtube_api: YouTubeAPI, playlist_id: Option<i64>) -> Result<impl Reply, Rejection> {
    match db.get_oldest_song().await {
        Ok(Some(song)) => {
            // Delete the song after retrieving it
            let _ = db.delete_song_by_id(song.id.unwrap()).await;
            Ok(json(&song))
        }
        Ok(None) => {
            // Main queue is empty, try to get a song from playlist queue
            match db.get_oldest_playlist_song().await {
                Ok(Some(playlist_song)) => {
                    // Delete the song from playlist queue after retrieving it
                    let _ = db.delete_playlist_song_by_id(playlist_song.id.unwrap()).await;
                    
                    // Convert to YouTubeURL format
                    let song = YouTubeURL {
                        id: None,
                        title: playlist_song.title,
                        url: playlist_song.url,
                        user: "Playlist".to_string(),
                        created_at: None,
                    };
                    Ok(json(&song))
                }
                Ok(None) => {
                    // Playlist queue is also empty, return error
                    Ok(json(&serde_json::json!({
                        "error": "No songs in queue or playlist"
                    })))
                }
                Err(_) => Ok(json(&serde_json::json!({
                    "error": "Error fetching playlist song"
                }))),
            }
        }
        Err(_) => Ok(json(&serde_json::json!({
            "error": "Error fetching URL"
        }))),
    }
}

pub async fn get_all_songs(db: Database) -> Result<impl Reply, Rejection> {
    match db.get_all_songs().await {
        Ok(songs) => Ok(json(&songs)),
        Err(_) => Ok(json(&serde_json::json!({
            "error": "Error fetching URLs"
        }))),
    }
}

pub async fn get_recommendation(
    db: Database,
    youtube_api: YouTubeAPI,
) -> Result<impl Reply, Rejection> {
    let recent_recommendations = db.get_recent_recommendations().await
        .unwrap_or_default();

    match youtube_api.get_recommendation(&recent_recommendations).await {
        Ok(recommendation) => {
            // Store the recommendation
            let _ = db.store_recommendation(&recommendation.video_id).await;
            
            let song = YouTubeURL {
                id: None,
                title: recommendation.title,
                url: recommendation.url,
                user: "Recommended".to_string(),
                created_at: None,
            };
            
            Ok(json(&song))
        }
        Err(_) => Ok(json(&serde_json::json!({
            "error": "Error finding recommendation"
        }))),
    }
}

// Playlist handlers
pub async fn create_playlist(
    db: Database,
    request: CreatePlaylistRequest,
) -> Result<impl Reply, Rejection> {
    let playlist = Playlist {
        id: None,
        name: request.name,
        description: request.description,
        youtube_playlist_url: request.youtube_playlist_url,
        created_at: None,
    };

    match db.create_playlist(&playlist).await {
        Ok(id) => Ok(with_status(
            json(&serde_json::json!({
                "id": id,
                "message": "Playlist created successfully"
            })),
            warp::http::StatusCode::CREATED,
        )),
        Err(_) => Ok(warp::reply::with_status(
            json(&serde_json::json!({
                "error": "Error creating playlist"
            })),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

pub async fn get_all_playlists(db: Database) -> Result<impl Reply, Rejection> {
    match db.get_all_playlists().await {
        Ok(playlists) => Ok(json(&playlists)),
        Err(_) => Ok(json(&serde_json::json!({
            "error": "Error fetching playlists"
        }))),
    }
}

pub async fn get_playlist(db: Database, id: i64) -> Result<impl Reply, Rejection> {
    match db.get_playlist(id).await {
        Ok(Some(playlist)) => Ok(json(&playlist)),
        Ok(None) => Ok(json(&serde_json::json!({
            "error": "Playlist not found"
        }))),
        Err(_) => Ok(json(&serde_json::json!({
            "error": "Error fetching playlist"
        }))),
    }
}

pub async fn add_to_playlist(
    db: Database,
    request: AddToPlaylistRequest,
) -> Result<impl Reply, Rejection> {
    let item = PlaylistItem {
        id: None,
        playlist_id: request.playlist_id,
        title: request.title,
        url: request.url,
        user: request.user,
        created_at: None,
    };

    match db.add_to_playlist(&item).await {
        Ok(_) => Ok(warp::reply::with_status(
            json(&serde_json::json!({
                "message": "Song added to playlist successfully"
            })),
            warp::http::StatusCode::CREATED,
        )),
        Err(_) => Ok(warp::reply::with_status(
            json(&serde_json::json!({
                "error": "Error adding song to playlist"
            })),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

pub async fn get_playlist_items(db: Database, id: i64) -> Result<impl Reply, Rejection> {
    match db.get_playlist_items(id).await {
        Ok(items) => Ok(json(&items)),
        Err(_) => Ok(json(&serde_json::json!({
            "error": "Error fetching playlist items"
        }))),
    }
}

pub async fn get_random_playlist_item(db: Database, id: i64) -> Result<impl Reply, Rejection> {
    match db.get_random_playlist_item(id).await {
        Ok(Some(item)) => Ok(json(&item)),
        Ok(None) => Ok(json(&serde_json::json!({
            "error": "No items in playlist"
        }))),
        Err(_) => Ok(json(&serde_json::json!({
            "error": "Error fetching playlist item"
        }))),
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidUrl,
    YouTubeSearchFailed,
}

impl warp::reject::Reject for Error {}

// Playlist queue handlers
pub async fn add_playlist_songs(
    db: Database,
    youtube_api: YouTubeAPI,
    request: AddPlaylistSongsRequest,
) -> Result<impl Reply, Rejection> {
    match youtube_api.get_playlist_songs(&request.playlist_url, &[]).await {
        Ok(songs) => {
            let mut added_count = 0;
            for song in songs {
                if let Err(e) = db.add_to_playlist_queue(&song.title, &song.url, &song.video_id).await {
                    eprintln!("Error adding song to playlist queue: {:?}", e);
                } else {
                    added_count += 1;
                }
            }
            
            Ok(warp::reply::with_status(
                json(&serde_json::json!({
                    "message": format!("Added {} songs to playlist queue", added_count)
                })),
                warp::http::StatusCode::CREATED,
            ))
        }
        Err(e) => {
            eprintln!("Error fetching playlist songs: {:?}", e);
            Ok(warp::reply::with_status(
                json(&serde_json::json!({
                    "error": "Error fetching playlist songs"
                })),
                warp::http::StatusCode::BAD_REQUEST,
            ))
        }
    }
}

pub async fn get_playlist_songs(db: Database) -> Result<impl Reply, Rejection> {
    match db.get_all_playlist_songs().await {
        Ok(songs) => Ok(json(&songs)),
        Err(_) => Ok(json(&serde_json::json!({
            "error": "Error fetching playlist songs"
        }))),
    }
}

pub async fn clear_playlist_queue(db: Database) -> Result<impl Reply, Rejection> {
    match db.clear_playlist_queue().await {
        Ok(_) => Ok(json(&serde_json::json!({
            "message": "Playlist queue cleared"
        }))),
        Err(_) => Ok(json(&serde_json::json!({
            "error": "Error clearing playlist queue"
        }))),
    }
}
