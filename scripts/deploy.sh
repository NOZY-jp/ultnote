#!/bin/bash
set -e

echo "=== UltNote Production Deployment ==="

if [ ! -f .env ]; then
    echo "Error: .env file not found. Copy .env.example to .env and fill in values."
    exit 1
fi

source .env

if [ -z "$CLOUDFLARE_TUNNEL_TOKEN" ]; then
    echo "Error: CLOUDFLARE_TUNNEL_TOKEN not set in .env"
    exit 1
fi

echo "Building images..."
docker compose -f docker-compose.prod.yml build

echo "Starting services..."
docker compose -f docker-compose.prod.yml up -d

echo "Waiting for services to be ready..."
sleep 10

echo "Checking health..."
docker compose -f docker-compose.prod.yml ps

echo "Testing API health..."
docker exec ultnote-nginx curl -s http://api:8080/health || echo "API not ready yet"

echo ""
echo "=== Deployment complete ==="
echo "Access via: https://ultnote.com"
echo ""
echo "To view logs: docker compose -f docker-compose.prod.yml logs -f"
echo "To stop: docker compose -f docker-compose.prod.yml down"
