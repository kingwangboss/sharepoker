use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("用户名已存在")]
    UserAlreadyExists,
    
    #[error("用户不存在")]
    UserNotFound,
    
    #[error("密码错误")]
    InvalidPassword,
    
    #[error("无效的token")]
    InvalidToken,
    
    #[error("牌局不存在")]
    GameNotFound,
    
    #[error("牌局代码无效")]
    InvalidGameCode,
    
    #[error("用户已在牌局中")]
    UserAlreadyInGame,
    
    #[error("内部服务器错误")]
    InternalServerError,
    
    #[error("认证失败")]
    Unauthorized,
    
    #[error("请求参数错误: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::UserAlreadyExists => (StatusCode::CONFLICT, self.to_string()),
            AppError::UserNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::InvalidPassword => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::GameNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::InvalidGameCode => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::UserAlreadyInGame => (StatusCode::CONFLICT, self.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
