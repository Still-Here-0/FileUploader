use axum::http::StatusCode;

pub async fn api_scream() -> StatusCode {
    StatusCode::FORBIDDEN
}
