


[working-directory:'desktop']
watch-frontend:
    npm run dev

[working-directory:'desktop']
watch-tauri:
    npm run tauri dev

[working-directory:'desktop']
watch-desktop:
    npm run dev & npm run tauri dev

[working-directory:'desktop']
build-desktop:
    npm run tauri build


[working-directory:'desktop']
install-frontend-dependencies:
    npm install


[working-directory:'desktop']
test-desktop:
    echo "TBD"



[working-directory:'desktop/src-tauri']
test-tauri:
    echo "TBD"