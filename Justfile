alias w := watch-desktop
alias b := build-desktop


[working-directory:'almonds']
watch-frontend:
    npm run dev

[working-directory:'almonds']
watch-tauri:
    npm run tauri dev

[working-directory:'almonds']
watch-desktop:
    npm run dev & npm run tauri dev


[working-directory:'almonds']
build-desktop:
    npm run tauri build
