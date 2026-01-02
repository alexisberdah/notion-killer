# API Reference

This document describes the Notion Killer REST API.

## Base URL

```
http://localhost:3000/api/v1
```

## Authentication

All authenticated endpoints require a Bearer token in the Authorization header:

```
Authorization: Bearer <access_token>
```

## Error Responses

All errors follow this format:

```json
{
  "error": {
    "message": "Human-readable error message",
    "code": 400
  }
}
```

### Common Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad Request - Invalid input |
| 401 | Unauthorized - Missing or invalid token |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource doesn't exist |
| 409 | Conflict - Resource already exists |
| 429 | Too Many Requests - Rate limited |
| 500 | Internal Server Error |

---

## Authentication

### Register

Create a new user account.

```http
POST /auth/register
```

**Request Body:**

```json
{
  "email": "user@example.com",
  "password": "securepassword123",
  "name": "John Doe"
}
```

**Response:** `200 OK`

```json
{
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "name": "John Doe",
    "avatar_url": null
  },
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIs..."
}
```

**Errors:**
- `400` - Invalid email format or password too short
- `409` - User already exists

---

### Login

Authenticate an existing user.

```http
POST /auth/login
```

**Request Body:**

```json
{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

**Response:** `200 OK`

```json
{
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "name": "John Doe",
    "avatar_url": null
  },
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIs..."
}
```

**Errors:**
- `401` - Invalid credentials

---

### Refresh Token

Get new access and refresh tokens.

```http
POST /auth/refresh
```

**Request Body:**

```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIs..."
}
```

**Response:** `200 OK`

```json
{
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "name": "John Doe",
    "avatar_url": null
  },
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIs..."
}
```

**Errors:**
- `401` - Invalid or expired refresh token

---

### Logout

Revoke the refresh token.

```http
POST /auth/logout
```

**Request Body:**

```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIs..."
}
```

**Response:** `200 OK`

```json
{
  "message": "Logged out successfully"
}
```

---

## Users

### Get Current User

Get the authenticated user's profile.

```http
GET /users/me
```

**Response:** `200 OK`

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "name": "John Doe",
  "avatar_url": "https://..."
}
```

---

## Workspaces

### List Workspaces

Get all workspaces the user is a member of.

```http
GET /workspaces
```

**Response:** `200 OK`

```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440001",
    "name": "My Workspace",
    "icon": "🚀",
    "owner_id": "550e8400-e29b-41d4-a716-446655440000",
    "created_at": "2025-01-01T00:00:00Z"
  }
]
```

---

### Create Workspace

Create a new workspace.

```http
POST /workspaces
```

**Request Body:**

```json
{
  "name": "New Workspace",
  "icon": "📁"
}
```

**Response:** `200 OK`

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440002",
  "name": "New Workspace",
  "icon": "📁",
  "owner_id": "550e8400-e29b-41d4-a716-446655440000",
  "created_at": "2025-01-01T00:00:00Z"
}
```

---

### Get Workspace

Get a specific workspace.

```http
GET /workspaces/:id
```

**Response:** `200 OK`

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "name": "My Workspace",
  "icon": "🚀",
  "owner_id": "550e8400-e29b-41d4-a716-446655440000",
  "created_at": "2025-01-01T00:00:00Z"
}
```

**Errors:**
- `404` - Workspace not found or not a member

---

### Update Workspace

Update workspace details.

```http
PATCH /workspaces/:id
```

**Request Body:**

```json
{
  "name": "Updated Name",
  "icon": "🎯"
}
```

**Response:** `200 OK`

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "name": "Updated Name",
  "icon": "🎯",
  "owner_id": "550e8400-e29b-41d4-a716-446655440000",
  "created_at": "2025-01-01T00:00:00Z"
}
```

**Errors:**
- `403` - Not an admin or owner
- `404` - Workspace not found

---

### Delete Workspace

Delete a workspace (owner only).

```http
DELETE /workspaces/:id
```

**Response:** `200 OK`

```json
{
  "message": "Workspace deleted"
}
```

**Errors:**
- `403` - Not the owner
- `404` - Workspace not found

---

## Pages

### List Pages

Get all pages in a workspace.

```http
GET /workspaces/:workspace_id/pages
```

**Query Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `parent_id` | UUID | Filter by parent page |

**Response:** `200 OK`

```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440010",
    "title": "My Page",
    "icon": "📝",
    "parent_id": null,
    "is_database": false,
    "created_at": "2025-01-01T00:00:00Z"
  }
]
```

---

### Create Page

Create a new page.

```http
POST /workspaces/:workspace_id/pages
```

**Request Body:**

```json
{
  "title": "New Page",
  "icon": "📄",
  "parent_id": null
}
```

**Response:** `200 OK`

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440011",
  "title": "New Page",
  "icon": "📄",
  "parent_id": null,
  "is_database": false,
  "created_at": "2025-01-01T00:00:00Z"
}
```

---

### Get Page

Get a specific page with its content.

```http
GET /pages/:id
```

**Response:** `200 OK`

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440010",
  "title": "My Page",
  "icon": "📝",
  "cover_url": null,
  "parent_id": null,
  "is_database": false,
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-01-01T00:00:00Z"
}
```

---

### Update Page

Update page metadata.

```http
PATCH /pages/:id
```

**Request Body:**

```json
{
  "title": "Updated Title",
  "icon": "🎉",
  "cover_url": "https://..."
}
```

**Response:** `200 OK`

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440010",
  "title": "Updated Title",
  "icon": "🎉",
  "cover_url": "https://..."
}
```

---

### Delete Page

Delete a page and its children.

```http
DELETE /pages/:id
```

**Response:** `200 OK`

```json
{
  "message": "Page deleted"
}
```

---

## WebSocket Sync

### Connect to Page

Establish a real-time sync connection for a page.

```
WS /sync/:page_id
```

**Connection:**

```javascript
const ws = new WebSocket('ws://localhost:3000/api/v1/sync/PAGE_ID');
ws.onopen = () => {
  ws.send(JSON.stringify({
    type: 'auth',
    token: 'ACCESS_TOKEN'
  }));
};
```

**Message Types:**

#### Sync Request
```json
{
  "type": "sync",
  "state_vector": "base64_encoded_state_vector"
}
```

#### Update
```json
{
  "type": "update",
  "data": "base64_encoded_crdt_update"
}
```

#### Awareness (Presence)
```json
{
  "type": "awareness",
  "data": {
    "user": {
      "id": "user_id",
      "name": "John",
      "color": "#ff0000"
    },
    "cursor": {
      "block_id": "block_id",
      "offset": 10
    }
  }
}
```

---

## Rate Limiting

API requests are rate limited:

| Endpoint Type | Limit |
|---------------|-------|
| Authentication | 10 requests/minute |
| General API | 100 requests/minute |
| WebSocket | 1000 messages/minute |

Rate limit headers are included in responses:

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1609459200
```

---

## Pagination

List endpoints support pagination:

**Query Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `limit` | number | 50 | Items per page (max 100) |
| `offset` | number | 0 | Number of items to skip |

**Response Headers:**

```
X-Total-Count: 150
X-Page-Size: 50
```

---

## Versioning

The API is versioned via the URL path (`/api/v1/`). When breaking changes are introduced, a new version will be released.

Current version: **v1**
