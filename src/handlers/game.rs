use crate::error::Result;
use crate::game::GameManager;
use crate::models::{GameResponse, PlayerInfo, UploadPlayerRequest};
use axum::{
	extract::{Path, State},
	Json,
};
use std::sync::Arc;

pub async fn upload_player_image(
	State(game_manager): State<Arc<GameManager>>,
	Json(payload): Json<UploadPlayerRequest>,
) -> Result<Json<GameResponse>> {
	let game = game_manager.upload_player_image(
		payload.game_code,
		payload.username,
		payload.image,
	)?;

	let players: Vec<PlayerInfo> = game.players.into_iter().map(|p| PlayerInfo {
		username: p.username,
		hand_image: p.hand_image,
		uploaded_at: p.uploaded_at,
	}).collect();

	let response = GameResponse {
		code: game.code,
		players,
		created_at: game.created_at,
	};

	Ok(Json(response))
}

pub async fn get_game_info(
	State(game_manager): State<Arc<GameManager>>,
	Path(code): Path<String>,
) -> Result<Json<GameResponse>> {
	let game = game_manager.get_game(&code)?;

	let players: Vec<PlayerInfo> = game.players.into_iter().map(|p| PlayerInfo {
		username: p.username,
		hand_image: p.hand_image,
		uploaded_at: p.uploaded_at,
	}).collect();

	let response = GameResponse {
		code: game.code,
		players,
		created_at: game.created_at,
	};

	Ok(Json(response))
}

pub async fn clear_game(
	State(game_manager): State<Arc<GameManager>>,
	Path(code): Path<String>,
) -> Result<Json<serde_json::Value>> {
	game_manager.clear_game(&code)?;
	
	Ok(Json(serde_json::json!({
		"message": "牌局清理成功",
		"code": code
	})))
}
