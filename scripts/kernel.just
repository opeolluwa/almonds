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


