@android target:
	@echo "Android: running {{target}}"
	@just "android-{{target}}"


[working-directory: 'app']
@android-build-apk:
	#!/usr/bin/env sh
	export ENVIRONMENT=production
	npm run tauri android build -- --apk true --verbose


[working-directory: 'app']
@android-build-aab:
	npm run tauri android build -- --aab


[working-directory: 'app']
@android-watch:
	#!/usr/bin/env sh
	export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
	export ANDROID_HOME="$HOME/Library/Android/sdk"
	export NDK_VERSION=$(ls -1 "$ANDROID_HOME/ndk" | tail -n 1)
	export NDK_HOME="$ANDROID_HOME/ndk/$NDK_VERSION"
	export NODE_ENV="development"
	npm run tauri android dev 


[working-directory: 'almonds']
@android-init:
	#!/usr/bin/env sh
	export JAVA_HOME="/Applications/Android Studio.app/Contents/jbr/Contents/Home"
	export ANDROID_HOME="$HOME/Library/Android/sdk"
	export NDK_VERSION=$(ls -1 "$ANDROID_HOME/ndk" | tail -n 1)
	export NDK_HOME="$ANDROID_HOME/ndk/$NDK_VERSION"
	export NODE_ENV="development"
	npm run tauri android init


[working-directory: 'app']
@android-keychain target:
	keytool -genkey -v -keystore {{target}}-release.jks -keyalg RSA -keysize 2048 -validity 10000 -alias {{target}}