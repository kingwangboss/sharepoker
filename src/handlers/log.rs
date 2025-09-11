use crate::error::Result;
use crate::game::GameManager;
use crate::models::{UploadLogRequest, UploadLogResponse};
use axum::{
	extract::{State, Query},
	Json,
};
use crate::models::{LogsQuery, LogsResponse};
use std::sync::Arc;

pub async fn upload_log(
	State(game_manager): State<Arc<GameManager>>,
	Json(payload): Json<UploadLogRequest>,
) -> Result<Json<UploadLogResponse>> {
	let resp = game_manager.upload_log(payload)?;
	Ok(Json(resp))
}

pub async fn get_logs(
	State(game_manager): State<Arc<GameManager>>,
	Query(params): Query<LogsQuery>,
) -> Result<Json<LogsResponse>> {
	let logs = game_manager.get_logs(params.game_code, params.device_id, params.limit)?;
	Ok(Json(LogsResponse { logs }))
} 