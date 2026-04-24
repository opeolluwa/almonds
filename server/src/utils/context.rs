use axum::http::HeaderMap;
use sea_orm::DatabaseConnection;
use seaography::async_graphql::Context;

use crate::errors::app_error::AppError;

pub struct RequestContext<'a> {
    pub db_conn: &'a DatabaseConnection,
    pub api_key: &'a str,
}

pub fn extract_request_context<'a>(ctx: &'a Context<'_>) -> Result<RequestContext<'a>, AppError> {
    let db_conn = ctx
        .data::<DatabaseConnection>()
        .map_err(|err| AppError::InternalError(err.message))?;

    let headers = ctx
        .data::<HeaderMap>()
        .map_err(|_| AppError::InternalError("Missing request headers".to_string()))?;

    let api_key = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::InternalError("Missing Authorization header".to_string()))?;

    Ok(RequestContext { db_conn, api_key })
}

pub fn extract_db_conn<'a>(ctx: &'a Context<'_>) -> Result<&'a DatabaseConnection, AppError> {
    ctx.data::<DatabaseConnection>()
        .map_err(|err| AppError::InternalError(err.message))
}
