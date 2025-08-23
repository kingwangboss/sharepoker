use crate::error::{AppError, Result};
use crate::models::{Game, Player};
use chrono::Utc;
use dashmap::DashMap;
use std::sync::Arc;

pub struct GameManager {
	games: Arc<DashMap<String, Game>>, // game_code -> Game
}

impl GameManager {
	pub fn new() -> Self {
		Self {
			games: Arc::new(DashMap::new()),
		}
	}

	pub fn upload_player_image(&self, game_code: String, username: String, image: String) -> Result<Game> {
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

		// 检查用户是否已在游戏中，如果是则更新手牌
		if let Some(player) = game_entry.players.iter_mut().find(|p| p.username == username) {
			player.hand_image = Some(image);
			player.uploaded_at = Utc::now();
		} else {
			// 添加新玩家
			let new_player = Player {
				username,
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
