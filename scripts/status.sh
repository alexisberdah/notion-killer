#!/bin/bash

# Notion Killer - Status Script
# Usage: ./scripts/status.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

echo "=========================================="
echo "  Notion Killer - Status"
echo "=========================================="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Check service status
check_service() {
    local name=$1
    local url=$2
    local port=$3

    if curl -s "$url" > /dev/null 2>&1; then
        echo -e "  ${GREEN}●${NC} $name (port $port)"
    else
        echo -e "  ${RED}○${NC} $name (port $port)"
    fi
}

check_docker_service() {
    local name=$1
    local container=$2

    if docker ps --format '{{.Names}}' 2>/dev/null | grep -q "$container"; then
        local health=$(docker inspect --format='{{.State.Health.Status}}' "$container" 2>/dev/null || echo "running")
        if [ "$health" == "healthy" ]; then
            echo -e "  ${GREEN}●${NC} $name (healthy)"
        else
            echo -e "  ${YELLOW}●${NC} $name ($health)"
        fi
    else
        echo -e "  ${RED}○${NC} $name (stopped)"
    fi
}

echo "Application Services:"
check_service "Frontend" "http://localhost:5173" "5173"
check_service "Backend API" "http://localhost:3000" "3000"

echo ""
echo "Infrastructure:"

if docker info > /dev/null 2>&1; then
    check_docker_service "PostgreSQL" "notion-killer-db"
    check_docker_service "Redis" "notion-killer-redis"
    check_docker_service "MinIO" "notion-killer-minio"
else
    echo -e "  ${RED}○${NC} Docker not running"
fi

echo ""
echo "=========================================="
echo ""
echo "Legend: ${GREEN}●${NC} Running  ${YELLOW}●${NC} Starting  ${RED}○${NC} Stopped"
echo ""
