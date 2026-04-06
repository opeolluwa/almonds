import 'scripts/kernel.just'
import 'scripts/misc.just'
import 'scripts/almond.just'
import 'scripts/lint.just'
import 'scripts/build.just'
import 'scripts/test.just'
import 'scripts/clean.just'
import 'scripts/android.just'
import 'scripts/orchard.just'
import 'scripts/docs.just'
import 'scripts/grove.just'
import 'scripts/release.just'


DB_PATH := "sqlite://../../test.sqlite?mode=rwc"
DOCKER_CMD := "docker compose -f docker-compose.yaml"
POSTGRES_URL := "postgres://almond:almond@localhost:5433/almond"
MYSQL_URL    := "mysql://almond:almond@localhost:3307/almond"
SQLITE_URL   := "sqlite://./almond.db?mode=rwc"
DATABASE_URL :="postgres://orchard:orchard@localhost:6543/orchard"

set dotenv-required := true
set dotenv-load := true
set dotenv-path := ".env"
set export := true

alias w := watch
alias b := build
alias cfg := configure

configure:
	just install-dependencies
	just create-kernel-test-file
	just install-frontend-dependencies
	chmod +x scripts/release.sh

watch target:
	just watch-{{target}}

build target:
	just build-{{target}}

lint target:
	#!/usr/bin/env bash
	if [ "{{target}}" = "all" ]; then
		just lint-almonds
		just lint-kernel
		just lint-orchard
		just lint-tauri
	else
		just lint-{{target}}
	fi


test target:
	#!/usr/bin/env bash
	if [ "{{target}}" = "all" ]; then
		just test-almonds
		just test-kernel
		just test-orchard
		just test-tauri
	else
		just test-{{target}}
	fi

[working-directory:'kernel']
@migrate-run:
	DATABASE_URL={{DB_PATH}} sea-orm-cli  migrate up


[working-directory:'.']
release target:
	@just release-{{target}}



@server-logs:
    {{ DOCKER_CMD }} logs -f --tail='30' app


@server-dev:
    {{ DOCKER_CMD }} up -d 
    @just server-logs


[working-directory:'kernel']
gph-pull:
	graphql-client generate  --schema-path .graphql/schema.graphql .graphql/sync_queue_query.graphql --output-directory src/contracts 



sync:
	sh scripts/sync.sh



