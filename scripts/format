#!/usr/bin/env bash
set -e

# Ensure rustfmt exists
if ! rustup component list --installed | grep -q rustfmt; then
  echo "Installing rustfmt..."
  rustup component add rustfmt
fi

# Ensure cargo-sort exists
if ! command -v cargo-sort >/dev/null 2>&1; then
  echo "Installing cargo-sort..."
  cargo install cargo-sort
fi

# Ensure cargo-clippy exists
if ! rustup component list --installed | grep -q clippy; then
  echo "Installing clippy..."
  rustup component add clippy
fi

echo "Running formatting and checks..."

cargo fmt
cargo sort

cargo group-imports --fix
# cargo clippy -- -D warnings || exit 1