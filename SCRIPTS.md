# Server Management Scripts

Easy scripts to start and stop the Rust Song Request Manager.

## For macOS/Linux:

### Start Server (automatically stops existing server):
```bash
./start.sh
```

## For Windows:

### Start Server (automatically stops existing server):
```cmd
start.bat
```
or
```powershell
.\start.ps1
```

## Manual Stop (if needed):

### macOS/Linux:
```bash
./stop.sh
```

### Windows:
```cmd
stop.bat
```
or
```powershell
.\stop.ps1
```

## What the Scripts Do:

### Start Script:
1. Checks if Rust is installed
2. Creates `.env` file if missing
3. Validates YouTube API key
4. Builds and runs the server
5. Shows helpful error messages

### Stop Script:
1. Finds and kills server processes
2. Confirms if server was stopped
3. No manual process management needed

## First Time Setup:

1. **Run start script** - It will create `.env` file
2. **Edit `.env`** - Add your YouTube API key:
   ```
   YOUTUBE_API_KEY=your_actual_api_key_here
   ```
3. **Run start script again** - Server starts automatically!

## Troubleshooting:

- **"Rust not found"** → Install Rust from https://rustup.rs/
- **"API key not set"** → Edit `.env` file with your YouTube API key
- **"Build failed"** → Check error messages and fix code issues
- **"Port in use"** → Run stop script first, then start script

## Notes:

- Scripts work on macOS, Linux, and Windows
- Server runs on `http://localhost:420`
- Use `Ctrl+C` to stop server manually if needed
- Scripts handle all dependency checking automatically
