[working-directory:'orchard']
lint-orchard:
    sh ../scripts/format

[working-directory:'kernel']
lint-kernel:
    sh ../scripts/format


[working-directory:'almonds/src-tauri']
lint-tauri:
    sh ../../scripts/format


[working-directory:'almonds']
lint-almonds:
    npm run format
    npm run lint:fix

