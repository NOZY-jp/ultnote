use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use eyre::{Result, WrapErr}; // eyreをインポート
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// --- 型定義 ---

#[derive(Deserialize)]
struct CreateNoteRequest {
    content: String,
}

#[derive(Serialize)]
struct EmbedRequest {
    text: String,
}

#[derive(Deserialize)]
struct EmbedResponse {
    vector: Vec<f32>,
}

// --- ロジック (eyre::Resultを返す部分) ---

// ここでエラーが起きたら eyre がリッチなレポートを作ってくれます
async fn generate_embedding(text: &str) -> Result<Vec<f32>> {
    let client = reqwest::Client::new();

    let res = client
        .post("http://localhost:5000/embed")
        .json(&EmbedRequest {
            text: format!("passage: {}", text),
        })
        .send()
        .await
        .wrap_err("Failed to connect to Python AI Engine")?; // コンテキスト追加

    if !res.status().is_success() {
        // ステータスコードが200系以外ならエラーにする
        let status = res.status();
        let error_text = res.text().await.unwrap_or_default();
        return Err(eyre::eyre!(
            "AI Engine returned error: {} - {}",
            status,
            error_text
        ));
    }

    let data = res
        .json::<EmbedResponse>()
        .await
        .wrap_err("Failed to parse JSON response from AI Engine")?; // コンテキスト追加

    Ok(data.vector)
}

// --- ハンドラー (HTTPレスポンスへの変換) ---

async fn health_check() -> &'static str {
    "Core API is running!"
}

async fn register_note(Json(payload): Json<CreateNoteRequest>) -> impl IntoResponse {
    println!("Received content: {}", payload.content);

    // ロジックを呼び出し
    match generate_embedding(&payload.content).await {
        Ok(vector) => {
            println!("Vector generated successfully. Dim: {}", vector.len());
            // 成功時のレスポンス
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "status": "success",
                    "message": "Vectorized successfully",
                    "vector_preview": &vector[0..5]
                })),
            )
        }
        Err(report) => {
            // エラー時のレスポンス
            // サーバーのログには詳細なスタックトレースを表示 (color-eyreのおかげで見やすい)
            eprintln!("{:?}", report);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "status": "error",
                    "message": report.to_string() // クライアントには簡潔なメッセージを返す
                })),
            )
        }
    }
}

// --- メイン関数 ---

#[tokio::main]
async fn main() -> Result<()> {
    // color-eyreの初期化 (パニック時やエラー時の表示をきれいにする)
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(health_check))
        .route("/register", post(register_note));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Core API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .wrap_err("Failed to bind port 3000")?;
    axum::serve(listener, app).await.wrap_err("Server error")?;

    Ok(())
}
