[working-directory: 'desktop']
clean-desktop:
    rm -rf node_modules dist .nuxt
    cd src-tauri && cargo clean

[working-directory: 'kernel']
clean-kernel:
    cargo clean

[working-directory: 'server']
clean-server:
    cargo clean

clean target:
    #!/usr/bin/env sh
    echo "Cleaning {{target}} build assets"
    if [ "{{target}}" = "desktop" ]; then
        just clean-desktop
    elif [ "{{target}}" = "kernel" ]; then
        just clean-kernel
    elif [ "{{target}}" = "server" ]; then
        just clean-server
    elif [ "{{target}}" = "all" ]; then
        just clean-desktop
        just clean-kernel
        just clean-server
    else
        echo "Invalid target '{{target}}'. Use one of: desktop, kernel, server, all"
    fi
