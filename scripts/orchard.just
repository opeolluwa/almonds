[working-directory:'orchard']
orchard-init url:
    seaography-cli -e ./src/entities  --output-dir . --database-url={{url}} --framework axum orchard


[working-directory:'orchard']
watch-orchard:
    cargo watch -qcx run 