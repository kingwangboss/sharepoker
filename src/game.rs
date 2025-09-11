use crate::error::{AppError, Result};
use crate::models::{Game, Player, UploadLogRequest, UploadLogResponse, LogEntry};
use chrono::Utc;
use dashmap::DashMap;
use std::sync::Arc;

pub struct GameManager {
	games: Arc<DashMap<String, Game>>, // game_code -> Game
	logs: Arc<DashMap<String, Vec<LogEntry>>>, // key -> logs
}

impl GameManager {
	pub fn new() -> Self {
		Self {
			games: Arc::new(DashMap::new()),
			logs: Arc::new(DashMap::new()),
		}
	}

	pub fn upload_player_image(&self, game_code: String, username: String, device_id: String, image: String) -> Result<Game> {
		// 如果牌局不存在，创建新牌局
		if !self.games.contains_key(&game_code) {
			let game = Game {
				code: game_code.clone(),
				players: Vec::new(),
				created_at: Utc::now(),
			};
			self.games.insert(game_code.clone(), game);
		}

		let mut game_entry = self.games.get_mut(&game_code)
			.ok_or(AppError::GameNotFound)?;

		// 首先检查是否有相同设备ID的玩家，如果有则更新该玩家的用户名和手牌
		if let Some(player) = game_entry.players.iter_mut().find(|p| p.device_id == device_id) {
			player.username = username; // 覆盖玩家名称
			player.hand_image = Some(image);
			player.uploaded_at = Utc::now();
		} else {
			// 如果没有找到相同设备ID的玩家，添加新玩家
			let new_player = Player {
				username,
				device_id,
				hand_image: Some(image),
				uploaded_at: Utc::now(),
			};
			game_entry.players.push(new_player);
		}

		let updated_game = game_entry.clone();
		Ok(updated_game)
	}

	pub fn get_game(&self, game_code: &str) -> Result<Game> {
		self.games.get(game_code)
			.map(|entry| entry.clone())
			.ok_or(AppError::GameNotFound)
	}

	pub fn clear_game(&self, game_code: &str) -> Result<()> {
		if self.games.contains_key(game_code) {
			self.games.remove(game_code);
			Ok(())
		} else {
			Err(AppError::GameNotFound)
		}
	}

	pub fn upload_log(&self, req: UploadLogRequest) -> Result<UploadLogResponse> {
		let key = req.game_code.clone().unwrap_or_else(|| format!("device:{}", req.device_id.clone()));

		let entry = LogEntry {
			device_id: req.device_id,
			message: req.message,
			game_code: req.game_code,
			username: req.username,
			level: req.level,
			created_at: Utc::now(),
		};

		self.logs
			.entry(key)
			.and_modify(|v| v.push(entry.clone()))
			.or_insert_with(|| vec![entry]);

		Ok(UploadLogResponse {
			status: "ok".to_string(),
			stored_at: Utc::now(),
		})
	}

	pub fn get_logs(&self, game_code: Option<String>, device_id: Option<String>, limit: Option<usize>) -> Result<Vec<LogEntry>> {
		let mut all: Vec<LogEntry> = Vec::new();

		if let Some(code) = game_code {
			if let Some(v) = self.logs.get(&code) { all.extend(v.value().clone()); }
		}

		if let Some(dev) = device_id {
			let key = format!("device:{}", dev);
			if let Some(v) = self.logs.get(&key) { all.extend(v.value().clone()); }
		}

		// 如果未提供任何过滤条件，聚合所有日志
		if all.is_empty() {
			for item in self.logs.iter() { all.extend(item.value().clone()); }
		}

		// 按时间排序，最新在前
		all.sort_by(|a, b| b.created_at.cmp(&a.created_at));

		// 截断到limit
		if let Some(l) = limit { all.truncate(l); }

		Ok(all)
	}
}

// 添加rand依赖的简单实现
mod rand {
	use std::collections::hash_map::DefaultHasher;
	use std::hash::{Hash, Hasher};
	use std::time::{SystemTime, UNIX_EPOCH};

	pub fn random<T>() -> T 
	where 
		T: From<u64>,
	{
		let mut hasher = DefaultHasher::new();
		SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_nanos()
			.hash(&mut hasher);
		T::from(hasher.finish())
	}
}
