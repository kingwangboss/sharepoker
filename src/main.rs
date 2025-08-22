mod game;
mod models;
mod handlers;
mod error;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tower_http::services::ServeDir;
use tracing_subscriber;

use crate::game::GameManager;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 创建游戏管理器
    let game_manager = Arc::new(GameManager::new());

    // 创建路由
    let app = Router::new()
        .route("/api/health", get(|| async { "OK" }))
        .route("/api/game/upload", post(handlers::game::upload_player_image))
        .route("/api/game/:code", get(handlers::game::get_game_info))
        .with_state(game_manager)
        .nest_service("/", ServeDir::new("."))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Server running on http://0.0.0.0:3001");
    
    axum::serve(listener, app).await.unwrap();
}
