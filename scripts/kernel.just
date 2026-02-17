
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
    sea-orm-cli generate entity --database-url={{url}} --with-serde both  --model-extra-attributes 'serde(rename_all="camelCase")' -o src/entities

[working-directory: 'kernel']
db-pull:
    DATABASE_URL=sqlite://test.sqlite sea-orm-cli migrate up
    just generate-entities sqlite://test.sqlite