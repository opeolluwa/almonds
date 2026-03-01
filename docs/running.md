# Running

## Prerequisites

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://www.rust-lang.org/tools/install)
- [just](https://github.com/casey/just)
- [Ollama](https://ollama.com/) _(optional — required for AI features)_

## Setup

```bash
git clone https://github.com/opeolluwa/almonds.git
cd almonds
cp .env.example .env
just configure
```

`just configure` installs Cargo tools (`cargo-watch`, `cargo-sort`, `cargo-group-imports`, `sea-orm-cli`, `seaography-cli`) and creates the local database file.

## System Dependencies

### Linux (Debian / Ubuntu)

Before building or running locally, install the required system libraries:

```bash
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf \
  libasound2-dev
```

## Development

```bash
just watch almond     # frontend + Tauri (recommended)
just watch frontend   # Nuxt dev server only
just watch tauri      # Tauri desktop only
just watch kernel     # watch the kernel crate
```

## Android

```bash
just android init     # initialize Android project
just android watch    # run on device/emulator
```
