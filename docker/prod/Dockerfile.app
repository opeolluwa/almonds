# syntax=docker/dockerfile:1.4

# ── Builder ────────────────────────────────────────────────────────────────────
FROM rust:1.93-slim-bookworm AS builder

ENV CARGO_TARGET_DIR=/cargo-target

RUN set -e && \
    apt-get update -q && \
    apt-get install -yq --no-install-recommends \
        libssl-dev \
        pkg-config \
        clang \
        libclang-dev \
        zlib1g-dev \
        llvm-dev \
        build-essential && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the full workspace (server depends on kernel via relative path ../kernel)
COPY kernel ./kernel
COPY server ./server

# Compile in release mode.
# BuildKit cache mounts keep the registry and target dir across rebuilds
# so only changed crates are recompiled.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/cargo-target \
    cargo build --release --manifest-path server/Cargo.toml --bin orchard && \
    # Copy the binary out of the cache mount before the layer is sealed
    cp /cargo-target/release/orchard /orchard

# ── Runtime ────────────────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN set -e && \
    apt-get update -q && \
    apt-get install -yq --no-install-recommends \
        # lettre (tokio1-native-tls) and OpenSSL-linked crates need these at runtime
        libssl3 \
        libgcc-s1 \
        ca-certificates \
        # lightweight curl for HEALTHCHECK
        curl && \
    rm -rf /var/lib/apt/lists/* && \
    groupadd --system --gid 1001 orchard && \
    useradd  --system --uid 1001 --gid orchard --no-create-home orchard

WORKDIR /app

COPY --from=builder --chown=orchard:orchard /orchard ./orchard

USER orchard

EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=5s --start-period=15s --retries=3 \
    CMD curl -fsS http://localhost:8000/health || exit 1

ENTRYPOINT ["./orchard"]
