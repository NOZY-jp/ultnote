# UltNote 開発進捗

## 現在のフェーズ

**Phase 2: バックエンド実装** 🚧 進行中

## 進捗サマリー

| フェーズ | 状態 | 開始日 | 完了日 |
|---------|------|--------|--------|
| Phase 0: 設計・計画 | ✅ 完了 | 2026-01-09 | 2026-01-09 |
| Phase 1: インフラ構築 | ✅ 完了 | 2026-01-09 | 2026-01-09 |
| Phase 2: バックエンド実装 | 🚧 進行中 | 2026-01-09 | - |
| Phase 3: フロントエンド実装 | ⏳ 未着手 | - | - |
| Phase 4: 統合・テスト | ⏳ 未着手 | - | - |
| Phase 5: デプロイ・公開 | ⏳ 未着手 | - | - |

---

## Phase 0: 設計・計画 ✅

### 完了タスク

- [x] 要件ヒアリング・インタビュー
- [x] 機能仕様の決定
- [x] 技術スタックの選定
- [x] データモデル設計
- [x] API設計
- [x] インフラ構成設計
- [x] architecture.md 作成

### 成果物

- `docs/architecture.md` - アーキテクチャ設計書
- `docs/progress.md` - 進捗管理

---

## Phase 1: インフラ構築 ✅

### 完了タスク

- [x] ディレクトリ構成の整理（ai-engine → embedder, core-api → api）
- [x] docker-compose.yml の更新（PostgreSQL削除、Nginx/SvelteKit追加）
- [x] Nginx設定ファイル作成（nginx/nginx.conf）
- [x] 各サービスのDockerfile作成
  - api/Dockerfile (Rust multi-stage build)
  - embedder/Dockerfile (Python)
  - frontend/Dockerfile (Node.js)
- [ ] Qdrantコレクション初期化スクリプト（APIで自動作成に変更）
- [ ] ローカル開発環境の動作確認

### 成果物

- `docker-compose.yml` - Docker Compose設定
- `nginx/nginx.conf` - Nginx設定
- 各サービスのDockerfile

---

## Phase 2: バックエンド実装 🚧

### Embedder (Python) ✅

- [x] multilingual-e5-base への変更
- [x] /embed エンドポイント
- [x] /health エンドポイント

### API (Rust/Axum) ✅

- [x] プロジェクト構成（Cargo.toml修正、モジュール分割）
- [x] Qdrantクライアント実装（services/qdrant.rs）
- [x] メモCRUD実装（routes/memo.rs）
- [x] 検索実装（routes/search.rs）
- [x] タグ管理実装（routes/tags.rs）
- [ ] JWT検証実装（Cloudflare Access）
- [ ] デモモード対応

### APIエンドポイント

| Endpoint | Method | Status | Description |
|----------|--------|--------|-------------|
| `/health` | GET | ✅ | ヘルスチェック |
| `/memo` | POST | ✅ | メモ追加 |
| `/memo/{id}` | GET | ✅ | メモ取得 |
| `/memo/{id}` | PUT | ✅ | メモ更新 |
| `/memo/{id}` | DELETE | ✅ | メモ削除 |
| `/search` | POST | ✅ | 認証済みユーザー検索 |
| `/demo/search` | POST | ✅ | デモユーザー検索 |
| `/tags` | GET | ✅ | タグ一覧（ツリー形式） |

### ファイル構成

```
api/
├── Cargo.toml
├── Dockerfile
└── src/
    ├── main.rs          # エントリーポイント
    ├── config.rs        # 設定管理
    ├── error.rs         # エラー型定義
    ├── models/
    │   ├── mod.rs
    │   └── memo.rs      # メモ関連の型定義
    ├── routes/
    │   ├── mod.rs
    │   ├── health.rs    # ヘルスチェック
    │   ├── memo.rs      # メモCRUD
    │   ├── search.rs    # 検索API
    │   └── tags.rs      # タグ管理
    └── services/
        ├── mod.rs
        ├── embedder.rs  # Embedder HTTPクライアント
        └── qdrant.rs    # Qdrantサービス
```

---

## Phase 3: フロントエンド実装（予定）

### SvelteKit

- [x] プロジェクト初期化（基本ファイル作成済み）
- [ ] SCSS設定
- [ ] レイアウト・共通コンポーネント
- [ ] メイン画面（検索・一覧）
- [ ] メモ追加画面
- [ ] タグ入力UI
- [ ] 日付フィルタUI
- [ ] レスポンシブ対応
- [ ] デモモード対応

---

## Phase 4: 統合・テスト（予定）

- [ ] E2Eテスト
- [ ] パフォーマンステスト
- [ ] セキュリティレビュー

---

## Phase 5: デプロイ・公開（予定）

- [ ] Proxmox VMセットアップ
- [ ] Cloudflare Tunnel設定
- [ ] Cloudflare Access設定
- [ ] デモデータ投入
- [ ] 本番稼働確認

---

## 変更履歴

| 日付 | 変更内容 |
|------|----------|
| 2026-01-09 | 初版作成。Phase 0完了。 |
| 2026-01-09 | Phase 1インフラ構築完了。Phase 2バックエンド実装開始。 |
| 2026-01-09 | API (Rust) 基本実装完了。メモCRUD、検索、タグ管理。 |
