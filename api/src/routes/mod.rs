mod health;
mod memo;
mod search;
mod tags;

use axum::Router;

use crate::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .merge(health::routes())
        .merge(memo::routes())
        .merge(search::routes())
        .merge(tags::routes())
}
