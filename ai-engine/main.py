import torch
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from sentence_transformers import CrossEncoder, SentenceTransformer

app = FastAPI()

# GPUが使えるならcuda、だめならcpu
device = "cuda" if torch.cuda.is_available() else "cpu"
print(f"Loading models on: {device}")

# モデルのロード (起動時に1回だけ行う)
# ※ダウンロードに時間がかかるため、初回起動は遅くなります
print("Loading Embedding Model...")
embed_model = SentenceTransformer("intfloat/multilingual-e5-large", device=device)

print("Loading Re-ranking Model...")
rerank_model = CrossEncoder(
    "hotchpotch/japanese-reranker-cross-encoder-large-v1", device=device
)

print("Models loaded successfully!")

# --- リクエストの型定義 ---


class EmbedRequest(BaseModel):
    text: str


class RerankRequest(BaseModel):
    query: str
    candidates: list[str]


# --- エンドポイント ---


@app.get("/health")
def health_check():
    return {"status": "ok", "device": device}


@app.post("/embed")
def create_embedding(req: EmbedRequest):
    """テキストを受け取り、ベクトル(List[float])を返す"""
    try:
        # e5モデルのお作法: クエリ以外は "passage: " をつける運用にするか、
        # あるいは呼び出し元(Rust)で制御するか。ここでは単純化のためそのまま通す。
        # 実際に使うときは Rust側で "passage: " を付与して送ることを推奨。
        vector = embed_model.encode(req.text, normalize_embeddings=True)
        return {"vector": vector.tolist()}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/rerank")
def rerank_candidates(req: RerankRequest):
    """クエリと候補リストを受け取り、スコア付きで返す"""
    try:
        if not req.candidates:
            return {"results": []}

        # ペアを作成 [ [query, doc1], [query, doc2], ... ]
        pairs = [[req.query, doc] for doc in req.candidates]

        # 採点
        scores = rerank_model.predict(pairs)

        # 結果を整形して返す (スコア順には並べ替えず、インデックス対応のまま返す)
        # Rust側でソートやフィルタリングをしやすくするため
        results = [
            {"index": i, "score": float(score), "candidate": doc}
            for i, (score, doc) in enumerate(zip(scores, req.candidates))
        ]

        return {"results": results}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
