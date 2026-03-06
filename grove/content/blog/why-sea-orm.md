---
title: "Why SeaORM over other Rust database options?"
description: ""
date: 2026-03-08
author: "Adefemi Adeoye"
tags: ["toolchain", "open-source", "rust", "engineering"]
---

---

## Overview

A few weeks ago, I made an announcement about [Wild Almonds](https://opeolluwa.github.io/almonds). This post reflects on some of the decisions that influenced the choice of technologies behind the project, starting with SeaORM.

## Introduction

Needless to say, the choice of technology can greatly influence the performance and long-term viability of a product across several parameters, including size, speed, and security.

Rust was an easy choice because it checks many of these boxes. Its performance, memory safety guarantees, and strong ecosystem make it a compelling foundation for building reliable applications.

One of the earliest architectural concerns when planning Wild Almonds was **data replication**, while the sync server is still a WIP, it is imperative that things are done right from the ground up.

There are quite a number of offline-first client databases available today, including PouchDB, RxDB, Dexi and many more. However, it was important for me to streamline both forward and backward compatibility of the data model from the beginning. Managing NoSQL databases can quickly become murky, especially when the data eventually needs to integrate with relational databases such as Oracle, MySQL, or Postgres.

Amidst these considerations, SeaORM fits nicely into the architecture. It allows me to use SQLite locally for the client database while keeping a structured relational model. Combined with Seaography, this also makes it possible to replicate or expose the data to a cloud database in a consistent and maintainable way.

What is more important is SeaORM entities can be bootstrapped as GraphQl types, this makes it easier, considering it's all driven by codegen here and there.

In the following section I'll highlight feature that resonated strongly with me starting with other database client in the Rust ecosystem

## SeaORM vs SQLx vs Diesel

One interesting detail is that SeaORM is built on top of the SQLx driver. In many ways, it inherits the performance and async capabilities that SQLx provides while adding a higher level of abstraction.

Using SQLx directly is powerful, but it would require writing and maintaining a significant amount of manual query logic. While that approach works well for smaller systems, it can become tedious as the application grows and the data model becomes more complex.

The main advantage of an ORM is abstraction. Instead of writing raw SQL for every interaction, the data model is expressed through entities and relationships. This allows the application logic to remain largely independent of the underlying database engine.

For Wild Almonds, this flexibility is important. I do not want the application logic to depend heavily on whether the database is SQLite, MySQL, Postgres, or another supported backend. SeaORM provides a clean way to model the data while still allowing the project to remain portable across multiple database systems.

Diesel was another option worth considering. However, Diesel’s synchronous design and heavier compile-time constraints can make it less convenient for applications that rely heavily on asynchronous workflows.

SeaORM strikes a balance between these approaches: it provides the abstraction of an ORM while still leveraging the async capabilities and performance of SQLx under the hood.

### SeaORM migration

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
                        // ... more lines removed
                        .to_owned(),
                )
                .await?;

            db_connection
                .execute_unprepared(
                    r#"
                    INSERT INTO "reminders_new" ("identifier", "title", "description", "recurring", "recurrence_rule", "alarm_sound", "remind_at", "created_at", "updated_at")
                    --- lines removed --
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


```

### SeaORm entity generation

### Using Seaography

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

### Testing

## Conclusion

SeaORM helped unify a lot of code that would have otherwise become platform specific 
