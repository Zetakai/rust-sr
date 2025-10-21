const { execSync } = require('child_process');
const fs = require('fs');

console.log('Starting build process...');

try {
  // Check if we're in a Cloudflare Pages environment
  if (process.env.CF_PAGES) {
    console.log('Running in Cloudflare Pages environment');
    
    // Install Rust toolchain
    console.log('Installing Rust toolchain...');
    execSync('curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y', { stdio: 'inherit' });
    
    // Source cargo env
    process.env.PATH = process.env.PATH + ':' + process.env.HOME + '/.cargo/bin';
    
    // Install wasm-pack
    console.log('Installing wasm-pack...');
    execSync('curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh', { stdio: 'inherit' });
    
    // Add wasm32 target
    console.log('Adding wasm32 target...');
    execSync('rustup target add wasm32-unknown-unknown', { stdio: 'inherit' });
  } else {
    console.log('Running in local environment');
    // Check if wasm-pack is available
    try {
      execSync('wasm-pack --version', { stdio: 'pipe' });
    } catch (e) {
      console.log('Installing wasm-pack...');
      execSync('curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh', { stdio: 'inherit' });
    }
  }

  // Build the worker
  console.log('Building Rust worker...');
  execSync('cd worker && wasm-pack build --target web --out-dir ../dist', { stdio: 'inherit' });

  console.log('Build completed successfully!');
} catch (error) {
  console.error('Build failed:', error.message);
  process.exit(1);
}
