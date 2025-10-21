use sqlx::{SqlitePool, Row};
use crate::models::*;
use anyhow::Result;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        let db = Database { pool };
        db.init_tables().await?;
        Ok(db)
    }

    async fn init_tables(&self) -> Result<()> {
        // Create youtube_urls table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS youtube_urls (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                url TEXT NOT NULL UNIQUE,
                user TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create playlists table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS playlists (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                youtube_playlist_url TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create playlist_items table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS playlist_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                playlist_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                url TEXT NOT NULL,
                user TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (playlist_id) REFERENCES playlists (id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create recommended_videos table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS recommended_videos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                video_id TEXT NOT NULL UNIQUE,
                timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create indexes for better performance
        // Create playlist_queue table for separate playlist songs
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS playlist_queue (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                url TEXT NOT NULL,
                video_id TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create playlist_progress table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS playlist_progress (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                playlist_id INTEGER NOT NULL,
                video_id TEXT NOT NULL,
                title TEXT NOT NULL,
                url TEXT NOT NULL,
                played_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (playlist_id) REFERENCES playlists (id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_video_id ON recommended_videos(video_id)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_playlist_id ON playlist_items(playlist_id)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_playlist_progress_playlist_id ON playlist_progress(playlist_id)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_playlist_progress_video_id ON playlist_progress(video_id)")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // YouTube URLs operations
    pub async fn add_song(&self, song: &YouTubeURL) -> Result<()> {
        sqlx::query(
            "INSERT INTO youtube_urls (title, url, user) VALUES (?, ?, ?)"
        )
        .bind(&song.title)
        .bind(&song.url)
        .bind(&song.user)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_all_songs(&self) -> Result<Vec<YouTubeURL>> {
        let rows = sqlx::query(
            "SELECT id, title, url, user, created_at FROM youtube_urls ORDER BY id ASC"
        )
        .fetch_all(&self.pool)
        .await?;

        let songs = rows
            .into_iter()
            .map(|row| YouTubeURL {
                id: Some(row.get("id")),
                title: row.get("title"),
                url: row.get("url"),
                user: row.get("user"),
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(songs)
    }

    pub async fn get_oldest_song(&self) -> Result<Option<YouTubeURL>> {
        let row = sqlx::query(
            "SELECT id, title, url, user, created_at FROM youtube_urls ORDER BY id ASC LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(YouTubeURL {
                id: Some(row.get("id")),
                title: row.get("title"),
                url: row.get("url"),
                user: row.get("user"),
                created_at: row.get("created_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_song_by_url(&self, url: &str) -> Result<bool> {
        let result = sqlx::query("DELETE FROM youtube_urls WHERE url = ?")
            .bind(url)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_song_by_id(&self, id: i64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM youtube_urls WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // Playlist operations
    pub async fn create_playlist(&self, playlist: &Playlist) -> Result<i64> {
        let result = sqlx::query(
            "INSERT INTO playlists (name, description, youtube_playlist_url) VALUES (?, ?, ?)"
        )
        .bind(&playlist.name)
        .bind(&playlist.description)
        .bind(&playlist.youtube_playlist_url)
        .execute(&self.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get_all_playlists(&self) -> Result<Vec<Playlist>> {
        let rows = sqlx::query(
            "SELECT id, name, description, youtube_playlist_url, created_at FROM playlists ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let playlists = rows
            .into_iter()
            .map(|row| Playlist {
                id: Some(row.get("id")),
                name: row.get("name"),
                description: row.get("description"),
                youtube_playlist_url: row.get("youtube_playlist_url"),
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(playlists)
    }

    pub async fn get_playlist(&self, id: i64) -> Result<Option<Playlist>> {
        let row = sqlx::query(
            "SELECT id, name, description, youtube_playlist_url, created_at FROM playlists WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(Playlist {
                id: Some(row.get("id")),
                name: row.get("name"),
                description: row.get("description"),
                youtube_playlist_url: row.get("youtube_playlist_url"),
                created_at: row.get("created_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn add_to_playlist(&self, item: &PlaylistItem) -> Result<()> {
        sqlx::query(
            "INSERT INTO playlist_items (playlist_id, title, url, user) VALUES (?, ?, ?, ?)"
        )
        .bind(item.playlist_id)
        .bind(&item.title)
        .bind(&item.url)
        .bind(&item.user)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_playlist_items(&self, playlist_id: i64) -> Result<Vec<PlaylistItem>> {
        let rows = sqlx::query(
            "SELECT id, playlist_id, title, url, user, created_at FROM playlist_items WHERE playlist_id = ? ORDER BY id ASC"
        )
        .bind(playlist_id)
        .fetch_all(&self.pool)
        .await?;

        let items = rows
            .into_iter()
            .map(|row| PlaylistItem {
                id: Some(row.get("id")),
                playlist_id: row.get("playlist_id"),
                title: row.get("title"),
                url: row.get("url"),
                user: row.get("user"),
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(items)
    }

    pub async fn get_random_playlist_item(&self, playlist_id: i64) -> Result<Option<PlaylistItem>> {
        let row = sqlx::query(
            "SELECT id, playlist_id, title, url, user, created_at FROM playlist_items WHERE playlist_id = ? ORDER BY RANDOM() LIMIT 1"
        )
        .bind(playlist_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(PlaylistItem {
                id: Some(row.get("id")),
                playlist_id: row.get("playlist_id"),
                title: row.get("title"),
                url: row.get("url"),
                user: row.get("user"),
                created_at: row.get("created_at"),
            }))
        } else {
            Ok(None)
        }
    }

    // Recommendation tracking
    pub async fn store_recommendation(&self, video_id: &str) -> Result<()> {
        sqlx::query(
            "INSERT OR IGNORE INTO recommended_videos (video_id) VALUES (?)"
        )
        .bind(video_id)
        .execute(&self.pool)
        .await?;

        // Clean up old recommendations (keep only last 200)
        sqlx::query(
            "DELETE FROM recommended_videos WHERE id NOT IN (SELECT id FROM recommended_videos ORDER BY timestamp DESC LIMIT 200)"
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_recent_recommendations(&self) -> Result<Vec<String>> {
        let rows = sqlx::query(
            "SELECT video_id FROM recommended_videos WHERE timestamp > datetime('now', '-7 day') ORDER BY timestamp DESC LIMIT 100"
        )
        .fetch_all(&self.pool)
        .await?;

        let video_ids = rows
            .into_iter()
            .map(|row| row.get("video_id"))
            .collect();

        Ok(video_ids)
    }

    // Playlist progress operations
    pub async fn mark_song_played(&self, playlist_id: i64, video_id: &str, title: &str, url: &str) -> Result<()> {
        sqlx::query(
            "INSERT INTO playlist_progress (playlist_id, video_id, title, url) VALUES (?, ?, ?, ?)"
        )
        .bind(playlist_id)
        .bind(video_id)
        .bind(title)
        .bind(url)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_played_songs(&self, playlist_id: i64) -> Result<Vec<String>> {
        let rows = sqlx::query(
            "SELECT video_id FROM playlist_progress WHERE playlist_id = ?"
        )
        .bind(playlist_id)
        .fetch_all(&self.pool)
        .await?;

        let video_ids = rows
            .into_iter()
            .map(|row| row.get("video_id"))
            .collect();

        Ok(video_ids)
    }

    pub async fn reset_playlist_progress(&self, playlist_id: i64) -> Result<()> {
        sqlx::query(
            "DELETE FROM playlist_progress WHERE playlist_id = ?"
        )
        .bind(playlist_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // Playlist queue operations
    pub async fn add_to_playlist_queue(&self, title: &str, url: &str, video_id: &str) -> Result<()> {
        sqlx::query(
            "INSERT INTO playlist_queue (title, url, video_id) VALUES (?, ?, ?)"
        )
        .bind(title)
        .bind(url)
        .bind(video_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_oldest_playlist_song(&self) -> Result<Option<PlaylistQueueItem>> {
        let row = sqlx::query(
            "SELECT id, title, url, video_id, created_at FROM playlist_queue ORDER BY created_at ASC LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(PlaylistQueueItem {
                id: Some(row.get("id")),
                title: row.get("title"),
                url: row.get("url"),
                video_id: row.get("video_id"),
                created_at: row.get("created_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_playlist_song_by_id(&self, id: i64) -> Result<bool> {
        let result = sqlx::query(
            "DELETE FROM playlist_queue WHERE id = ?"
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn get_all_playlist_songs(&self) -> Result<Vec<PlaylistQueueItem>> {
        let rows = sqlx::query(
            "SELECT id, title, url, video_id, created_at FROM playlist_queue ORDER BY created_at ASC"
        )
        .fetch_all(&self.pool)
        .await?;

        let songs = rows
            .into_iter()
            .map(|row| PlaylistQueueItem {
                id: Some(row.get("id")),
                title: row.get("title"),
                url: row.get("url"),
                video_id: row.get("video_id"),
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(songs)
    }

    pub async fn clear_playlist_queue(&self) -> Result<()> {
        sqlx::query("DELETE FROM playlist_queue")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
