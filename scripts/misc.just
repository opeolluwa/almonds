install-dependencies:
	cargo install cargo-watch 
	cargo install sea-orm-cli
	cargo install cargo-sort 
	cargo install cargo-group-imports 



generate-bindings:
    cd server && cargo test 
    cd app/src-tauri && cargo test 
    rm -rf app/app/bindings 
    cp -r server/bindings app/app/bindings 
    cp -r app/src-tauri/bindings/* app/app/bindings


create-kernel-test-file:
    touch kernel/test.sqlite