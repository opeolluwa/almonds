[working-directory:'orchard']
orchard-init url:
    seaography-cli -e ./src/entities  --output-dir . --database-url={{url}} --framework axum orchard


[working-directory:'orchard']
watch-orchard:
    cargo watch -qcx run 


[working-directory:'orchard']
test-orchard:
    echo "TBD"


[working-directory: 'orchard']
@migrate-orchard-add target:
	@sea-orm-cli migrate generate "{{target}}"

[working-directory: 'orchard']
@migrate-orchard-init:
	@sea-orm-cli migrate init


[working-directory: 'orchard']
@generate--orchard-entities url:
	sea-orm-cli generate entity \
		--database-url {{url}} \
		--with-serde both \
		--model-extra-attributes 'serde(rename_all="camelCase")' \
		-o src/entities