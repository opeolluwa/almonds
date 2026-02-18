# Almonds

A personal productivity desktop app built with Tauri, Nuxt 4, and Vue 3. Almonds brings your notes, snippets, bookmarks, tasks, and AI conversations into one unified workspace.

---

## Features

- **Notes** — Create and manage personal notes
- **Bookmarks** — Save and organize web links
- **Snippets** — Store reusable code with syntax highlighting
- **Todo** — Track tasks and to-dos
- **Ollama** — Chat with local AI models via Ollama
- **Scratch Pad** — A quick freeform writing space
- **Moodboard** — Visual inspiration board
- **Settings** — Customize the app to your preference

---

## Tech Stack

| Layer | Technology |
|---|---|
| Framework | Nuxt 4 + Vue 3 |
| Desktop | Tauri v2 |
| UI | @nuxt/ui + Tailwind CSS |
| State | Pinia + persisted state |
| Database | Tauri SQL plugin |
| i18n | @nuxtjs/i18n |
| Linting | ESLint + Prettier |

---

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://www.rust-lang.org/tools/install) (for Tauri)
- [Ollama](https://ollama.com/) (optional, required for AI features)

### Installation

```bash
# Clone the repository
git clone https://github.com/opeolluwa/almonds.git
cd almonds

# Install dependencies
npm install
```

### Development

```bash
# Start the Nuxt dev server (web only)
npm run dev

# Start the Tauri desktop app in dev mode
npm run tauri dev
```

### Build

```bash
# Build the Nuxt app
npm run build

# Build the Tauri desktop app
npm run tauri build
```

### Other Commands

```bash
npm run preview      # Preview production build
npm run lint         # Run ESLint
npm run lint:fix     # Auto-fix lint issues
npm run format       # Format code with Prettier
```

---

## Roadmap

Planned features and improvements, based on the current navigation structure:

### Core Modules

- [ ] **Notes** — rich text editing, tagging, search
- [ ] **Bookmarks** — import/export, folder organization, favicon fetch
- [ ] **Snippets** — language detection, copy-to-clipboard, tagging
- [ ] **Todo** — due dates, priorities, recurring tasks
- [ ] **Scratch Pad** — auto-save, markdown preview
- [ ] **Moodboard** — drag-and-drop image layout, image upload

### AI / Ollama

- [ ] Persistent conversation history
- [ ] Multi-model support
- [ ] Context-aware responses using workspace content

### App-wide

- [ ] Global search across all modules
- [ ] Notifications centre
- [ ] Data export (JSON / Markdown)
- [ ] Sync across devices
- [ ] Plugin/extension system

---

## License

MIT License — see [LICENSE](LICENSE) for details.
