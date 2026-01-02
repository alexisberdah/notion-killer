# Notion Killer

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![SvelteKit](https://img.shields.io/badge/SvelteKit-2.0-red.svg)](https://kit.svelte.dev/)
[![Flutter](https://img.shields.io/badge/Flutter-3.0-blue.svg)](https://flutter.dev/)

A blazing-fast, offline-first, open-source alternative to Notion. Built for speed, privacy, and real-time collaboration.

## Why Notion Killer?

Notion is great, but it has significant drawbacks:
- **Slow performance** - Heavy React + Electron stack causes lag
- **Always online** - Poor offline experience
- **Mobile app issues** - Slow and unresponsive on mobile devices
- **Privacy concerns** - All your data lives on their servers

**Notion Killer** solves these problems with:
- ⚡ **< 200ms load time** - Local-first architecture means instant operations
- 📴 **Offline-first** - Works without internet, syncs when connected
- 📱 **Native mobile apps** - Flutter-powered iOS/Android apps
- 🔒 **Privacy-focused** - Self-host option, your data stays yours
- 🤝 **Real-time collaboration** - CRDT-based conflict-free editing

## Features

### Core Features
- 📝 **Block-based editor** - Rich text, headings, lists, code blocks, and more
- 📊 **Databases** - Table, Kanban, Calendar, Gallery views
- 📁 **Nested pages** - Infinite hierarchy with drag-and-drop
- 🔍 **Full-text search** - Find anything instantly
- 🎨 **Customization** - Icons, covers, custom properties

### Collaboration
- 👥 **Real-time editing** - See cursors and changes live
- 💬 **Comments & mentions** - Discuss inline
- 🔗 **Sharing** - Public links with permissions
- 👤 **Workspaces** - Team collaboration

### Import & Export
- 📥 **Import from Notion** - Via API or ZIP export
- 📤 **Export** - Markdown, HTML, PDF
- 🔄 **Sync** - Cross-device synchronization

## Tech Stack

| Component | Technology |
|-----------|------------|
| **Frontend Web** | SvelteKit + TailwindCSS |
| **Backend API** | Rust (Axum) |
| **Mobile** | Flutter |
| **Database** | PostgreSQL (server) + SQLite (local) |
| **Real-time Sync** | WebSockets + CRDT (Yjs) |
| **File Storage** | S3-compatible (MinIO) |
| **Cache** | Redis |

## Quick Start

### Prerequisites

- [Node.js](https://nodejs.org/) 20+
- [Rust](https://rustup.rs/) 1.75+
- [Docker](https://docker.com/) & Docker Compose
- [pnpm](https://pnpm.io/) 9+

### Installation

```bash
# Clone the repository
git clone https://github.com/your-org/notion-killer.git
cd notion-killer

# Install dependencies
pnpm install

# Start infrastructure (PostgreSQL, Redis, MinIO)
docker compose -f docker/docker-compose.yml up -d

# Setup backend
cd packages/backend
cp .env.example .env
# Edit .env with your configuration

# Run database migrations
cargo sqlx migrate run

# Start the backend server
cargo run

# In a new terminal, start the frontend
cd packages/web
pnpm dev
```

Visit **http://localhost:5173** to access the application.

### Default Credentials

After starting, register a new account at `/register`.

## Project Structure

```
notion-killer/
├── packages/
│   ├── backend/              # Rust API server
│   │   ├── src/
│   │   │   ├── api/          # HTTP handlers & WebSocket
│   │   │   ├── domain/       # Business logic & entities
│   │   │   ├── infrastructure/  # Database & external services
│   │   │   └── sync/         # CRDT synchronization
│   │   └── migrations/       # SQL migrations
│   │
│   ├── web/                  # SvelteKit frontend
│   │   ├── src/
│   │   │   ├── lib/
│   │   │   │   ├── api/      # API client
│   │   │   │   ├── components/  # UI components
│   │   │   │   ├── crdt/     # Yjs integration
│   │   │   │   ├── editor/   # Block editor
│   │   │   │   └── stores/   # State management
│   │   │   └── routes/       # Pages
│   │   └── static/           # Static assets
│   │
│   ├── mobile/               # Flutter app (coming soon)
│   │
│   └── shared/
│       └── crdt-core/        # Shared CRDT logic (Rust/WASM)
│
├── tools/
│   └── notion-import/        # Notion import CLI
│
├── docker/                   # Docker configurations
├── docs/                     # Documentation
└── scripts/                  # Development scripts
```

## Development

### Running the Development Environment

```bash
# Terminal 1: Infrastructure
docker compose -f docker/docker-compose.yml up

# Terminal 2: Backend (with hot reload)
cd packages/backend
cargo watch -x run

# Terminal 3: Frontend (with hot reload)
cd packages/web
pnpm dev
```

### Running Tests

```bash
# Backend tests
cd packages/backend
cargo test

# Frontend tests
cd packages/web
pnpm test

# E2E tests
pnpm test:e2e
```

### Code Quality

```bash
# Lint
pnpm lint

# Format
pnpm format

# Type check
pnpm check
```

## Architecture

### Local-First with CRDT

Notion Killer uses a local-first architecture powered by CRDTs (Conflict-free Replicated Data Types):

```
┌─────────────────┐     ┌─────────────────┐
│   Client A      │     │   Client B      │
│  ┌───────────┐  │     │  ┌───────────┐  │
│  │  Yjs Doc  │  │     │  │  Yjs Doc  │  │
│  └─────┬─────┘  │     │  └─────┬─────┘  │
│        │        │     │        │        │
│  ┌─────▼─────┐  │     │  ┌─────▼─────┐  │
│  │ IndexedDB │  │     │  │ IndexedDB │  │
│  └───────────┘  │     │  └───────────┘  │
└────────┬────────┘     └────────┬────────┘
         │                       │
         │    ┌───────────┐      │
         └────►  WebSocket ◄─────┘
              │   Server   │
              └─────┬──────┘
                    │
              ┌─────▼──────┐
              │ PostgreSQL │
              └────────────┘
```

**Benefits:**
- All operations are instant (no network latency)
- Works offline automatically
- Conflicts are resolved automatically
- Real-time collaboration without locks

### API Design

RESTful API with WebSocket for real-time features:

```
POST   /api/v1/auth/register    # Create account
POST   /api/v1/auth/login       # Login
POST   /api/v1/auth/refresh     # Refresh tokens

GET    /api/v1/workspaces       # List workspaces
POST   /api/v1/workspaces       # Create workspace
GET    /api/v1/workspaces/:id   # Get workspace
PATCH  /api/v1/workspaces/:id   # Update workspace
DELETE /api/v1/workspaces/:id   # Delete workspace

GET    /api/v1/pages            # List pages
POST   /api/v1/pages            # Create page
GET    /api/v1/pages/:id        # Get page
PATCH  /api/v1/pages/:id        # Update page
DELETE /api/v1/pages/:id        # Delete page

WS     /api/v1/sync/:page_id    # Real-time sync
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `PORT` | Server port | `3000` |
| `DATABASE_URL` | PostgreSQL connection string | Required |
| `JWT_SECRET` | Secret for JWT signing | Required |
| `JWT_ACCESS_TOKEN_EXPIRY` | Access token lifetime (seconds) | `900` |
| `JWT_REFRESH_TOKEN_EXPIRY` | Refresh token lifetime (seconds) | `604800` |
| `REDIS_URL` | Redis connection string | Optional |
| `S3_BUCKET` | S3 bucket for file storage | Optional |

### Docker Compose Services

| Service | Port | Description |
|---------|------|-------------|
| PostgreSQL | 5432 | Main database |
| Redis | 6379 | Caching & sessions |
| MinIO | 9000, 9001 | S3-compatible storage |

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add new block type
fix: resolve sync conflict issue
docs: update API documentation
refactor: simplify editor logic
test: add unit tests for auth
```

## Roadmap

- [x] **Phase 1**: Foundation (Auth, Workspaces, Basic UI)
- [x] **Phase 2**: Block Editor (Tiptap, Slash commands, Selection toolbar, CRDT sync)
- [ ] **Phase 3**: Pages & Navigation (Hierarchy, Sidebar, Search)
- [ ] **Phase 4**: Databases (Table, Kanban, Calendar, Gallery)
- [ ] **Phase 5**: Collaboration (Real-time WebSocket, Comments, Sharing)
- [ ] **Phase 6**: Notion Import (API & ZIP)
- [ ] **Phase 7**: Mobile Apps (Flutter iOS/Android)

## Ideas & Future Features

- [ ] **Mascot**: Add a cute animal mascot to give the app personality and make it more memorable
- [ ] **AI Assistant**: Smart writing suggestions and auto-complete
- [ ] **Templates**: Pre-built page templates for common use cases
- [ ] **Plugins/Extensions**: Allow community extensions
- [ ] **Self-hosting**: Easy Docker deployment for privacy-focused users

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Notion](https://notion.so) - For the inspiration
- [Yjs](https://yjs.dev) - CRDT implementation
- [Tiptap](https://tiptap.dev) - Editor framework
- [SvelteKit](https://kit.svelte.dev) - Web framework
- [Axum](https://github.com/tokio-rs/axum) - Rust web framework

---

<p align="center">
  Made with ❤️ by the Notion Killer Team
</p>
