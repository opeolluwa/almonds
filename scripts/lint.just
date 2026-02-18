[working-directory:'orchard']
lint-orchard:
    cargo fmt
    cargo sort -w
    cargo group-imports --fix

[working-directory:'kernel']
lint-kernel:
    cargo fmt
    cargo sort -w
    cargo group-imports --fix

[working-directory:'almonds/src-tauri']
lint-tauri:
    cargo fmt
    cargo sort -w
    cargo group-imports --fix

[working-directory:'almonds']
lint-almonds:
    npm run format
    npm run lint:fix

