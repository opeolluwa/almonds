[working-directory: 'kernel']
watch-kernel:
	cargo watch -qcx check

[working-directory: 'kernel']
@migrate-add target:
	@sea-orm-cli migrate generate "{{target}}"

[working-directory: 'kernel']
@migrate-init:
	@sea-orm-cli migrate init

[working-directory: 'kernel']
@migrate-run:
	DATABASE_URL=sqlite://../test.sqlite sea-orm-cli migrate up

[working-directory: 'kernel']
@generate-entities url:
	sea-orm-cli generate entity \
		--database-url {{url}} \
		--with-serde both \
		--model-extra-attributes 'serde(rename_all="camelCase")' \
		-o src/entities

[working-directory: 'kernel/graphql']
@generate-graphql-bindings:
	sea-orm-cli generate entity \
		--database-url sqlite://../test.sqlite \
		-o src/entities \
		--seaography
	seaography-cli \
		--entities src/entities \
		--database-url sqlite://../test.sqlite \
		kernel-graphql
	cargo run

db-pull:
	just migrate-run
	just generate-entities sqlite://../test.sqlite?mode=rwc
