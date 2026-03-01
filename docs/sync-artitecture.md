# Offline-First Sync Architecture (Tauri + SQLite + PostgreSQL)

## Overview

This document describes a production-ready offline-first synchronization system using:

* **Tauri (Frontend + Rust backend)**
* **SQLite (local database)**
* **Cloud API (Rust/Node/etc.)**
* **PostgreSQL (cloud database)**

The system ensures:

* Full offline capability
* Automatic background synchronization
* Conflict resolution
* Soft deletes
* Scalable architecture

---

# Architecture

```
Tauri Frontend (JS/TS)
        ↓
SQLite (Local)
        ↓
Tauri Rust Sync Command
        ↓
Cloud REST API
        ↓
PostgreSQL
```

Important:

* The frontend NEVER connects directly to PostgreSQL.
* All synchronization goes through the backend API.

---

# Core Database Design

## SQLite + PostgreSQL Schema (Must Match)

```sql
CREATE TABLE notes (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  content TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  deleted_at TEXT,
  version INTEGER DEFAULT 1,
  synced INTEGER DEFAULT 0
);
```

### Why These Fields?

| Field      | Purpose                    |
| ---------- | -------------------------- |
| id         | UUID shared across devices |
| created_at | Record creation timestamp  |
| updated_at | Conflict resolution        |
| deleted_at | Soft delete support        |
| version    | Optional conflict control  |
| synced     | Local sync tracking        |

---

# Rules for Reliable Sync

## 1. Always Write Locally First

Never wait for network before saving.

Frontend → SQLite → Later sync

---

## 2. Use UUIDs (Never Auto Increment IDs)

Rust example:

```rust
use uuid::Uuid;
let id = Uuid::new_v4().to_string();
```

---

## 3. Soft Deletes Only

Instead of:

```sql
DELETE FROM notes WHERE id = ?;
```

Use:

```sql
UPDATE notes
SET deleted_at = CURRENT_TIMESTAMP,
    synced = 0
WHERE id = ?;
```

This ensures deletes sync across devices.

---

# Sync Flow

## Step 1 – Detect Online Status

Frontend (JS):

```ts
window.addEventListener("online", () => {
  invoke("sync_data")
})
```

You can also sync:

* On app startup
* On window focus
* On interval (e.g. every 30s)

---

## Step 2 – Rust Sync Command

```rust
#[tauri::command]
async fn sync_data(app: tauri::AppHandle) -> Result<(), String> {
    let db = get_db(&app)?;

    // 1. Fetch unsynced rows
    let notes: Vec<Note> = sqlx::query_as!(
        Note,
        "SELECT * FROM notes WHERE synced = 0"
    )
    .fetch_all(&db)
    .await
    .map_err(|e| e.to_string())?;

    if notes.is_empty() {
        return Ok(());
    }

    // 2. Send to server
    let client = reqwest::Client::new();
    client
        .post("https://your-api.com/sync")
        .json(&notes)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    // 3. Mark as synced
    sqlx::query!("UPDATE notes SET synced = 1 WHERE synced = 0")
        .execute(&db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
```

---

# Backend API (PostgreSQL)

Use UPSERT with conflict protection:

```sql
INSERT INTO notes (id, title, content, updated_at)
VALUES ($1, $2, $3, $4)
ON CONFLICT (id)
DO UPDATE SET
  title = EXCLUDED.title,
  content = EXCLUDED.content,
  updated_at = EXCLUDED.updated_at
WHERE notes.updated_at < EXCLUDED.updated_at;
```

This ensures last-write-wins conflict handling.

---

# Pulling Remote Changes (Bidirectional Sync)

To support multi-device sync:

## Add last_sync timestamp locally

```sql
CREATE TABLE app_state (
  key TEXT PRIMARY KEY,
  value TEXT
);
```

Store:

```
last_sync = "2026-02-17T10:00:00Z"
```

---

## Sync Process

1. Send unsynced local changes
2. Server applies UPSERT
3. Server returns records where updated_at > last_sync
4. Apply those to SQLite
5. Update last_sync

---

# Advanced Production Improvement (Recommended)

Instead of scanning tables with `synced = 0`, create a sync queue.

## Sync Queue Table

```sql
CREATE TABLE sync_queue (
  id TEXT PRIMARY KEY,
  table_name TEXT NOT NULL,
  record_id TEXT NOT NULL,
  operation TEXT NOT NULL,
  created_at TEXT NOT NULL
);
```

Each mutation inserts into queue.

During sync:

* Read queue
* Send batch to server
* Clear processed rows

This scales much better for large datasets.

---

# Conflict Resolution Strategies

## Simple

Last write wins using updated_at.

## Safer

Use version numbers:

```sql
version INTEGER DEFAULT 1
```

Increment version on each update.

Reject updates when version mismatch occurs.

---

# Sync Triggers

Recommended triggers:

* App startup
* Online event
* Every 30 seconds
* On window focus
* After mutation (if online)

---

# Failure Handling

If sync fails:

* Do NOT mark rows as synced
* Retry on next trigger
* Implement exponential backoff if needed

---

# Security Best Practices

* Authenticate API requests
* Scope sync per user
* Never expose Postgres directly
* Use HTTPS only

---

# Summary

This architecture provides:

* True offline capability
* Safe conflict resolution
* Multi-device support
* Scalable sync
* Production-grade reliability

Core principle:

> Always write locally first. Sync later. Never block UI on network.
