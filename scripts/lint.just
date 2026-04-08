[working-directory:'server']
lint-server:
    sh ../scripts/format

[working-directory:'kernel']
lint-kernel:
    sh ../scripts/format


[working-directory:'desktop/src-tauri']
lint-desktop-tauri:
    sh ../../scripts/format


[working-directory:'desktop']
lint-desktop:
    npm run format
    npm run lint:fix

