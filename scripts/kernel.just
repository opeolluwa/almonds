[working-directory: 'kernel']
watch-kernel:
	cargo watch -qcx check

# --- Docker -----------------------------------------------------------

# Start postgres and mysql containers (kernel/docker/docker-compose.yml)
[working-directory: 'kernel']
db-up:
	docker compose -f docker/docker-compose.yml up -d

# Stop and remove containers (data volumes are preserved)
[working-directory: 'kernel']
db-down:
	docker compose -f docker/docker-compose.yml down

# Tear down containers AND delete volumes
[working-directory: 'kernel']
db-clean:
	docker compose -f docker/docker-compose.yml down -v

# --- Migrations -------------------------------------------------------



[working-directory: 'kernel']
@migrate-add target:
	@sea-orm-cli migrate generate "{{target}}"

[working-directory: 'kernel']
@migrate-init:
	@sea-orm-cli migrate init

# Run pending migrations against postgres (docker)
[working-directory: 'kernel']
@migrate-up-postgres:
	sea-orm-cli migrate up --database-url {{POSTGRES_URL}}


# Run pending migrations against postgres (docker)
[working-directory: 'server']
@migrate-up-server:
	sea-orm-cli migrate up --database-url {{DATABASE_URL}}

# Run pending migrations against sqlite
[working-directory: 'kernel']
@migrate-up-sqlite:
	sea-orm-cli migrate up --database-url {{SQLITE_URL}}

# Run pending migrations against mysql (docker)
# [working-directory: 'kernel']
# @migrate-up-mysql:
# 	sea-orm-cli migrate up --database-url {{MYSQL_URL}}

# Run pending migrations against all active databases
[working-directory: 'kernel']
@migrate-up:
	just migrate-up-postgres
	just migrate-up-sqlite
	# just migrate-up-mysql

# --- Entity generation ------------------------------------------------

[working-directory: 'kernel']
@generate-entities url:
	sea-orm-cli generate entity \
		--database-url {{url}} \
		--with-serde both \
		--model-extra-attributes 'serde(rename_all="camelCase")' \
		-o src/entities

# Pull schema and generate entities for SQLite (feature: sqlite)
[working-directory: 'kernel']
@db-pull-sqlite url:
	sea-orm-cli generate entity \
		--database-url {{url}} \
		--with-serde both \
		--model-extra-attributes 'serde(rename_all="camelCase")' \
		-o src/entities/sqlite

# Pull schema and generate entities for PostgreSQL using docker defaults (feature: postgres)
[working-directory: 'kernel']
@db-pull-postgres:
	sea-orm-cli generate entity \
		--database-url {{POSTGRES_URL}} \
		--with-serde both \
		--model-extra-attributes 'serde(rename_all="camelCase")' \
		-o src/entities/postgres

# Pull schema and generate entities for MySQL using docker defaults (feature: mysql)
[working-directory: 'kernel']
@db-pull-mysql:
	sea-orm-cli generate entity \
		--database-url {{MYSQL_URL}} \
		--with-serde both \
		--model-extra-attributes 'serde(rename_all="camelCase")' \
		-o src/entities/mysql

# Generate entities for all backends.
# Starts docker, waits for postgres, runs migrations (postgres + sqlite), then generates entities.
# Pass a custom sqlite URL or omit to use the default (sqlite://./almond.db?mode=rwc).
#   just db-pull
#   just db-pull sqlite://./custom.db
[working-directory: 'kernel']
db-pull sqlite_url=SQLITE_URL:
	just db-up
	@echo "Waiting for PostgreSQL to be ready..."
	@until pg_isready -h localhost -p 5433 -U almond -q; do sleep 1; done
	just migrate-up
	just db-pull-sqlite {{sqlite_url}}
	just db-pull-postgres
	just db-pull-server

[working-directory: 'server']
@generate-graphql-bindings url:
	sea-orm-cli generate entity \
		--database-url {{url}} \
		-o src/entities \
		--seaography


[working-directory:'kernel']
test-kernel:
    cargo test

db-pull-server:
	just migrate-up-server
	just generate-server-entities



	
[working-directory:'kernel']
gph-pull:
		graphql-client generate  --schema-path .graphql/sync_queue_schema.graphql .graphql/sync_queue_query.graphql --output-directory src
