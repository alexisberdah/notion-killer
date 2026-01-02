# Architecture Overview

This document describes the high-level architecture of Notion Killer.

## System Architecture

```
                                    ┌─────────────────────────────────────┐
                                    │            CLIENTS                  │
                                    └─────────────────────────────────────┘
                                                    │
                    ┌───────────────────────────────┼───────────────────────────────┐
                    │                               │                               │
            ┌───────▼───────┐               ┌───────▼───────┐               ┌───────▼───────┐
            │   Web App     │               │  Mobile App   │               │   Desktop     │
            │  (SvelteKit)  │               │   (Flutter)   │               │   (Tauri)     │
            └───────┬───────┘               └───────┬───────┘               └───────┬───────┘
                    │                               │                               │
                    └───────────────────────────────┼───────────────────────────────┘
                                                    │
                            ┌───────────────────────┴───────────────────────┐
                            │                                               │
                    ┌───────▼───────┐                               ┌───────▼───────┐
                    │   LOCAL-FIRST │                               │   REAL-TIME   │
                    │     LAYER     │                               │     SYNC      │
                    │               │                               │               │
                    │  ┌─────────┐  │                               │  ┌─────────┐  │
                    │  │   Yjs   │  │◄──────────────────────────────┤  │WebSocket│  │
                    │  │  (CRDT) │  │                               │  │ Server  │  │
                    │  └────┬────┘  │                               │  └────┬────┘  │
                    │       │       │                               │       │       │
                    │  ┌────▼────┐  │                               │       │       │
                    │  │IndexedDB│  │                               │       │       │
                    │  │/SQLite  │  │                               │       │       │
                    │  └─────────┘  │                               │       │       │
                    └───────────────┘                               └───────┼───────┘
                                                                            │
                                    ┌───────────────────────────────────────▼───────────────────────────────────────┐
                                    │                           BACKEND (RUST / AXUM)                               │
                                    │  ┌─────────────────────────────────────────────────────────────────────────┐  │
                                    │  │                              API LAYER                                   │  │
                                    │  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │  │
                                    │  │  │  REST    │  │WebSocket │  │  Auth    │  │  Rate    │  │  CORS    │   │  │
                                    │  │  │ Handlers │  │ Handler  │  │Middleware│  │ Limiter  │  │          │   │  │
                                    │  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │  │
                                    │  └─────────────────────────────────────────────────────────────────────────┘  │
                                    │  ┌─────────────────────────────────────────────────────────────────────────┐  │
                                    │  │                            DOMAIN LAYER                                  │  │
                                    │  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │  │
                                    │  │  │  Auth    │  │  Page    │  │ Database │  │   Sync   │  │  Import  │   │  │
                                    │  │  │ Service  │  │ Service  │  │ Service  │  │ Service  │  │ Service  │   │  │
                                    │  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │  │
                                    │  └─────────────────────────────────────────────────────────────────────────┘  │
                                    │  ┌─────────────────────────────────────────────────────────────────────────┐  │
                                    │  │                        INFRASTRUCTURE LAYER                              │  │
                                    │  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │  │
                                    │  │  │PostgreSQL│  │  Redis   │  │    S3    │  │  Notion  │  │  Email   │   │  │
                                    │  │  │   Repo   │  │  Cache   │  │ Storage  │  │   API    │  │ Service  │   │  │
                                    │  │  └────┬─────┘  └────┬─────┘  └────┬─────┘  └──────────┘  └──────────┘   │  │
                                    │  └───────┼─────────────┼────────────┼──────────────────────────────────────┘  │
                                    └──────────┼─────────────┼────────────┼─────────────────────────────────────────┘
                                               │             │            │
                                    ┌──────────▼─────────────▼────────────▼─────────────────────────────────────────┐
                                    │                              DATA STORES                                      │
                                    │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────────┐    │
                                    │  │   PostgreSQL    │  │      Redis      │  │       S3 / MinIO            │    │
                                    │  │                 │  │                 │  │                             │    │
                                    │  │  - Users        │  │  - Sessions     │  │  - File uploads             │    │
                                    │  │  - Workspaces   │  │  - Cache        │  │  - Images                   │    │
                                    │  │  - Pages        │  │  - Rate limits  │  │  - Exports                  │    │
                                    │  │  - CRDT state   │  │  - Presence     │  │                             │    │
                                    │  │  - Databases    │  │                 │  │                             │    │
                                    │  └─────────────────┘  └─────────────────┘  └─────────────────────────────┘    │
                                    └───────────────────────────────────────────────────────────────────────────────┘
```

## Core Concepts

### 1. Local-First Architecture

Every client maintains a complete copy of the data locally:

- **Instant Operations**: All changes happen locally first, providing sub-millisecond response times
- **Offline Support**: The app works fully offline, syncing when connection is restored
- **Conflict Resolution**: CRDTs automatically merge changes without conflicts

### 2. CRDT (Conflict-free Replicated Data Types)

We use [Yjs](https://yjs.dev/) for CRDT implementation:

```typescript
// Document structure in Yjs
const doc = new Y.Doc();
const blocks = doc.getMap('blocks');      // Block content
const blockOrder = doc.getArray('order'); // Block ordering
const title = doc.getText('title');       // Page title
```

**Why CRDTs?**
- No central authority needed for conflict resolution
- Guaranteed eventual consistency
- Perfect for collaborative editing

### 3. Block-Based Content Model

Content is organized as a tree of blocks:

```typescript
interface Block {
  id: string;
  type: BlockType;
  content: RichText[];
  properties: Record<string, unknown>;
  children: string[]; // Child block IDs
}

enum BlockType {
  Paragraph = 'paragraph',
  Heading1 = 'heading_1',
  Heading2 = 'heading_2',
  Heading3 = 'heading_3',
  BulletedList = 'bulleted_list',
  NumberedList = 'numbered_list',
  TodoList = 'todo_list',
  Toggle = 'toggle',
  Quote = 'quote',
  Callout = 'callout',
  Code = 'code',
  Divider = 'divider',
  Image = 'image',
  // ... more types
}
```

## Backend Architecture

### Clean Architecture Layers

```
┌─────────────────────────────────────────────────────────┐
│                      API Layer                          │
│  - HTTP handlers (REST)                                 │
│  - WebSocket handlers                                   │
│  - Request/Response DTOs                                │
│  - Middleware (auth, rate limiting, CORS)               │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                    Domain Layer                         │
│  - Entities (User, Page, Workspace, Block)              │
│  - Services (AuthService, PageService, SyncService)     │
│  - Repository traits (interfaces)                       │
│  - Domain errors                                        │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                Infrastructure Layer                     │
│  - Repository implementations (PostgreSQL)              │
│  - External service clients (S3, Redis, Notion API)     │
│  - Database connection management                       │
└─────────────────────────────────────────────────────────┘
```

### Key Design Decisions

1. **Axum for Web Framework**: Async-first, type-safe, excellent performance
2. **SQLx for Database**: Compile-time checked SQL queries
3. **Yrs for CRDT**: Rust port of Yjs, compatible protocol

## Frontend Architecture

### SvelteKit Structure

```
src/
├── lib/
│   ├── components/
│   │   ├── ui/           # Base components (Button, Input, Modal)
│   │   ├── layout/       # Layout components (Sidebar, Header)
│   │   ├── editor/       # Editor components
│   │   └── database/     # Database view components
│   │
│   ├── stores/           # Svelte 5 runes-based state
│   │   ├── auth.svelte.ts
│   │   ├── pages.svelte.ts
│   │   └── editor.svelte.ts
│   │
│   ├── crdt/             # Yjs integration
│   │   ├── document.ts
│   │   ├── provider.ts
│   │   └── persistence.ts
│   │
│   └── api/              # API client
│
└── routes/
    ├── (auth)/           # Auth pages (login, register)
    ├── (app)/            # Authenticated app pages
    └── share/            # Public shared pages
```

### State Management

Using Svelte 5 runes for reactive state:

```typescript
// stores/pages.svelte.ts
class PageStore {
  pages = $state<Map<string, Page>>(new Map());
  currentPageId = $state<string | null>(null);

  get currentPage() {
    return this.currentPageId
      ? this.pages.get(this.currentPageId)
      : null;
  }
}
```

## Data Model

### Database Schema (Simplified)

```sql
-- Users
CREATE TABLE users (
  id UUID PRIMARY KEY,
  email VARCHAR(255) UNIQUE NOT NULL,
  name VARCHAR(255) NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Workspaces
CREATE TABLE workspaces (
  id UUID PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  owner_id UUID REFERENCES users(id)
);

-- Pages
CREATE TABLE pages (
  id UUID PRIMARY KEY,
  workspace_id UUID REFERENCES workspaces(id),
  parent_id UUID REFERENCES pages(id),
  title VARCHAR(500),
  crdt_state BYTEA,  -- Serialized Yjs document
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- CRDT Updates (for sync)
CREATE TABLE crdt_updates (
  id BIGSERIAL PRIMARY KEY,
  page_id UUID REFERENCES pages(id),
  update_data BYTEA NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);
```

## Sync Protocol

### WebSocket Message Flow

```
Client                                    Server
   │                                         │
   │──────── Connect + Auth Token ──────────>│
   │                                         │
   │<─────── Connection Accepted ────────────│
   │                                         │
   │──────── Subscribe to Page ─────────────>│
   │                                         │
   │<─────── Current State Vector ───────────│
   │                                         │
   │──────── Request Missing Updates ───────>│
   │                                         │
   │<─────── CRDT Updates ───────────────────│
   │                                         │
   │──────── Local Changes ─────────────────>│
   │                                         │
   │<─────── Broadcast to Other Clients ─────│
   │                                         │
```

### Sync Algorithm

1. Client connects and sends state vector
2. Server compares with its state vector
3. Server sends missing updates to client
4. Client applies updates and sends its missing updates
5. Continuous bidirectional sync via WebSocket

## Security

### Authentication Flow

```
┌────────┐     ┌────────┐     ┌────────┐
│ Client │     │ Server │     │   DB   │
└───┬────┘     └───┬────┘     └───┬────┘
    │              │              │
    │──Register───>│              │
    │              │──Hash Pass──>│
    │              │<─────────────│
    │<─JWT Tokens──│              │
    │              │              │
    │──Request────>│              │
    │  + Access    │              │
    │    Token     │              │
    │              │──Verify──────│
    │<─Response────│              │
    │              │              │
    │──Refresh────>│              │
    │  Token       │──Validate───>│
    │<─New Tokens──│<─────────────│
```

### Security Measures

- **Password Hashing**: Argon2id
- **JWT Tokens**: Short-lived access tokens (15 min), long-lived refresh tokens (7 days)
- **HTTPS Only**: All traffic encrypted
- **CORS**: Strict origin validation
- **Rate Limiting**: Per-IP and per-user limits
- **Input Validation**: Server-side validation for all inputs

## Performance Optimizations

1. **Local-First**: Eliminates network latency for most operations
2. **CRDT Compaction**: Periodic snapshots to reduce sync payload
3. **Lazy Loading**: Pages loaded on demand
4. **Virtual Scrolling**: For large documents and databases
5. **WebSocket Connection Pooling**: Efficient resource usage
6. **Redis Caching**: Hot data cached in memory
7. **PostgreSQL Indexes**: Optimized queries

## Scalability

### Horizontal Scaling

```
                    ┌─────────────┐
                    │   Clients   │
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │Load Balancer│
                    └──────┬──────┘
                           │
          ┌────────────────┼────────────────┐
          │                │                │
    ┌─────▼─────┐    ┌─────▼─────┐    ┌─────▼─────┐
    │  Server 1 │    │  Server 2 │    │  Server 3 │
    └─────┬─────┘    └─────┬─────┘    └─────┬─────┘
          │                │                │
          └────────────────┼────────────────┘
                           │
                    ┌──────▼──────┐
                    │ Redis Pub/Sub│ (for cross-server sync)
                    └──────┬──────┘
                           │
                    ┌──────▼──────┐
                    │ PostgreSQL  │ (with read replicas)
                    └─────────────┘
```

## Future Considerations

1. **GraphQL API**: For more flexible queries
2. **Edge Deployment**: For lower latency globally
3. **E2E Encryption**: For sensitive workspaces
4. **Plugin System**: For extensibility
5. **AI Features**: Smart suggestions, auto-complete
