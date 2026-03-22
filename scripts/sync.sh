
#!/usr/bin/env bash

echo "Enter the path to the Rust modules directory:"
read DIR

# validate directory
if [ ! -d "$DIR" ]; then
  echo "Directory does not exist."
  exit 1
fi

MODULE_FILE="$DIR/mod.rs"

# create mod.rs if it does not exist
touch "$MODULE_FILE"

echo "Scanning Rust files in $DIR ..."

tmp=$(mktemp)

# collect modules already declared
grep -oP 'pub mod \K[a-zA-Z0-9_]+' "$MODULE_FILE" 2>/dev/null > "$tmp"

# scan directory for rust files
for file in "$DIR"/*.rs; do
    name=$(basename "$file" .rs)

    # skip mod.rs itself
    if [[ "$name" == "mod" ]]; then
        continue
    fi

    echo "$name" >> "$tmp"
done

# rebuild mod.rs
{
    echo "//! Auto generated module list"
    sort -u "$tmp" | while read m; do
        echo "pub mod $m;"
    done
} > "$MODULE_FILE"

rm "$tmp"

echo "✔ Modules synced successfully in $MODULE_FILE"

