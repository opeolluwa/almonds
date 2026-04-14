[working-directory:'server']
server-init url:
    seaography-cli -e ./src/entities  --output-dir . --database-url={{url}} --framework axum orchard


@server-logs:
    {{ DOCKER_CMD }} logs -f --tail='30' app


@watch-server:
    {{ DOCKER_CMD }} up -d 
    @just server-logs



[working-directory:'server']
test-server:
    echo "TBD"


[working-directory: 'server']
@migrate-server-add target:
	@sea-orm-cli migrate generate "{{target}}"

[working-directory: 'server']
@migrate-server-init:
	@sea-orm-cli migrate init


[working-directory: 'server']
@generate--server-entities url:
	sea-orm-cli generate entity \
		--database-url {{url}} \
		--with-serde both \
		--model-extra-attributes 'serde(rename_all="camelCase")' \
		-o src/entities

# Pull schema and generate entities + seaography bindings for PostgreSQL
[working-directory: 'server']
@generate-server-entities:
	sea-orm-cli generate entity \
		--database-url {{DATABASE_URL}} \
		--with-serde both \
		--model-extra-attributes 'serde(rename_all="camelCase")' \
		--seaography \
		-o src/entities

# Pull schema and generate entities + seaography bindings for MySQL (muted)
# [working-directory: 'server']
# @generate-server-entities-mysql:
# 	sea-orm-cli generate entity \
# 		--database-url {{DATABASE_URL}} \
# 		--with-serde both \
# 		--model-extra-attributes 'serde(rename_all="camelCase")' \
# 		--seaography \
# 		-o src/entities