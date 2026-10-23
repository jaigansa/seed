# Seed (v0.2.0)

> A modern, keyboard-first Terminal Workspace Manager, Project Scaffolder, and Health Auditor written in pure Rust.

Seed replaces cluttered working directories with an organized, 4-pillar workspace architecture, scaffolds multi-disciplinary software and creative projects in seconds, audits project health, and provides a keyboard-driven terminal dashboard.

---

## Key Features

- **Unified Dashboard**: Fast, keyboard-first TUI dashboard with live search, project metadata cards, and preview inspectors.
- **Developer Actions**: 
  - **`[I]` / `[V]`**: Open selected project in your configured `$EDITOR` or graphical IDE (`code`, `cursor`, `nvim`, `vim`).
  - **`[$]`**: Drop directly into an interactive subshell inside the project folder.
  - **`[S]`**: Cycle project sorting modes (Recently Modified, Alphabetical, Disk Size, Health Score).
  - **`[F]`**: Pin favorite projects to the very top with gold star badges (`★`).
- **In-TUI Markdown Editor**: Edit and preview project documentation with checkbox toggle support (`- [ ]`), dirty buffer tracking, and safety exit confirmations.
- **Project Doctor & Health Auditor**: Diagnostic engine that detects missing documentation/licenses, checks Git hygiene, sweeps junk files (`.DS_Store`, `Thumbs.db`), and calculates a health score.
- **Multi-Disciplinary Scaffolder**: 30+ built-in presets covering software (Rust, TypeScript, Python, FastAPI, React, C++, Go) and creative workflows (Blender 3D, CAD 3D printing, DaVinci Resolve video, podcast, logo design, game design).
- **Yearly Archival**: Safe one-key archival (`[A]`) moving completed work into cold storage (`archive/<year>/`).
- **Zero Runtime Dependencies**: Compiled to a single, compact, self-contained native binary.

---

## Workspace Architecture

All project scaffolding and organization automatically routes to your configured master workspace:

```text
Your Configured Workspace Root (e.g. ~/Projects)
├── personal/              # Personal software, experiments & creative assets
│   └── <project_name>/
├── clients/               # Client commissions & deliverables
│   └── <client_name>/
│       └── <project_name>/
├── resources/             # Shared master 3D, audio, font & stock libraries
│   └── <resource_name>/
└── archive/               # Yearly cold-storage for completed work
    └── 2026/
        └── <project_name>/
```

---

## Installation & Packaging

### Option 1: Universal Installer Script
```bash
# Clone the repository and run the installer:
git clone https://github.com/jaigansa/seed.git
cd seed
bash install.sh
```
This builds and installs `seed` to `~/.local/bin/seed`, automatically configures shell completions for Bash, Zsh, and Fish, and checks your `$PATH`.

### Option 2: From Source with Cargo
```bash
cargo install --path .
```

### Option 3: Arch Linux (AUR / `makepkg`)
```bash
cd packages/arch
makepkg -si
```

### Option 4: Debian / Ubuntu (`.deb`)
```bash
bash packages/debian/build_deb.sh
sudo dpkg -i seed_0.2.0_amd64.deb
```

---

## Shell Auto-Completions

Generate native auto-completion scripts for your active shell on the fly:

```bash
# Bash
seed completions bash > ~/.local/share/bash-completion/completions/seed

# Zsh
seed completions zsh > ~/.zfunc/_seed

# Fish
seed completions fish > ~/.config/fish/completions/seed.fish

# PowerShell
seed completions powershell > seed.ps1
```

---

## Interactive TUI Dashboard Keybindings

Launch the interactive dashboard at any time:
```bash
seed tui
# or simply:
seed
```

### Main Dashboard (`Tab 0: Projects`)

| Hotkey | Action |
|:---:|---|
| **`[F]`** | **Pin / Unpin project to top of list** (marked with `★`) |
| **`[S]`** | **Cycle Sort Mode** (`Recent` ➔ `Name A-Z` ➔ `Size` ➔ `Health`) |
| **`[I]` / `[V]`** | **Open in IDE / Editor** (launches `$VISUAL`, `$EDITOR`, `code`, `nvim`, `vim`) |
| **`[$]`** | **Launch Subshell** inside selected project directory |
| **`[E]`** | **Edit / Preview README** in in-TUI Markdown editor |
| **`[O]` / `[Enter]`** | **Open project directory** in desktop file manager |
| **`[D]`** | **Project Doctor**: Audit health, missing files & view score gauge |
| **`[P]` / `[T]`** | **Interactive Directory Tree** preview with scrollable file list |
| **`[A]`** | **Archive project** to `archive/<year>/` with confirmation dialog |
| **`[N]`** | **New Project**: Open scaffolding modal overlay |
| **`[C]`** | **Configuration**: Open workspace paths and settings modal |
| **`[/]`** | **Live Search**: Instant real-time filter across projects |
| **`[R]`** | **Refresh**: Rescan workspace projects from disk |
| **`[?]` / `[F1]`** | **Help**: Shortcut cheat sheet modal overlay |
| **`[q]`** | **Quit application** |

---

## Command Line Interface (CLI)

Seed includes a complete command-line interface for scripting and terminal efficiency:

```bash
# Initialize standard 4-pillar workspace structure
seed workspace ~/Projects

# Scaffold a new personal project
seed new my-api -t rust -c personal --git

# Scaffold a client commission
seed new campaign-brand -t logo -c client --client "AcmeCorp"

# Audit project health and detect junk files
seed doctor ./personal/my-api

# Repair missing baseline files & clean junk
seed sync ./personal/my-api

# Pin a project to the top of the dashboard
seed pin my-api

# Archive a completed project
seed archive ./clients/AcmeCorp/campaign-brand

# Visualize directory tree with sizes
seed tree ./personal/my-api

# Configure icon mode (nerdfont or ascii)
seed icons toggle
```

---

## Icon Support

Seed includes a pure Rust icon engine with zero external dependencies:
- **`ascii`**: Universal ASCII indicators (`[★]`, `[OK]`, `[DIR]`, `-`, `+`). Works seamlessly across all terminals and remote SSH sessions.
- **`nerdfont`**: Crisp Nerd Font glyphs (`` Rust, `` Python, `` Audio, `` Video, `` Folders, `` Stars).

Toggle anytime via `seed icons toggle` or in the TUI Settings modal (`[C]`).

---

## Verification & Test Suite

All 55 unit and integration tests pass cleanly:
```bash
cargo test
cargo clippy --all-targets
```

---

## License

Licensed under the [MIT](LICENSE) License.
