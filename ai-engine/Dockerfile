FROM python:3.11-slim

WORKDIR /app

# 必要なシステムパッケージ (ビルド用)
RUN apt-get update && apt-get install -y \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# 依存ライブラリのインストール
# キャッシュを効かせるため先に実行
RUN pip install --no-cache-dir \
    fastapi \
    uvicorn[standard] \
    sentence-transformers \
    torch \
    fugashi \
    unidic-lite \
    protobuf

# ソースコードは docker-compose の volume でマウントするので
# ここでの COPY は最小限でOK
COPY . .

CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
