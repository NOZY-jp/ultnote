import torch
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from sentence_transformers import SentenceTransformer

app = FastAPI(
    title="UltNote Embedder",
    description="Text embedding service using multilingual-e5-base",
    version="1.0.0",
)

# GPU が使えるなら cuda、だめなら cpu
device = "cuda" if torch.cuda.is_available() else "cpu"
print(f"Loading model on: {device}")

# モデルのロード (起動時に1回だけ行う)
# multilingual-e5-base: 278M params, 768次元
print("Loading Embedding Model: multilingual-e5-base...")
embed_model = SentenceTransformer("intfloat/multilingual-e5-base", device=device)
print("Model loaded successfully!")


# --- リクエスト/レスポンスの型定義 ---


class EmbedRequest(BaseModel):
    """埋め込みリクエスト"""

    text: str


class EmbedResponse(BaseModel):
    """埋め込みレスポンス"""

    vector: list[float]
    dimension: int


class HealthResponse(BaseModel):
    """ヘルスチェックレスポンス"""

    status: str
    device: str
    model: str


# --- エンドポイント ---


@app.get("/health", response_model=HealthResponse)
def health_check() -> HealthResponse:
    """ヘルスチェック"""
    return HealthResponse(
        status="ok", device=device, model="intfloat/multilingual-e5-base"
    )


@app.post("/embed", response_model=EmbedResponse)
def create_embedding(req: EmbedRequest) -> EmbedResponse:
    """
    テキストを受け取り、768次元のベクトルを返す

    注意: multilingual-e5 モデルはプレフィックスが必要
    - 保存時: "passage: {content}"
    - 検索時: "query: {query}"

    呼び出し元（Rust API）でプレフィックスを付与すること
    """
    try:
        vector = embed_model.encode(req.text, normalize_embeddings=True)
        return EmbedResponse(vector=vector.tolist(), dimension=len(vector))
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
