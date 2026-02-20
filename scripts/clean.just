[working-directory: 'almonds']
clean-almonds:
    rm -rf node_modules dist .nuxt
    cd src-tauri && cargo clean

[working-directory: 'kernel']
clean-kernel:
    cargo clean

[working-directory: 'orchard']
clean-orchard:
    cargo clean

clean target:
    #!/usr/bin/env sh
    echo "Cleaning {{target}} build assets"
    if [ "{{target}}" = "almonds" ]; then
        just clean-almonds
    elif [ "{{target}}" = "kernel" ]; then
        just clean-kernel
    elif [ "{{target}}" = "orchard" ]; then
        just clean-orchard
    elif [ "{{target}}" = "all" ]; then
        just clean-almonds
        just clean-kernel
        just clean-orchard
    else
        echo "Invalid target '{{target}}'. Use one of: almonds, kernel, orchard, all"
    fi
