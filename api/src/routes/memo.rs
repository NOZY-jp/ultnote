use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::Utc;
use uuid::Uuid;

use crate::auth::AuthState;
use crate::error::{AppError, Result};
use crate::models::{CreateMemoRequest, Memo, MemoResponse, UpdateMemoRequest};
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/memo", post(create_memo))
        .route("/memo/{id}", get(get_memo))
        .route("/memo/{id}", put(update_memo))
        .route("/memo/{id}", delete(delete_memo))
        // Demo routes (no auth required, use memos_demo collection)
        .route("/demo/memo", post(create_demo_memo))
        .route("/demo/memo/{id}", get(get_demo_memo))
}

fn require_auth(auth: &AuthState) -> Result<()> {
    if !auth.is_authenticated {
        return Err(AppError::Unauthorized("Authentication required".into()));
    }
    Ok(())
}

async fn create_memo(
    State(state): State<AppState>,
    auth: AuthState,
    Json(req): Json<CreateMemoRequest>,
) -> Result<Json<MemoResponse>> {
    require_auth(&auth)?;

    let now = Utc::now();
    let id = Uuid::new_v4();

    let memo = Memo {
        id,
        content: req.content.clone(),
        memo_type: req.memo_type,
        from: req.from,
        until: req.until,
        tags: req.tags,
        date_added: now,
        access_count: 0,
        last_accessed: now,
        completed: false,
    };

    let vector = state.embedder.embed_for_storage(&req.content).await?;
    state.qdrant.insert_memo(&memo, vector, false).await?;

    Ok(Json(memo.into()))
}

async fn get_memo(
    State(state): State<AppState>,
    auth: AuthState,
    Path(id): Path<Uuid>,
) -> Result<Json<MemoResponse>> {
    require_auth(&auth)?;

    let memo = state
        .qdrant
        .get_memo(id, false)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Memo {} not found", id)))?;

    Ok(Json(memo.into()))
}

async fn update_memo(
    State(state): State<AppState>,
    auth: AuthState,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateMemoRequest>,
) -> Result<Json<MemoResponse>> {
    require_auth(&auth)?;

    let mut memo = state
        .qdrant
        .get_memo(id, false)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Memo {} not found", id)))?;

    let content_changed = req.content.is_some() && req.content.as_ref() != Some(&memo.content);

    if let Some(content) = req.content {
        memo.content = content;
    }
    if let Some(memo_type) = req.memo_type {
        memo.memo_type = memo_type;
    }
    if let Some(from) = req.from {
        memo.from = Some(from);
    }
    if let Some(until) = req.until {
        memo.until = Some(until);
    }
    if let Some(tags) = req.tags {
        memo.tags = tags;
    }
    if let Some(completed) = req.completed {
        memo.completed = completed;
    }

    memo.last_accessed = Utc::now();

    let vector = if content_changed {
        Some(state.embedder.embed_for_storage(&memo.content).await?)
    } else {
        None
    };

    state.qdrant.update_memo(&memo, vector, false).await?;

    Ok(Json(memo.into()))
}

async fn delete_memo(
    State(state): State<AppState>,
    auth: AuthState,
    Path(id): Path<Uuid>,
) -> Result<Json<()>> {
    require_auth(&auth)?;

    state.qdrant.delete_memo(id, false).await?;
    Ok(Json(()))
}

async fn create_demo_memo(
    State(state): State<AppState>,
    Json(req): Json<CreateMemoRequest>,
) -> Result<Json<MemoResponse>> {
    let now = Utc::now();
    let id = Uuid::new_v4();

    let memo = Memo {
        id,
        content: req.content.clone(),
        memo_type: req.memo_type,
        from: req.from,
        until: req.until,
        tags: req.tags,
        date_added: now,
        access_count: 0,
        last_accessed: now,
        completed: false,
    };

    let vector = state.embedder.embed_for_storage(&req.content).await?;
    state.qdrant.insert_memo(&memo, vector, true).await?;

    Ok(Json(memo.into()))
}

async fn get_demo_memo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<MemoResponse>> {
    let memo = state
        .qdrant
        .get_memo(id, true)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Memo {} not found", id)))?;

    Ok(Json(memo.into()))
}
