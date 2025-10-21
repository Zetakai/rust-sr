# Rust Song Request Manager

A Rust-based application that allows users to request YouTube songs by title. The app retrieves the closest YouTube video match using the YouTube Data API and manages song queues with playlist support.

## Features

- Add songs by title and retrieve the closest matching YouTube video
- Store video titles and URLs in a local SQLite database
- Manage song queue (add, delete, and play songs in order)
- YouTube Player integration for autoplaying requested videos
- **Playlist support**: When the queue is empty, play songs from a chosen playlist
- Smart recommendations that avoid compilations and recently played songs
- **Playlist Queue**: Separate queue for playlist songs with automatic cleanup
- **Pagination**: Fetch unlimited songs from YouTube playlists (up to 1000)

## Setup

### Prerequisites

- Rust (latest stable version)
- YouTube Data API key

### Getting a YouTube API Key

1. Go to the [Google Cloud Console](https://console.developers.google.com/)
2. Create a new project or select an existing one
3. Enable the YouTube Data API v3
4. Create credentials (API Key)
5. Copy the API key for use in your `.env` file

### Installation

1. Clone or download this repository
2. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```
3. Edit `.env` and add your YouTube API key:
   ```
   YOUTUBE_API_KEY=your_actual_youtube_api_key_here
   ```
4. Install dependencies:
   ```bash
   cargo build
   ```

### Running the Application

1. Start the server:
   ```bash
   cargo run
   ```

2. Access the application:
   - **Requester page**: http://localhost:420/ (for users to request songs)
   - **Host page**: http://localhost:420/host (for managing the queue and playlists)

## API Endpoints

### Main Queue
| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | Loads the requester frontend |
| `/host` | GET | Loads the host frontend |
| `/url` | POST | Adds a new song to the queue |
| `/url` | DELETE | Removes a song from the queue |
| `/url/oldest` | GET | Gets and deletes the oldest song |
| `/urls` | GET | Lists all songs in the queue |
| `/recommendation` | GET | Gets a recommended video |

### Playlist Queue
| Endpoint | Method | Description |
|----------|--------|-------------|
| `/playlist-queue` | POST | Add songs from YouTube playlist URL |
| `/playlist-queue` | GET | Get all songs in playlist queue |
| `/playlist-queue` | DELETE | Clear playlist queue |

### Legacy Playlist Management
| Endpoint | Method | Description |
|----------|--------|-------------|
| `/playlists` | GET | Lists all playlists |
| `/playlists` | POST | Creates a new playlist |
| `/playlists/{id}` | GET | Gets a specific playlist |
| `/playlists/{id}/items` | POST | Adds a song to a playlist |
| `/playlists/{id}/items` | GET | Gets all items in a playlist |
| `/playlists/{id}/random` | GET | Gets a random item from a playlist |

## Queue Priority System

The application follows a strict priority order:

1. **Main Queue**: User-requested songs (highest priority)
2. **Playlist Queue**: Songs from YouTube playlists (fallback when main queue is empty)
3. **Recommendations**: YouTube recommendations (only when both queues are empty)

## Playlist Queue System

- **Simple URL Input**: Just paste a YouTube playlist URL
- **Automatic Fetching**: Fetches all songs from the playlist (up to 1000 songs)
- **Auto-cleanup**: Songs are automatically removed after being played
- **No Duplicates**: Each song plays only once
- **Real-time Updates**: Frontend automatically refreshes when songs are played

## Frontend

- **requester.html**: Allows users to add songs to the queue by entering a song title
- **host.html**: Displays both main queue and playlist queue, includes playlist URL input and management

## Usage

### Adding Songs
1. Go to http://localhost:420/ (requester page)
2. Enter a song title and your name
3. The system will find the closest YouTube match and add it to the queue

### Managing Playlists
1. Go to http://localhost:420/host (host page)
2. Paste a YouTube playlist URL in the "Add Playlist" section
3. Click "Add All Songs" to fetch all songs from the playlist
4. Songs will automatically play when the main queue is empty

### Queue Management
- **Main Queue**: User-requested songs play first
- **Playlist Queue**: Plays when main queue is empty
- **Automatic Cleanup**: Played songs are removed automatically
- **Real-time Updates**: Interface updates automatically

## Security Notes

- **Never commit your `.env` file**: It contains your API key and should remain private
- **The `.env.example` file is safe to commit**: It only contains placeholder values
- **API Key Security**: Keep your YouTube API key secure and don't share it publicly

## Troubleshooting

- **Error loading API key**: Make sure the `.env` file is correctly set up in the root directory
- **YouTube API errors**: Ensure your API key is valid and has access to the YouTube Data API v3
- **Database errors**: The SQLite database will be created automatically on first run
- **Playlist not loading**: Check that the YouTube playlist URL is public and accessible

## License

This project is licensed under the MIT License.
