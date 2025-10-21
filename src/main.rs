mod models;
mod database;
mod youtube;
mod handlers;

use dotenv::dotenv;
use std::env;
use warp::Filter;
use crate::database::Database;
use crate::youtube::YouTubeAPI;
use crate::handlers::*;
use crate::models::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let api_key = env::var("YOUTUBE_API_KEY")
        .expect("YOUTUBE_API_KEY not found in environment");

    let database_url = "sqlite:./youtube_urls.db";
    let db = Database::new(database_url).await?;
    let youtube_api = YouTubeAPI::new(api_key);

    // Create a filter that clones the database and YouTube API
    let db_filter = warp::any().map(move || db.clone());
    let youtube_filter = warp::any().map(move || youtube_api.clone());

    // Static file serving
    let static_files = warp::path("static")
        .and(warp::fs::dir("static/"));

    // Routes
    let requester_route = warp::path::end()
        .and_then(requester_handler);

    let host_route = warp::path("host")
        .and_then(host_handler);

    let add_song_route = warp::path("url")
        .and(warp::post())
        .and(db_filter.clone())
        .and(youtube_filter.clone())
        .and(warp::body::json())
        .and_then(add_song);

    let delete_song_route = warp::path("url")
        .and(warp::delete())
        .and(db_filter.clone())
        .and(warp::body::json())
        .and_then(delete_song);

    let get_oldest_song_route = warp::path("url")
        .and(warp::path("oldest"))
        .and(warp::get())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(db_filter.clone())
        .and(youtube_filter.clone())
        .and_then(|query: std::collections::HashMap<String, String>, db: Database, youtube_api: YouTubeAPI| {
            let playlist_id = query.get("playlist_id").and_then(|id| id.parse::<i64>().ok());
            get_oldest_song(db, youtube_api, playlist_id)
        });

    let get_all_songs_route = warp::path("urls")
        .and(warp::get())
        .and(db_filter.clone())
        .and_then(get_all_songs);

    let get_recommendation_route = warp::path("recommendation")
        .and(warp::get())
        .and(db_filter.clone())
        .and(youtube_filter.clone())
        .and_then(get_recommendation);

    // Playlist routes
    let create_playlist_route = warp::path("playlists")
        .and(warp::post())
        .and(db_filter.clone())
        .and(warp::body::json())
        .and_then(create_playlist);

    let get_all_playlists_route = warp::path("playlists")
        .and(warp::get())
        .and(db_filter.clone())
        .and_then(get_all_playlists);

    let get_playlist_route = warp::path("playlists")
        .and(warp::path::param::<i64>())
        .and(warp::get())
        .and(db_filter.clone())
        .and_then(|id: i64, db: Database| get_playlist(db, id));

    let add_to_playlist_route = warp::path("playlists")
        .and(warp::path::param::<i64>())
        .and(warp::path("items"))
        .and(warp::post())
        .and(db_filter.clone())
        .and(warp::body::json())
        .and_then(|id: i64, db: Database, request: AddToPlaylistRequest| {
            let mut request = request;
            request.playlist_id = id;
            add_to_playlist(db, request)
        });

    let get_playlist_items_route = warp::path("playlists")
        .and(warp::path::param::<i64>())
        .and(warp::path("items"))
        .and(warp::get())
        .and(db_filter.clone())
        .and_then(|id: i64, db: Database| get_playlist_items(db, id));

    let get_random_playlist_item_route = warp::path("playlists")
        .and(warp::path::param::<i64>())
        .and(warp::path("random"))
        .and(warp::get())
        .and(db_filter.clone())
        .and_then(|id: i64, db: Database| get_random_playlist_item(db, id));

    // Playlist queue routes
    let add_playlist_songs_route = warp::path("playlist-queue")
        .and(warp::post())
        .and(db_filter.clone())
        .and(youtube_filter.clone())
        .and(warp::body::json())
        .and_then(add_playlist_songs);

    let get_playlist_songs_route = warp::path("playlist-queue")
        .and(warp::get())
        .and(db_filter.clone())
        .and_then(get_playlist_songs);

    let clear_playlist_queue_route = warp::path("playlist-queue")
        .and(warp::delete())
        .and(db_filter.clone())
        .and_then(clear_playlist_queue);

    let routes = requester_route
        .or(host_route)
        .or(add_song_route)
        .or(delete_song_route)
        .or(get_oldest_song_route)
        .or(get_all_songs_route)
        .or(get_recommendation_route)
        .or(create_playlist_route)
        .or(get_all_playlists_route)
        .or(get_playlist_route)
        .or(add_to_playlist_route)
        .or(get_playlist_items_route)
        .or(get_random_playlist_item_route)
        .or(add_playlist_songs_route)
        .or(get_playlist_songs_route)
        .or(clear_playlist_queue_route)
        .or(static_files)
        .with(warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type"])
            .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"]));

    println!("Starting server on http://localhost:420/");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 420))
        .await;

    Ok(())
}
