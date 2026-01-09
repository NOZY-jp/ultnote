use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;

use crate::auth::AuthState;
use crate::error::Result;
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/tags", get(list_tags))
        .route("/demo/tags", get(list_demo_tags))
}

#[derive(Debug, Serialize)]
pub struct TagNode {
    pub name: String,
    pub path: String,
    pub children: Vec<TagNode>,
}

#[derive(Debug, Serialize)]
pub struct TagsResponse {
    pub tags: Vec<TagNode>,
}

async fn list_tags(State(state): State<AppState>, auth: AuthState) -> Result<Json<TagsResponse>> {
    let is_demo = !auth.is_authenticated;
    let tags = state.qdrant.get_all_tags(is_demo).await?;
    let tree = build_tag_tree(tags);
    Ok(Json(TagsResponse { tags: tree }))
}

async fn list_demo_tags(State(state): State<AppState>) -> Result<Json<TagsResponse>> {
    let tags = state.qdrant.get_all_tags(true).await?;
    let tree = build_tag_tree(tags);
    Ok(Json(TagsResponse { tags: tree }))
}

fn build_tag_tree(tags: Vec<String>) -> Vec<TagNode> {
    use std::collections::HashMap;

    let mut root: HashMap<String, TagNode> = HashMap::new();

    for tag in tags {
        let parts: Vec<&str> = tag.split('/').collect();
        insert_tag_path(&mut root, &parts, 0, &tag);
    }

    let mut result: Vec<TagNode> = root.into_values().collect();
    result.sort_by(|a, b| a.name.cmp(&b.name));
    result
}

fn insert_tag_path(
    nodes: &mut std::collections::HashMap<String, TagNode>,
    parts: &[&str],
    depth: usize,
    full_path: &str,
) {
    if depth >= parts.len() {
        return;
    }

    let name = parts[depth].to_string();
    let path_so_far = parts[..=depth].join("/");

    let node = nodes.entry(name.clone()).or_insert_with(|| TagNode {
        name: name.clone(),
        path: path_so_far.clone(),
        children: Vec::new(),
    });

    if depth + 1 < parts.len() {
        let mut child_map: std::collections::HashMap<String, TagNode> = node
            .children
            .drain(..)
            .map(|c| (c.name.clone(), c))
            .collect();

        insert_tag_path(&mut child_map, parts, depth + 1, full_path);

        node.children = child_map.into_values().collect();
        node.children.sort_by(|a, b| a.name.cmp(&b.name));
    }
}
