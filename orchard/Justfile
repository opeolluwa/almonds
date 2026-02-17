alias w:= watch
alias k:= kill
alias b:= build
alias l:= logs
alias install := install-dependencies
alias r:= restart


set dotenv-required
set dotenv-load := true
set dotenv-path := "./.env.local"
set export :=  true

@default: 
    @just --list --list-heading $'Available commands\n'

[doc('Install the application dependencies')]
@install-dependencies: 
    @echo "Installing backend dependencies"


@fmt:
    cargo fmt && cargo fix 

@watch: 
    docker compose up -d 
    @just l

@logs:
    docker compose logs -f --tail='30' app

build:
    cargo run build --release 

@kill:  
    docker compose down -v 

@restart:
    @just kill
    @just watch