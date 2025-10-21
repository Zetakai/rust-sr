use worker::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
struct YouTubeURL {
    id: Option<i64>,
    title: String,
    url: String,
    user: String,
    created_at: Option<String>,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Rust Song Request Manager - Cloudflare Worker"))
        .get("/host", |_, _| Response::ok("Host Interface - Rust Worker"))
        .post("/url", |mut req, _| async move {
            let body: serde_json::Value = req.json().await?;
            Response::ok("Song added (Rust implementation)")
        })
        .get("/urls", |_, _| Response::ok("[]"))
        .get("/url/oldest", |_, _| Response::ok("No songs in queue"))
        .run(req, env)
        .await
}
