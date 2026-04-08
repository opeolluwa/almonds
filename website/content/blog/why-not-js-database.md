---
title: "Why SeaORM over JavaScript client database options?"
description: "There are several offline-first JavaScript client databases today, each with it's own perks, but what happens when data replication is more important"
date: 2026-03-11
author: "Adefemi Adeoye"
tags: ["toolchain", "open-source", "rust", "engineering"]
---

## Introduction

[Wild Almonds](https://github.com/opeolluwa/almonds) is a developer productivity tool I’m building on [Tauri](https://tauri.app) — one place for snippets, tasks, reminders, and notes, instead of juggling Gists, To-Do lists, Notion, and my phone for reminders.

One of the earliest architectural concerns for Almonds was data replication. The model is straightforward:

1. Save data locally first.
2. Replicate to the cloud.
3. Sync across other devices.

That model shapes every technology decision, including the choice of SeaORM. In this article, I’ll highlight how SeaORM fits nicely into that data storage approach.

---

### Why not a JavaScript client database?

The problem is the data model. NoSQL stores give you flexibility upfront, but that flexibility quietly becomes a liability when your data needs to talk to a relational backend.

There are several offline-first JavaScript client databases today, including very ergonomic ones like **PouchDB**, **RxDB**, and **Dexie**, all of which I’ve interacted with at some point.

However, an important requirement for me was maintaining clear forward and backward compatibility for the data model from the beginning.

Managing NoSQL data can become complicated when it needs to integrate with relational databases, such as **PostgreSQL**, **MySQL**, or **Oracle**.

I also wanted a clear schema evolution from day one. With a document store, “migrations” often mean writing one-off scripts that run at startup and hope nothing breaks. With SeaORM and SQLite, migrations are first-class: versioned, ordered, and reversible.

The other factor was Tauri itself. Almonds is primarily a Rust application. Reaching for a JavaScript database would mean either bridging across the JS/Rust boundary for every query — which adds overhead and complexity — or running two separate data layers. Neither felt right. Using SeaORM keeps everything in Rust, which means one language, one type system, and no marshalling tax.

SeaORM fits cleanly here: SQLite locally, a structured relational model, and a schema that stays compatible with server databases. Everything lives in a shared [Kernel](https://github.com/opeolluwa/almonds/tree/master/kernel) library that a future mobile app can consume without model drift.

---

### Migrations

SeaORM lets you branch on the database backend mid-migration — useful when SQLite and Postgres handle schema changes differently. SQLite, for instance, doesn’t support ALTER TABLE to add foreign keys directly. The workaround is to create a new table, copy the data over, then drop the old one. SeaORM’s migration API exposes the backend at runtime, so you can handle each case explicitly rather than reaching for a lowest-common-denominator approach that works poorly for both:
```rust
if db_backend == DbBackend::Sqlite {
    // SQLite can't add FK constraints via ALTER TABLE
    // so we recreate the table with the new schema and migrate data
    manager.create_table(...).await?;
    db_connection.execute_unprepared("INSERT INTO reminders_new SELECT ...").await?;
    return Ok(());
}

// Postgres/MySQL support ALTER TABLE natively
manager.alter_table(...).await?;
manager.create_foreign_key(...).await?;
```
This pattern showed up immediately when I added workspaces to Almonds and needed to backfill a foreign key onto the reminders table. Without backend-aware migrations, I’d either have written two separate migration files and hoped they stayed in sync, or avoided the constraint entirely. Neither would have been acceptable long-term.

---

### Entity generation

sqlx is excellent, and I’ve used it extensively — but writing Rust bindings by hand means you’re one typo away from a runtime panic that a type mismatch should have caught at compile time. The more tables you have, the more that risk compounds.

SeaORM generates entities directly from a live database schema, so the Rust types are always an accurate reflection of what the database actually contains:
```rust
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "bookmark")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub identifier: Uuid,
    pub title: String,
    pub url: String,
    pub workspace_identifier: Option<Uuid>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}
```
Relations are generated too — the belongs_to and Related implementations are inferred from foreign keys in the schema, not written by hand. When the schema changes, you regenerate. The compiler then tells you exactly which call sites broke. That feedback loop matters: it turns a class of silent runtime bugs into loud compile-time errors.

---

### Seaography

Seaography generates a fully functional GraphQL server from SeaORM entities. One flag added to the code generation command is all it takes:
```bash
# Kernel: entities only
sea-orm-cli generate entity --database-url {{url}} --with-serde both -o src/entities

# Backend (Orchard): entities + GraphQL bindings
sea-orm-cli generate entity --database-url {{url}} --seaography -o src/entities
```
For Almonds, this was significant. The replication backend — the service that receives data from the client and writes it to the cloud database — needed a query interface that could express relationships between entities: workspaces containing reminders, bookmarks tagged across workspaces, and so on. Building that by hand with REST endpoints would have meant writing and maintaining route handlers for each relationship. With Seaography, those traversals come for free from the entity definitions that already exist.

It also means the API and the data model can’t drift from each other. The GraphQL schema is derived from the same entities that drive the local SQLite database. Add a column, regenerate, and both the ORM layer and the API layer reflect the change. That kind of tight coupling is usually a warning sign — but here it’s the point.

---

### Testing

Testing with SeaORM was notably straightforward compared to other database drivers I’ve used in both JavaScript and Rust. Tests run against a real SQLite instance — no mocking, no in-memory fakes, no test doubles pretending to be a database. Each test spins up an actual schema, runs the operation, and asserts against real query results. That means the tests are honest: if something breaks at the database layer, they catch it.

The setup_workspace helper handles the boilerplate of provisioning a workspace and returning the repository and metadata, so individual tests stay focused on the behavior under test:

```rust
#[tokio::test]
async fn test_store_with_workspace_recycle_bin() -> Result<(), KernelError> {
    let (meta, repo) = setup_workspace(get_recycle_bin_repository).await?;

    let payload = CreateRecycleBinEntry {
        item_id: Uuid::new_v4(),
        item_type: RecycleBinItemType::Note,
        payload: Paragraph(1..2).fake(),
        workspace_identifier: Some(meta.workspace_identifier),
    };

    let entry = repo.store(&payload, &Some(meta.clone())).await?;

    assert_eq!(entry.item_id, payload.item_id);
    assert_eq!(entry.workspace_identifier, Some(meta.workspace_identifier));

    Ok(())
}
```
The full suite covers creation, lookup by ID and type, pagination, and purging — both single entries and bulk. Because every test runs against SQLite offline, they’re fast, deterministic, and reproducible on any machine without a running database server. That matters for CI and for contributors who shouldn’t need infrastructure just to run cargo test.

---

### Role-based access control

SeaORM Pro includes built-in RBAC support. Almonds doesn’t need it today — it’s a single-user tool — but the workspace model already draws a boundary around data ownership that maps cleanly onto roles and permissions. If multi-user support ever becomes a goal, the path to RBAC is a configuration change, not a rewrite.

---

## Bottom line

The core value of an ORM is that it lets you express your data model once and stay out of the business of translating between representations. For Almonds, that matters at every layer: the local SQLite database, the cloud Postgres instance, the GraphQL replication API, and the shared Kernel library that ties them together.

I didn’t want to make application logic decisions based on which database engine was running underneath. Whether a query executes against SQLite on a laptop or Postgres in the cloud, the Rust code that calls it should look the same. SeaORM delivers that.

The migration system, entity generation, and Seaography integration aren’t independent features — they’re a coherent stack. Migrations keep the schema honest. Code generation keeps the types honest. Seaography keeps the API honest. For a local-first app with cloud replication, where data consistency across environments is the whole problem being solved, that coherence isn’t incidental. It’s what made SeaORM the right choice.

