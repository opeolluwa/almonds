[working-directory: 'console']
watch-frontend:
    npm run dev

[working-directory: 'console']
watch-frontend-mobile:
   NUXT_APP_TARGET=mobile npm run dev

    
[working-directory: 'console']
watch-tauri:
    npm run tauri dev

[working-directory: 'console']
watch-mobile:
    ./src-tauri/gen/android/gradlew --stop
    NUXT_APP_TARGET=mobile npm run tauri android dev 

[working-directory: 'console']
watch-mobile-device HOST:
    TAURI_DEV_HOST={{HOST}} npm run tauri android dev

[working-directory: 'console']
watch-ios:
   npm run tauri ios dev

[working-directory: 'console']
watch-ios-device HOST:
    TAURI_DEV_HOST={{HOST}} npm run tauri ios dev

[working-directory: 'console']
watch-console:
    NUXT_APP_TARGET=desktop npm run dev & npm run tauri dev

[working-directory: 'console']
build-console:
    npm run tauri build

[working-directory: 'console']
install-frontend-dependencies:
    npm install

[working-directory: 'console']
test-console:
    echo "TBD"

[working-directory: 'console/src-tauri']
test-tauri:
    echo "TBD"