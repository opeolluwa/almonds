---
title: "Why SeaORM over JavaScript client database options?"
description: ""
date: 2026-03-11
author: "Adefemi Adeoye"
tags: ["toolchain", "open-source", "rust", "engineering"]
---

## Overview

A few weeks ago, I announced [Wild Almonds](https://opeolluwa.github.io/almonds). In this post, I want to reflect on some of the technology decisions behind the project, starting with SeaORM.

In case you missed it: **Wild Almonds** (or simply **Almonds**) is a developer productivity tool I'm building off the [Tauri](https://tauri.app) framework.

I started building it because I got tired of juggling multiple tools — saving snippets in GitHub Gists, planning my day with Microsoft To Do, keeping reminders on my phone, taking notes in Notion, and a dozen other small apps.

I needed one place for everything — a single source of truth for how I work as a developer.

That's what Almonds is becoming.

That said, the choice of technology can greatly influence the performance and long-term viability of a product across several parameters, including size, speed, and security.

Rust was an easy choice because it checks many of these boxes. Its performance, memory safety guarantees, strong ecosystem, and overall developer experience make it a compelling foundation for building reliable applications.

One of the earliest architectural concerns for Almonds was **data replication**. The model is straightforward:

1. Save data locally first
2. Replicate to the cloud
3. Sync across other devices

In this article, I'll highlight how **SeaORM** fits nicely into that data storage approach.

## Introduction

There are several offline-first JavaScript client databases today, including very ergonomic ones like **PouchDB**, **RxDB**, and **Dexie**, all of which I’ve interacted with at some point.

However, an important requirement for me was maintaining clear forward and backward compatibility for the data model from the beginning.

Managing NoSQL data can become complicated when the data eventually needs to integrate with relational databases such as **Postgres**, **MySQL**, or **Oracle**.

SeaORM fits well into this architecture because it allows Almonds to:

- Use **SQLite locally** for the client database
- Maintain a **structured relational data model**
- Keep the same schema compatible with server databases

Combined with **Seaography** and other tooling, this also makes it possible to replicate or expose the data to a cloud database in a consistent and maintainable way.

All these are packaged in to a single shared library [Kernel](https://github.com/opeolluwa/almonds/tree/master/kernel) which can be used in future for the mobile application while still maintaining up-to-date data model

In the following sections, I'll highlight some features that resonated strongly with me.

---

## SeaORM migrations

SeaORM provides a very intuitive way to opt out of generic scripting and do some database specific implementation. I found this very helpful while adding workspaces to almonds

```rust
use sea_orm_migration::{prelude::*, schema::*, sea_orm::DbBackend};

use crate::{
    m20260221_065202_create_reminder_table::Reminder,
    m20260224_214545_create_workspaces::Workspaces,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();
        let db_connection = manager.get_connection();

        if db_backend == DbBackend::Sqlite {
            manager
                .create_table(
                    Table::create()
                        .table("reminders_new")
                        .if_not_exists()
                        .col(pk_uuid(Reminder::Identifier))
                        // ...
                        .to_owned(),
                )
                .await?;

            db_connection
                .execute_unprepared(
                    r#"
                    INSERT INTO "reminders_new"
                    ("identifier", "title", "description", "recurring",
                     "recurrence_rule", "alarm_sound", "remind_at",
                     "created_at", "updated_at")
                    -- lines removed --
                    "#,
                )
                .await?;

            return Ok(());
        }

        manager
            .alter_table(
                Table::alter()
                    .table(Reminder::Table)
                    .add_column(ColumnDef::new("workspace_identifier").uuid())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_reminder_workspace_identifier")
                    .from(Reminder::Table, "workspace_identifier")
                    .to(Workspaces::Table, "identifier")
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }
}
```

---

## SeaORM entity generation

I've used [sqlx](https://crates.io/crates/sqlx) extensively in personal project and at work. Writing Rust bindings for the database tables can be tricking at times.  SeaORM provides tooling that reduces boilerplate by generating entities directly from a database schema.

This keeps the data model synchronized with the database and reduces the risk of manual inconsistencies.

```rust
//! `SeaORM` Entity, @generated by sea-orm-codegen 2.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "bookmark")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub identifier: Uuid,
    pub title: String,
    pub url: String,
    #[sea_orm(column_type = "Text")]
    pub tag: String,
    pub workspace_identifier: Option<Uuid>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::workspaces::Entity",
        from = "Column::WorkspaceIdentifier",
        to = "super::workspaces::Column::Identifier",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Workspaces,
}

impl Related<super::workspaces::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Workspaces.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
```

---

## Using Seaography
SeaORM makes it easy to build a GraphQL web server around entities, I found this very instrumental while scratching out my data replication backend — 
```sh
[working-directory: 'kernel']
@generate-entities url:
	sea-orm-cli generate entity \
		--database-url {{url}} \
		--with-serde both \
		--model-extra-attributes 'serde(rename_all="camelCase")' \
		-o src/entities

[working-directory: 'orchard']
@generate-graphql-bindings url:
	sea-orm-cli generate entity \
		--database-url {{url}} \
		-o src/entities \
		--seaography
```

---


### Role based access control 
Finally, SeaORM provides the ease of integrating [Role based access control](https://www.sea-ql.org/SeaORM/docs/sea-orm-pro/role-based-access-control/#overview-of-seaorm-rbac), while there are no current intention for this in Almonds I can switch up this with relative ease in due time 

## Conclusion

One of the main advantages of an ORM is **abstraction through structured models and code generation**.

Instead of writing raw SQL for every interaction, the data model is expressed through entities and relationships. This allows the application logic to remain largely independent of the underlying database engine.

For Almonds, this flexibility is important. I don’t want the application logic to depend on whether the database is **SQLite, MySQL, Postgres, or another supported backend**.

SeaORM provides a clean way to model the data while keeping the project portable across multiple database systems.

Ultimately, it helped unify logic that would otherwise become **database-specific or platform-specific**, which is especially valuable for an application designed around **local-first data with cloud replication**.
