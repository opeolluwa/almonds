---
title: "Introducing Almonds — Your Personal Workspace, Reimagined"
description: "I'm excited to share why I'm building Almonds, a free, open source desktop app that brings your notes, tasks, bookmarks, and local AI together in one focused workspace."
date: 2026-02-23
author: "Adefemi Adeoye"
tags: ["announcement", "open-source", "productivity"]
---

## Why I'm building Almonds

Productivity tools have become fragmented. You have one app for notes, another for bookmarks, another for code snippets — and a monthly bill for each.

Almonds was born out of a simple idea: what if everything lived in one place, ran locally on your machine, and was completely free?

## Built on Rust, by design

Almonds is powered almost entirely by Rust.

At its core is a shared Rust library, dutifully called **kernel**, that handles the heavy lifting — data management, indexing, and cross-platform behavior. This core is reused across platforms to ensure consistency, reliability, and long-term maintainability.

I chose Rust not just because it's fast, but because it's safe. Memory safety, predictable performance, and strong concurrency guarantees allow Almonds to stay lightweight and responsive as your workspace grows.

The desktop app is built with Tauri, pairing a robust frontend with a native Rust backend. The result:

- Smaller binary size compared to traditional Electron apps  
- Lower memory footprint  
- Cross-platform support without duplicating core logic  

This architecture allows Almonds to stay fast, efficient, and sustainable long term.

## What's inside

Almonds ships with everything you need to stay focused:

- **Notes** — rich markdown editing, offline first  
- **Bookmarks** — save and tag links without the clutter  
- **Snippets** — your personal code library  
- **Todo** — lightweight task tracking  
- **Moodboard** — visual inspiration board  
- **Scratch Pad** — a blank canvas for quick thoughts  
- **AI** — local inference via Ollama, no API keys required  

## Roadmap

I currently have a working Proof of Concept (PoC) that validates the core architecture and local-first experience.

Next, I'm building the sync engine — **/orchard**.

Orchard is being designed specifically for:

- Self-hosting  
- Secure data synchronization  
- Reliable backup and restore  
- Multi-device support without vendor lock-in  

My goal is simple: your data should belong to you. Whether you run Orchard on your own server, a VPS, or locally, Almonds will remain flexible and fully under your control.

## Early days — and I need your help

Almonds is still in its early phase. Features are evolving, some are unimplemented, rough edges exist, and I’m actively shaping the roadmap.

If you encounter bugs, have ideas, or want to contribute — I’d genuinely appreciate your involvement.

- Report issues  
- Suggest improvements  
- Submit pull requests  
- Improve documentation  
- Help shape Orchard  

Open source works best when it's collaborative. Your feedback directly shapes the future of Almonds.

## Open source, MIT licensed

Every line of code is public. Fork it, self-host it, contribute to it. I believe great tools should be accessible to everyone.

[View on GitHub →](https://github.com/opeolluwa/almonds)