use axum::{
    Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        response::{ApiResponse, ApiResult},
        state::AppState,
    },
    models::language::*,
};

// Request DTOs
#[derive(Debug, Deserialize)]
pub struct CreateLanguageRequest {
    pub name: String,
    pub display_name: String,
    pub file_extension: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLanguageRequest {
    pub display_name: Option<String>,
    pub file_extension: Option<String>,
}

// Response DTOs
#[derive(Debug, Serialize)]
pub struct LanguageResponse {
    pub name: String,
    pub display_name: String,
    pub file_extension: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Language> for LanguageResponse {
    fn from(language: Language) -> Self {
        Self {
            name: language.name,
            display_name: language.display_name,
            file_extension: language.file_extension,
            created_at: language.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LanguageSummaryResponse {
    pub name: String,
    pub display_name: String,
    pub file_extension: String,
}

impl From<Language> for LanguageSummaryResponse {
    fn from(language: Language) -> Self {
        Self {
            name: language.name,
            display_name: language.display_name,
            file_extension: language.file_extension,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SupportedLanguagesResponse {
    pub languages: Vec<String>,
    pub count: usize,
}

pub async fn create_language(
    State(state): State<AppState>,
    Json(request): Json<CreateLanguageRequest>,
) -> ApiResult<LanguageResponse> {
    let create_request = crate::models::language::CreateLanguageRequest {
        name: request.name,
        display_name: request.display_name,
        file_extension: request.file_extension,
    };

    let language = state
        .language_service
        .create_language(create_request)
        .await?;
    let response = ApiResponse::success_with_message(
        language.into(),
        "Language created successfully".to_string(),
    );

    Ok(Json(response))
}

pub async fn get_language_by_name(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> ApiResult<LanguageResponse> {
    let language = state
        .language_service
        .get_language_by_name(&name)
        .await?
        .ok_or_else(|| crate::errors::Error::NotFound(format!("Language '{}' not found", name)))?;

    let response = ApiResponse::success(language.into());
    Ok(Json(response))
}

pub async fn list_languages(
    State(state): State<AppState>,
) -> ApiResult<Vec<LanguageSummaryResponse>> {
    let languages = state.language_service.get_all_languages().await?;

    let language_responses: Vec<LanguageSummaryResponse> =
        languages.into_iter().map(|lang| lang.into()).collect();

    let response = ApiResponse::success(language_responses);
    Ok(Json(response))
}

pub async fn update_language(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Json(request): Json<UpdateLanguageRequest>,
) -> ApiResult<LanguageResponse> {
    let update_request = crate::models::language::UpdateLanguageRequest {
        display_name: request.display_name,
        file_extension: request.file_extension,
    };

    let language = state
        .language_service
        .update_language(&name, update_request)
        .await?;
    let response = ApiResponse::success_with_message(
        language.into(),
        "Language updated successfully".to_string(),
    );

    Ok(Json(response))
}

pub async fn delete_language(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> ApiResult<()> {
    state.language_service.delete_language(&name).await?;
    let response =
        ApiResponse::success_message(format!("Language '{}' deleted successfully", name));
    Ok(Json(response))
}

pub async fn check_language_support(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> ApiResult<serde_json::Value> {
    let is_supported = state.language_service.is_language_supported(&name).await?;

    let response_data = serde_json::json!({
        "language": name,
        "supported": is_supported
    });

    let response = ApiResponse::success(response_data);
    Ok(Json(response))
}

pub async fn get_supported_languages(
    State(state): State<AppState>,
) -> ApiResult<SupportedLanguagesResponse> {
    let languages = state.language_service.get_supported_languages().await?;

    let response_data = SupportedLanguagesResponse {
        count: languages.len(),
        languages,
    };

    let response = ApiResponse::success(response_data);
    Ok(Json(response))
}
