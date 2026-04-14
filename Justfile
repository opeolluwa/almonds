import 'scripts/kernel.just'
import 'scripts/misc.just'
import 'scripts/desktop.just'
import 'scripts/lint.just'
import 'scripts/test.just'
import 'scripts/clean.just'
import 'scripts/server.just'
import 'scripts/website.just'
import 'scripts/release.just'
import 'scripts/mobile.just'


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
		just lint-desktop
		just lint-kernel
		just lint-server
		just lint-desktop-tauri
	else
		just lint-{{target}}
	fi


test target:
	#!/usr/bin/env bash
	if [ "{{target}}" = "all" ]; then
		just test-desktop
		just test-kernel
		just test-server
		just test-desktop-tauri
	else
		just test-{{target}}
	fi

[working-directory:'kernel']
@migrate-run:
	DATABASE_URL={{DB_PATH}} sea-orm-cli  migrate up


[working-directory:'.']
release target:
	@just release-{{target}}



sync:
	sh scripts/sync.sh



