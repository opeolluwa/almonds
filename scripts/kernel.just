
[working-directory:'kernel']
watch-kernel: 
    cargo watch -qcx check

[working-directory('kernel')]
@migrate-add target: 
    @sea-orm-cli migrate generate "{{target}}"

[working-directory('kernel')]
@migrate-init:
     @sea-orm-cli migrate init 

[working-directory: 'kernel']
@generate-entities url:
    sea-orm-cli generate entity --database-url={{url}} --with-serde both  --model-extra-attributes 'serde(rename_all="camelCase")' -o src/entities --seaography  seaography-cli --database-url={{url}}  

[working-directory: 'kernel']
@generate-graphql-bindings url:
	seaography-cli \
		--entities src/entities \
		--database-url {{url}} \
		graphql && \
	cargo run


    
[working-directory: 'kernel']
db-pull:
    DATABASE_URL=sqlite://test.sqlite sea-orm-cli migrate up
    just generate-entities sqlite://test.sqlite
    just generate-graphql-bindings sqlite://test.sqlite
    cargo test