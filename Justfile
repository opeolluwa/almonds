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


DB_PATH := "sqlite:://../../test.sqlite?mode=rwc"
alias w := watch
alias b := build
alias cfg := configure

configure:
	just install-dependencies
	just create-kernel-test-file

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


[working-directory:'kernel']
@migrate-run:
	DATABASE_URL={{DB_PATH}} sea-orm-cli  migrate up

db-pull:
	just migrate-run
	just generate-entities {{DB_PATH}}
	just generate-graphql-bindings {{DB_PATH}}
