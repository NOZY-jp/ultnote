#!/bin/bash
set -e

API_URL="${1:-http://localhost/api}"

echo "=== Seeding Demo Data ==="

curl -s -X POST "$API_URL/demo/memo" \
  -H "Content-Type: application/json" \
  -d '{"content": "歯医者に行く予定がある", "type": "permanent", "tags": ["health", "appointment"]}' > /dev/null

curl -s -X POST "$API_URL/demo/memo" \
  -H "Content-Type: application/json" \
  -d '{"content": "会議の資料を準備する", "type": "flash", "tags": ["work"], "from": "2026-01-10"}' > /dev/null

curl -s -X POST "$API_URL/demo/memo" \
  -H "Content-Type: application/json" \
  -d '{"content": "プロジェクトの設計書をレビューする", "type": "permanent", "tags": ["work", "review"]}' > /dev/null

curl -s -X POST "$API_URL/demo/memo" \
  -H "Content-Type: application/json" \
  -d '{"content": "買い物リスト: 牛乳、卵、パン", "type": "flash", "tags": ["shopping"]}' > /dev/null

curl -s -X POST "$API_URL/demo/memo" \
  -H "Content-Type: application/json" \
  -d '{"content": "新しいプログラミング言語を勉強したい", "type": "permanent", "tags": ["learning", "programming"]}' > /dev/null

curl -s -X POST "$API_URL/demo/memo" \
  -H "Content-Type: application/json" \
  -d '{"content": "週末に映画を見に行く", "type": "flash", "tags": ["entertainment"], "from": "2026-01-11"}' > /dev/null

curl -s -X POST "$API_URL/demo/memo" \
  -H "Content-Type: application/json" \
  -d '{"content": "UltNoteへようこそ！このアプリはセマンティック検索でメモを見つけられます", "type": "permanent", "tags": ["ultnote", "welcome"]}' > /dev/null

curl -s -X POST "$API_URL/demo/memo" \
  -H "Content-Type: application/json" \
  -d '{"content": "クリニックと検索すると歯医者のメモがヒットします", "type": "permanent", "tags": ["ultnote", "tips"]}' > /dev/null

echo "Demo data seeded successfully!"
echo ""
echo "Try searching for:"
echo "  - 'クリニック' -> finds dental appointment"
echo "  - '仕事のドキュメント' -> finds work-related memos"
echo "  - '買い物' -> finds shopping list"
