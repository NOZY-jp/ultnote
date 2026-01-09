# UltNote

セマンティック検索を備えたメモアプリ。「クリニック」と検索すると「歯医者に行く予定」がヒットするような、意味的類似性に基づく検索機能を持つ。

## 技術スタック

| レイヤー | 技術 |
|----------|------|
| Frontend | SvelteKit (SSR) + SCSS |
| API | Rust (Axum) |
| Embedder | Python (FastAPI) + multilingual-e5-base |
| Vector DB | Qdrant |
| 認証 | Cloudflare Access (Google OAuth) |
| インフラ | Docker Compose + Cloudflare Tunnel |

## 開発環境

### 必要条件

- Docker & Docker Compose
- (オプション) Rust 1.92+, Node.js 20+, Python 3.11+ (ローカル開発用)

### 起動手順

```bash
# 1. リポジトリをクローン
git clone https://github.com/your-username/ultnote.git
cd ultnote

# 2. Docker Composeで起動
docker compose up -d

# 3. ビルドを待つ (初回は5-10分程度)
docker compose logs -f

# 4. アクセス
open http://localhost
```

### 動作確認

```bash
# APIヘルスチェック
curl http://localhost/api/health
# => {"status":"ok","version":"0.1.0"}

# Nginxヘルスチェック
curl http://localhost/nginx-health
# => OK

# デモメモ作成
curl -X POST http://localhost/api/demo/memo \
  -H "Content-Type: application/json" \
  -d '{"content": "歯医者に行く予定がある", "type": "permanent", "tags": ["health"]}'

# セマンティック検索テスト
curl -X POST http://localhost/api/demo/search \
  -H "Content-Type: application/json" \
  -d '{"query": "クリニック"}'
# => "歯医者に行く予定がある" がヒット (score: ~0.84)
```

### デモデータ投入

```bash
./scripts/seed-demo.sh
```

### 停止

```bash
docker compose down
```

### ローカル開発 (ホットリロード)

各サービスを個別に開発する場合:

```bash
# Frontend
cd frontend
npm install
npm run dev

# API (別ターミナル)
cd api
cargo watch -x run

# Embedder (別ターミナル)  
cd embedder
pip install -r requirements.txt
uvicorn main:app --reload --port 8000
```

## 本番環境デプロイ

### 前提条件

1. Proxmox VM または Docker が動作するサーバー
2. Cloudflare アカウント
3. ドメイン (ultnote.com など)

### 1. Cloudflare Access 設定

1. Cloudflare Zero Trust ダッシュボードにログイン
2. Access > Applications で新規アプリケーション作成
3. Self-hosted を選択
4. ドメインを設定 (例: ultnote.com)
5. Policy を作成 (Google OAuth など)
6. 以下の値をメモ:
   - **Team Domain**: `your-team.cloudflareaccess.com`
   - **Application Audience (AUD)**: Application Settings で確認

### 2. Cloudflare Tunnel 設定

```bash
# cloudflared をインストール (サーバー上で)
curl -L https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64 -o /usr/local/bin/cloudflared
chmod +x /usr/local/bin/cloudflared

# トンネル作成
cloudflared tunnel login
cloudflared tunnel create ultnote

# トンネルトークン取得
cloudflared tunnel token ultnote
# => このトークンを .env に設定
```

### 3. 環境変数設定

```bash
cp .env.example .env
vim .env
```

```env
CF_TEAM_DOMAIN=your-team.cloudflareaccess.com
CF_POLICY_AUD=your-application-audience-tag
CLOUDFLARE_TUNNEL_TOKEN=your-tunnel-token
```

### 4. DNS 設定

Cloudflare DNS で CNAME レコードを追加:

| Type | Name | Content |
|------|------|---------|
| CNAME | ultnote.com | {tunnel-id}.cfargotunnel.com |

### 5. デプロイ実行

```bash
./scripts/deploy.sh
```

### 6. デモデータ投入

```bash
./scripts/seed-demo.sh https://ultnote.com/api
```

### 運用コマンド

```bash
# ログ確認
docker compose -f docker-compose.prod.yml logs -f

# 再起動
docker compose -f docker-compose.prod.yml restart

# 停止
docker compose -f docker-compose.prod.yml down

# 更新デプロイ
git pull
docker compose -f docker-compose.prod.yml build
docker compose -f docker-compose.prod.yml up -d
```

## アーキテクチャ

詳細は [docs/architecture.md](docs/architecture.md) を参照。

```
┌─────────────────────────────────────────────────────┐
│  Cloudflare Tunnel                                  │
└────────────────────────┬────────────────────────────┘
                         ▼
┌─────────────────────────────────────────────────────┐
│  Docker Compose                                     │
│  ┌───────────────────────────────────────────────┐  │
│  │  Nginx (port 80)                              │  │
│  │  - /api/* → API (Axum)                        │  │
│  │  - /* → Frontend (SvelteKit)                  │  │
│  └───────────────────────────────────────────────┘  │
│           │                    │                    │
│           ▼                    ▼                    │
│  ┌─────────────┐      ┌─────────────┐              │
│  │  Frontend   │      │    API      │              │
│  │  (SvelteKit)│      │   (Axum)    │              │
│  │  :3000      │      │   :8080     │              │
│  └─────────────┘      └──────┬──────┘              │
│                              │                      │
│                    ┌─────────┴─────────┐           │
│                    ▼                   ▼           │
│           ┌─────────────┐     ┌─────────────┐      │
│           │  Embedder   │     │   Qdrant    │      │
│           │  (Python)   │     │ (VectorDB)  │      │
│           │  :8000      │     │  :6334      │      │
│           └─────────────┘     └─────────────┘      │
└─────────────────────────────────────────────────────┘
```

## API エンドポイント

### 認証必要

| Method | Path | 説明 |
|--------|------|------|
| POST | /memo | メモ作成 |
| GET | /memo/{id} | メモ取得 |
| PUT | /memo/{id} | メモ更新 |
| DELETE | /memo/{id} | メモ削除 |
| POST | /search | 検索 (認証時は memos コレクション) |
| GET | /tags | タグ一覧 |

### 認証不要 (デモ用)

| Method | Path | 説明 |
|--------|------|------|
| POST | /demo/memo | デモメモ作成 |
| GET | /demo/memo/{id} | デモメモ取得 |
| POST | /demo/search | デモ検索 (memos_demo コレクション) |
| GET | /demo/tags | デモタグ一覧 |
| GET | /health | ヘルスチェック |

## ライセンス

MIT
