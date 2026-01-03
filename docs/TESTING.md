# Testing Guide

This document describes how to test Notion Killer during development.

## Prerequisites

### Required
- [Node.js](https://nodejs.org/) 20+
- [pnpm](https://pnpm.io/) 9+
- [Rust](https://rustup.rs/) 1.75+

### Optional (for full stack testing)
- [Docker](https://docker.com/) & Docker Compose
- PostgreSQL 15+ (if not using Docker)

## Quick Start (Frontend Only)

If you want to test the frontend without the backend:

```bash
# Install dependencies
pnpm install

# Start frontend dev server
cd packages/web
pnpm dev
```

Visit http://localhost:5173

> **Note:** Without the backend, API calls will fail but the editor UI works locally with IndexedDB persistence.

## Full Stack Testing

### 1. Start Infrastructure

```bash
# Start PostgreSQL, Redis, MinIO
docker compose -f docker/docker-compose.yml up -d

# Verify services are running
docker compose -f docker/docker-compose.yml ps
```

| Service | Port | Health Check |
|---------|------|--------------|
| PostgreSQL | 5432 | `pg_isready -h localhost -p 5432` |
| Redis | 6379 | `redis-cli ping` |
| MinIO | 9000, 9001 | http://localhost:9001 |

### 2. Setup Backend

```bash
cd packages/backend

# Copy environment file
cp .env.example .env

# Edit .env with your settings:
# DATABASE_URL=postgresql://postgres:postgres@localhost:5432/notion_killer
# JWT_SECRET=your-secret-key-here

# Run database migrations
cargo sqlx migrate run

# Start backend server
cargo run
```

Backend runs at http://localhost:3000

### 3. Start Frontend

```bash
cd packages/web
pnpm dev
```

Frontend runs at http://localhost:5173

## Testing the Editor

### Test Page

A dedicated test page is available at `/test-editor` for testing the block editor in isolation.

### Editor Features to Test

| Feature | How to Test |
|---------|-------------|
| **Slash Commands** | Type `/` to open command menu |
| **Text Formatting** | Select text to show formatting toolbar |
| **Bold** | `Cmd+B` or toolbar button |
| **Italic** | `Cmd+I` or toolbar button |
| **Underline** | `Cmd+U` or toolbar button |
| **Strikethrough** | `Cmd+Shift+S` or toolbar button |
| **Inline Code** | `Cmd+E` or toolbar button |
| **Headings** | Type `/h1`, `/h2`, `/h3` or use toolbar |
| **Bullet List** | Type `/bullet` or `-` + space |
| **Numbered List** | Type `/numbered` or `1.` + space |
| **Todo List** | Type `/todo` or `[]` + space |
| **Quote** | Type `/quote` or `>` + space |
| **Code Block** | Type `/code` |
| **Divider** | Type `/divider` or `---` |

### CRDT Sync Testing

The editor uses Yjs for CRDT-based sync with IndexedDB persistence:

1. Open a page in the editor
2. Type some content
3. Refresh the page - content should persist
4. Open the same page in another tab - changes should sync

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd+B` | Bold |
| `Cmd+I` | Italic |
| `Cmd+U` | Underline |
| `Cmd+E` | Inline code |
| `Cmd+Shift+S` | Strikethrough |
| `Cmd+Shift+H` | Highlight |
| `Cmd+K` | Add link |
| `Cmd+Z` | Undo |
| `Cmd+Shift+Z` | Redo |
| `Tab` | Indent list |
| `Shift+Tab` | Outdent list |
| `Enter` | New paragraph |
| `Shift+Enter` | Line break |

## API Testing

### Authentication

```bash
# Register
curl -X POST http://localhost:3000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123", "name": "Test User"}'

# Login
curl -X POST http://localhost:3000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123"}'
```

### Workspaces

```bash
# List workspaces (requires auth token)
curl http://localhost:3000/api/v1/workspaces \
  -H "Authorization: Bearer <token>"

# Create workspace
curl -X POST http://localhost:3000/api/v1/workspaces \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"name": "My Workspace"}'
```

### Pages

```bash
# List pages in workspace
curl http://localhost:3000/api/v1/pages?workspace_id=<workspace_id> \
  -H "Authorization: Bearer <token>"

# Create page
curl -X POST http://localhost:3000/api/v1/pages \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"workspace_id": "<workspace_id>", "title": "New Page"}'

# Get page
curl http://localhost:3000/api/v1/pages/<page_id> \
  -H "Authorization: Bearer <token>"
```

## Running Tests

### Backend Tests

```bash
cd packages/backend
cargo test
```

### Frontend Tests

```bash
cd packages/web

# Unit tests
pnpm test

# E2E tests (requires running server)
pnpm test:e2e
```

### Code Quality

```bash
# Lint
pnpm lint

# Format check
pnpm format

# Type check
pnpm check
```

## Troubleshooting

### Port Already in Use

```bash
# Find and kill process on port
lsof -ti:5173 | xargs kill -9
lsof -ti:3000 | xargs kill -9
```

### Database Connection Issues

```bash
# Check if PostgreSQL is running
docker compose -f docker/docker-compose.yml ps

# View logs
docker compose -f docker/docker-compose.yml logs postgres
```

### Clear Frontend Cache

```bash
cd packages/web
rm -rf .svelte-kit node_modules/.vite
pnpm dev
```

### Reset Database

```bash
# Drop and recreate database
docker compose -f docker/docker-compose.yml down -v
docker compose -f docker/docker-compose.yml up -d
cd packages/backend
cargo sqlx migrate run
```

## Browser DevTools

### Useful Console Commands

```javascript
// Check IndexedDB data
indexedDB.databases()

// Clear IndexedDB
indexedDB.deleteDatabase('notion-killer-crdt')

// Check localStorage
localStorage.getItem('auth')
```

### Network Tab

- Filter by `api` to see API calls
- Filter by `ws` to see WebSocket connections

## Test Accounts

For development, you can create test accounts via the register endpoint or UI.

Default test credentials after seeding (if implemented):
- Email: `admin@example.com`
- Password: `admin123`
