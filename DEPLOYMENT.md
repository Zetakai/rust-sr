# Git Integration with Cloudflare Workers

## Setup Instructions

### 1. GitHub Repository Setup
1. Create a new repository on GitHub
2. Push your code to the repository
3. Go to your Cloudflare dashboard → Workers & Pages → Your Worker → Settings → Integrations

### 2. Cloudflare Workers Git Integration
1. In Cloudflare dashboard, go to **Workers & Pages**
2. Click **Create application** → **Pages** → **Connect to Git**
3. Connect your GitHub repository
4. Set the following configuration:

#### Build Settings:
- **Framework preset**: None
- **Build command**: `npm run build`
- **Build output directory**: `dist`
- **Root directory**: `/` (leave empty)

#### Environment Variables:
- Add `YOUTUBE_API_KEY` in the Environment Variables section

### 3. Manual Deployment (Alternative)
If you prefer manual deployment instead of Git integration:

#### Build Command:
```bash
npm run build
```

#### Deploy Command:
```bash
wrangler deploy
```

### 4. GitHub Actions (Advanced)
The `.github/workflows/deploy.yml` file is already configured for automatic deployment.

#### Required Secrets:
Add these secrets to your GitHub repository (Settings → Secrets and variables → Actions):

1. `CLOUDFLARE_API_TOKEN`: Get from Cloudflare dashboard → My Profile → API Tokens
2. `CLOUDFLARE_ACCOUNT_ID`: Get from Cloudflare dashboard → Right sidebar

### 5. Local Development
```bash
# Install dependencies
npm install

# Build the worker
npm run build

# Deploy manually
npm run deploy

# Or use wrangler directly
wrangler dev
```

## Commands Summary

| Action | Command |
|--------|---------|
| **Build** | `npm run build` |
| **Deploy** | `npm run deploy` or `wrangler deploy` |
| **Dev** | `npm run dev` or `wrangler dev` |

## Environment Variables
- Set `YOUTUBE_API_KEY` in Cloudflare dashboard or use `wrangler secret put YOUTUBE_API_KEY`
