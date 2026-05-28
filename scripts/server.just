# Build the production Docker image for the app server.
# Usage: just build app  (via the root `build` alias)
#        just build-app  (direct)
#        just build-app my-registry.io/orchard  (custom tag prefix)
build-app tag="orchard":
    DOCKER_BUILDKIT=1 docker build \
        --file docker/prod/Dockerfile.app \
        --tag {{tag}}:latest \
        --tag {{tag}}:$(git rev-parse --short HEAD) \
        .

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


# Pull schema and generate entities + seaography bindings for PostgreSQL
[working-directory: 'server']
@generate-server-entities:
	sea-orm-cli generate entity \
		--database-url {{DATABASE_URL}} \
		--with-serde both \
		--model-extra-attributes 'serde(rename_all="camelCase")' \
		--tables users,one_time_passwords,countries,wait_list,revoked_tokens \
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