


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
watch-almonds:
    npm run dev & npm run tauri dev

[working-directory:'almonds']
build-almond:
    npm run tauri build


[working-directory:'almonds']
install-frontend-dependencies:
    npm install


[working-directory:'almonds']
test-almonds:
    echo "TBD"



[working-directory:'almonds/src-tauri']
test-tauri:
    echo "TBD"