#!/bin/bash

# Notion Killer - Start Script
# Usage: ./scripts/start.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

echo "=========================================="
echo "  Notion Killer - Starting Application"
echo "=========================================="
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if Colima/Docker is running
check_docker() {
    if ! docker info > /dev/null 2>&1; then
        echo -e "${YELLOW}Docker is not running. Starting Colima...${NC}"
        colima start --cpu 2 --memory 4 2>&1 | grep -E "(info|error)" || true
        sleep 2
    fi
    echo -e "${GREEN}✓ Docker is running${NC}"
}

# Start infrastructure
start_infrastructure() {
    echo ""
    echo "Starting infrastructure (PostgreSQL, Redis, MinIO)..."
    docker compose -f docker/docker-compose.yml up -d

    # Wait for services to be healthy
    echo "Waiting for services to be healthy..."
    sleep 5

    # Check health
    if docker compose -f docker/docker-compose.yml ps | grep -q "healthy"; then
        echo -e "${GREEN}✓ Infrastructure is healthy${NC}"
    else
        echo -e "${YELLOW}⚠ Some services may still be starting${NC}"
    fi
}

# Run migrations if needed
run_migrations() {
    echo ""
    echo "Checking database migrations..."
    cd packages/backend

    if [ -f .env ]; then
        source "$HOME/.cargo/env" 2>/dev/null || true
        if command -v sqlx &> /dev/null; then
            sqlx migrate run 2>/dev/null && echo -e "${GREEN}✓ Migrations up to date${NC}" || echo -e "${YELLOW}⚠ Migration check skipped${NC}"
        fi
    else
        echo -e "${YELLOW}⚠ No .env file found. Copy .env.example to .env${NC}"
    fi
    cd "$PROJECT_DIR"
}

# Start backend
start_backend() {
    echo ""
    echo "Starting backend server..."
    cd packages/backend
    source "$HOME/.cargo/env" 2>/dev/null || true

    # Kill any existing backend process
    pkill -f "notion-killer-backend" 2>/dev/null || true

    # Start backend in background
    cargo run 2>&1 > /tmp/notion-killer-backend.log &
    BACKEND_PID=$!
    echo $BACKEND_PID > /tmp/notion-killer-backend.pid

    cd "$PROJECT_DIR"

    # Wait for backend to start
    echo "Waiting for backend to start..."
    for i in {1..30}; do
        if curl -s http://localhost:3000 > /dev/null 2>&1; then
            echo -e "${GREEN}✓ Backend running on http://localhost:3000${NC}"
            return 0
        fi
        sleep 1
    done
    echo -e "${YELLOW}⚠ Backend may still be compiling. Check /tmp/notion-killer-backend.log${NC}"
}

# Start frontend
start_frontend() {
    echo ""
    echo "Starting frontend server..."
    cd packages/web

    # Kill any existing frontend process
    pkill -f "vite.*notion-killer" 2>/dev/null || true

    # Start frontend in background
    pnpm dev 2>&1 > /tmp/notion-killer-frontend.log &
    FRONTEND_PID=$!
    echo $FRONTEND_PID > /tmp/notion-killer-frontend.pid

    cd "$PROJECT_DIR"

    # Wait for frontend to start
    echo "Waiting for frontend to start..."
    for i in {1..15}; do
        if curl -s http://localhost:5173 > /dev/null 2>&1; then
            echo -e "${GREEN}✓ Frontend running on http://localhost:5173${NC}"
            return 0
        fi
        sleep 1
    done
    echo -e "${YELLOW}⚠ Frontend may still be starting. Check /tmp/notion-killer-frontend.log${NC}"
}

# Main
main() {
    check_docker
    start_infrastructure
    run_migrations
    start_backend
    start_frontend

    echo ""
    echo "=========================================="
    echo -e "${GREEN}  Application Started Successfully!${NC}"
    echo "=========================================="
    echo ""
    echo "  Frontend:  http://localhost:5173"
    echo "  Backend:   http://localhost:3000"
    echo "  MinIO:     http://localhost:9001"
    echo ""
    echo "  To stop:   ./scripts/stop.sh"
    echo "  Logs:      /tmp/notion-killer-*.log"
    echo ""
}

main
