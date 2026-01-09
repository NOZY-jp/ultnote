use axum::{extract::State, routing::post, Json, Router};

use crate::auth::AuthState;
use crate::error::Result;
use crate::models::{SearchRequest, SearchResponse};
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/search", post(search))
        .route("/demo/search", post(demo_search))
}

async fn search(
    State(state): State<AppState>,
    auth: AuthState,
    Json(req): Json<SearchRequest>,
) -> Result<Json<SearchResponse>> {
    let is_demo = !auth.is_authenticated;
    
    let vector = state.embedder.embed_for_search(&req.query).await?;
    let results = state
        .qdrant
        .search(vector, &req.filters, req.limit, is_demo)
        .await?;

    Ok(Json(SearchResponse {
        total: results.len(),
        results,
    }))
}

async fn demo_search(
    State(state): State<AppState>,
    Json(req): Json<SearchRequest>,
) -> Result<Json<SearchResponse>> {
    let vector = state.embedder.embed_for_search(&req.query).await?;
    let results = state
        .qdrant
        .search(vector, &req.filters, req.limit, true)
        .await?;

    Ok(Json(SearchResponse {
        total: results.len(),
        results,
    }))
}
