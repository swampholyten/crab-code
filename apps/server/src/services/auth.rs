use crate::errors::Result;
use async_trait::async_trait;

use crate::models::auth::{AuthResponse, Claims, LoginRequest, RegisterRequest};

#[async_trait]
pub trait AuthServiceTrait: Send + Sync {
    async fn register(&self, request: RegisterRequest) -> Result<AuthResponse>;
    async fn login(&self, request: LoginRequest) -> Result<AuthResponse>;
    async fn validate_token(&self, token: &str) -> Result<Claims>;
    async fn refresh_token(&self, refresh_token: &str) -> Result<AuthResponse>;
}
