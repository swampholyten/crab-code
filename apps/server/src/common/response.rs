use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

/// Standard API response wrapper for all endpoints
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    /// Indicates if the request was successful
    pub success: bool,
    /// The actual data payload (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// Error message (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Error code for client-side handling (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Additional metadata (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ResponseMeta>,
}

/// Metadata for API responses (pagination, counts, etc.)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMeta {
    /// Total count for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Current page (1-based)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// Items per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
    /// Total pages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i32>,
    /// Has next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_next: Option<bool>,
    /// Has previous page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_prev: Option<bool>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    /// Create a successful response with data
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            error_code: None,
            meta: None,
        }
    }

    /// Create a successful response with data and metadata
    pub fn success_with_meta(data: T, meta: ResponseMeta) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            error_code: None,
            meta: Some(meta),
        }
    }

    /// Create a successful response with custom message
    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message),
            error_code: None,
            meta: None,
        }
    }
}

impl ApiResponse<()> {
    /// Create a successful response without data
    pub fn success_empty() -> Self {
        Self {
            success: true,
            data: None,
            message: None,
            error_code: None,
            meta: None,
        }
    }

    /// Create a successful response with just a message
    pub fn success_message(message: String) -> Self {
        Self {
            success: true,
            data: None,
            message: Some(message),
            error_code: None,
            meta: None,
        }
    }

    /// Create an error response
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
            error_code: None,
            meta: None,
        }
    }

    /// Create an error response with error code
    pub fn error_with_code(message: String, error_code: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
            error_code: Some(error_code),
            meta: None,
        }
    }
}

/// Type alias for convenient usage
pub type ApiResult<T> = Result<ApiResponse<T>, crate::errors::Error>;

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status_code = if self.success {
            StatusCode::OK
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };

        (status_code, Json(self)).into_response()
    }
}

/// Helper for paginated responses
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationInfo {
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: i64, page: i32, per_page: i32) -> Self {
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;
        let has_next = page < total_pages;
        let has_prev = page > 1;

        Self {
            items,
            pagination: PaginationInfo {
                total,
                page,
                per_page,
                total_pages,
                has_next,
                has_prev,
            },
        }
    }
}
