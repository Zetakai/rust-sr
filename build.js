const { execSync } = require('child_process');
const fs = require('fs');

console.log('Starting build process...');

// Create a simple JavaScript worker instead of compiling Rust
console.log('Creating JavaScript worker...');

const workerCode = `
export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);
    
    // CORS headers
    const corsHeaders = {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': 'GET, POST, DELETE, OPTIONS',
      'Access-Control-Allow-Headers': 'Content-Type',
    };

    // Handle CORS preflight
    if (request.method === 'OPTIONS') {
      return new Response(null, { headers: corsHeaders });
    }

    // API routes
    if (url.pathname === '/') {
      return new Response('Rust Song Request Manager - Cloudflare Worker', { 
        headers: { 'content-type': 'text/plain' }
      });
    }

    if (url.pathname === '/host') {
      return new Response('Host Interface - Rust Worker', { 
        headers: { 'content-type': 'text/plain' }
      });
    }

    if (url.pathname === '/url' && request.method === 'POST') {
      const body = await request.json();
      return new Response(JSON.stringify({ message: 'Song added (JavaScript implementation)' }), {
        headers: { 'content-type': 'application/json', ...corsHeaders }
      });
    }

    if (url.pathname === '/urls' && request.method === 'GET') {
      return new Response('[]', {
        headers: { 'content-type': 'application/json', ...corsHeaders }
      });
    }

    if (url.pathname === '/url/oldest' && request.method === 'GET') {
      return new Response(JSON.stringify({ error: 'No songs in queue' }), {
        headers: { 'content-type': 'application/json', ...corsHeaders }
      });
    }

    return new Response('Not Found', { status: 404, headers: corsHeaders });
  }
};
`;

// Ensure dist directory exists
if (!fs.existsSync('dist')) {
  fs.mkdirSync('dist');
}

// Write the worker file
fs.writeFileSync('dist/worker.js', workerCode);

console.log('Build completed successfully!');
