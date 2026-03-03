[working-directory:'orchard']
lint-orchard:
    cargo fmt
    cargo sort 
    cargo group-imports --fix
    # cargo clippy -- -D warnings || exit 1

[working-directory:'kernel']
lint-kernel:
    cargo fmt
    cargo sort 
    cargo group-imports --fix
    # cargo clippy -- -D warnings || exit 1


[working-directory:'almonds/src-tauri']
lint-tauri:
    cargo fmt
    cargo sort 
    cargo group-imports --fix
    # cargo clippy -- -D warnings || exit 1


[working-directory:'almonds']
lint-almonds:
    npm run format
    npm run lint:fix

