use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
	pub username: String,
	pub device_id: String, // 设备唯一标识
	pub hand_image: Option<String>,
	pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
	pub suit: Suit,
	pub rank: Rank,
	pub image: Option<String>, // Base64编码的图片数据或图片URL
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Suit {
	Hearts,
	Diamonds,
	Clubs,
	Spades,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rank {
	Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
	Jack, Queen, King, Ace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
	pub code: String,
	pub players: Vec<Player>,
	pub created_at: DateTime<Utc>,
}

// API请求/响应结构
#[derive(Debug, Deserialize)]
pub struct UploadPlayerRequest {
	pub game_code: String,
	pub username: String,
	pub device_id: String, // 设备唯一标识
	pub image: String,
}

#[derive(Debug, Serialize)]
pub struct GameResponse {
	pub code: String,
	pub players: Vec<PlayerInfo>,
	pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PlayerInfo {
	pub username: String,
	pub device_id: String, // 设备唯一标识
	pub hand_image: Option<String>,
	pub uploaded_at: DateTime<Utc>,
}
