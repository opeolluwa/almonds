# Database

Almonds uses the Tauri SQL plugin backed by SQLite. Migrations are managed with SeaORM.

## Commands

```bash
just migrate-init            # initialize SeaORM migrations
just migrate-add <name>      # generate a new migration file
just migrate-run             # apply pending migrations
just db-pull                 # migrate, generate entities, and regenerate GraphQL bindings
```
