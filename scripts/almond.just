


[working-directory:'almonds']
watch-frontend:
    npm run dev

[working-directory:'almonds']
watch-tauri:
    npm run tauri dev

[working-directory:'almonds']
watch-almond:
    npm run dev & npm run tauri dev


[working-directory:'almonds']
build-almond:
    npm run tauri build
