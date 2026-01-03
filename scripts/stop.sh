#!/bin/bash

# Notion Killer - Stop Script
# Usage: ./scripts/stop.sh [--all]
#
# Options:
#   --all    Also stop Docker infrastructure and Colima

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

echo "=========================================="
echo "  Notion Killer - Stopping Application"
echo "=========================================="
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

STOP_ALL=false
if [ "$1" == "--all" ]; then
    STOP_ALL=true
fi

# Stop frontend
stop_frontend() {
    echo "Stopping frontend..."
    pkill -f "vite" 2>/dev/null || true
    rm -f /tmp/notion-killer-frontend.pid
    echo -e "${GREEN}✓ Frontend stopped${NC}"
}

# Stop backend
stop_backend() {
    echo "Stopping backend..."
    pkill -f "notion-killer-backend" 2>/dev/null || true
    rm -f /tmp/notion-killer-backend.pid
    echo -e "${GREEN}✓ Backend stopped${NC}"
}

# Stop infrastructure
stop_infrastructure() {
    echo "Stopping infrastructure..."
    docker compose -f docker/docker-compose.yml down 2>/dev/null || true
    echo -e "${GREEN}✓ Infrastructure stopped${NC}"
}

# Stop Colima
stop_colima() {
    echo "Stopping Colima..."
    colima stop 2>/dev/null || true
    echo -e "${GREEN}✓ Colima stopped${NC}"
}

# Main
main() {
    stop_frontend
    stop_backend

    if [ "$STOP_ALL" = true ]; then
        echo ""
        stop_infrastructure
        stop_colima
    else
        echo ""
        echo -e "${YELLOW}Note: Docker infrastructure is still running.${NC}"
        echo "To stop everything: ./scripts/stop.sh --all"
    fi

    echo ""
    echo "=========================================="
    echo -e "${GREEN}  Application Stopped${NC}"
    echo "=========================================="
    echo ""
}

main
