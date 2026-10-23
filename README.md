# Seed (v0.2.0)

> A focused, powerful Project Directory & File Creator, Scaffolder, and Workspace Manager TUI app written in Rust.

**No more messy current directory (`.`) clutter!** Seed strictly stores and maintains all your projects in your designated workspace path configured in `~/.config/seed/config.toml`.

---

## Guaranteed Workspace Hierarchy

All project creation and initialization automatically routes to your configured workspace:

```text
Your Configured Workspace (e.g. ~/Projects or ~/Creative)
├── personal/              # Personal & portfolio work
│   └── your_project/
├── clients/               # Client commissions & deliverables
│   └── ClientName/
│       └── your_project/
├── resources/             # Shared master asset library
│   └── your_asset/
└── archive/               # Yearly cold-storage for finished work
    └── 2026/
        └── your_project/
```

---

## Running the TUI App

```bash
# Launch interactive TUI
./target/release/seed
```

### The 4 Dedicated Sections:

1. **1. Create New Project**: Pick an output-driven template (Video: `yt-video`, Audio: `podcast`, `music`, Publishing: `book`, `newsletter`, `presentation`, `research-paper`, `portfolio`, Fabrication & 3D: `cad-maker`, `3d-blender`, `vector-design`, `photo-editing`, `creative-coding`, Interactive: `game`, `client-project`, Software: `react`, `fastapi`, `data-science`, `docker`, `docs`, `rust`, `python`, `go`, `cpp`, `c`, `java`, `bash`, `web`, `typescript`, `minimal`).
   - Press **`[F]`** to mark the highlighted template as a favorite: it immediately gets pinned to the top of the list with a `[FAV]` badge!
   - Choose Category (`Personal`, `Client`, or `Resource`).
   - Automatically paths to `<workspace>/personal/<name>` or `<workspace>/clients/<client>/<name>`.
   - Inspect concrete target deliverables before creating.
   - Press **`[Enter]`** to scaffold!

2. **2. View Projects**: Directly inspect, browse, search, audit, open, and archive projects across your workspace.
   - Filter bar: Switch between `[All]`, `[Personal]`, `[Clients]`, `[Resources]`, and `[Archive]` using **`[←]` / `[→]`**.
   - **Live Search (`/`)**: Instant real-time search across project names, detected stacks, and client names.
   - Browse projects using **`[↑]` / `[↓]`** or mouse wheel scroll.
   - Right-hand **Selected Project Details & Actions** container is a clean canvas for viewing and updating project info (full path, detected stack/type, directory/file counts) and gives quick actions to **view/edit** the project via file manager/tree or to act on it.
   - Press **`[O]`** or **`[Enter]`** to open selected project folder in your desktop file manager / viewer (to browse and edit files)!
   - Press **`[D]`** to open the **Doctor Health Gauge** modal with visual score bars and recommendations.
   - Press **`[T]`** to inspect full scrollable directory tree of the project.
   - Press **`[A]`** to open safe **Confirmation Dialog** before archiving to `archive/<year>/`.
   - Press **`[R]`** to refresh projects from disk.

3. **3. Templates**: Browse, filter, and create both **Built-in Presets** and **Custom Manual Templates**.
   - Filter bar: Switch between `[All]`, `[Preset]`, and `[Manual]` templates using **`[←]` / `[→]`**.
   - Press **`[M]`** to open the **New Manual Template Creator** modal (define custom template name, description, directories, starter files, and deliverables interactively!).
   - Press **`[F]`** to pin/unpin any template to the top of your favorites.
   - Press **`[C]`** to capture any directory into a reusable custom template.

4. **4. Project Paths & Config**:
   - Set your **Workspace Root** (e.g., `~/Projects` or `~/Creative`).
   - Toggle **Icon Mode** between `< ascii >` (clean text) and `< nerdfont >` (pure Rust terminal glyphs) with **`[Space]`**.
   - Edit paths without hotkey collisions (all standard characters type cleanly into text fields).
   - Press **`Ctrl+S`** or navigate to **`[Save Configuration]`** to persist changes.
   - Navigate to **`[Initialize Master Workspace]`** to create the clean master folders (`personal/`, `clients/`, `resources/`, `archive/`).

---

## TUI Keybindings

| Key | Action |
|---|---|
| `Tab` / `BackTab` | Switch between the 4 sections |
| `1`, `2`, `3`, `4` | Jump directly to a section |
| `Mouse Click` | Click on section tabs to switch directly |
| `Mouse Scroll` | Scroll projects list, templates, or tree lines |
| `↑` / `↓` | Navigate fields / projects in list |
| `←` / `→` | Change template (Create) / Switch filter category (View Projects & Templates) |
| `/` | Live Search projects (Tab 2) |
| `M` | Create a New Manual Template modal (Tab 3 Templates) |
| `Space` | Cycle categories (`Personal` / `Client` / `Resource`), cycle licenses, or toggle git repo |
| `Enter` | Create Project (Create tab) / Open Folder (View Projects tab) / Save or Init (Config tab) |
| `O` | Open selected project folder in desktop file manager (View Projects tab) |
| `D` | Run Doctor health check with visual Gauge modal (View Projects tab) |
| `T` | View scrollable Directory Tree modal (View Projects tab) |
| `A` | Archive project with confirmation dialog (View Projects tab) |
| `R` | Refresh workspace project list (View Projects tab) |
| `F` | Toggle favorite template (pins template to top of list with `[FAV]`) |
| `C` | Capture directory into custom manual template (Tab 3) |
| `Ctrl+S` | Save configuration (Config tab) or Save Manual Template (Modal) |
| `q` | Quit application (when not editing a text field) |
| `Ctrl+C` | Quit application anytime |

---

## Preset & Manual Templates CLI

Manage and filter preset and manual templates from your shell:

```bash
# List all templates with [Preset] and [Manual] badges
seed list

# List only built-in preset workflows
seed list --preset

# List only custom manual templates
seed list --manual

# Create a new custom manual template
seed template-new my-workflow --dirs "src,tests,docs" --files "README.md,main.py,config.json" --outputs "Executable App,API Docs"

# Create a project using a manual template
seed new my-project -t my-workflow

# Create a project in manual blank mode
seed new my-scratch-app -m
```

---

## Favorite Templates CLI

You can also manage your favorite pinned templates directly from your shell:

```bash
# Toggle a template as favorite (pins to the very top of lists)
seed fav rust
seed fav podcast

# List all currently pinned favorites
seed fav

# View all templates with favorites badged on top
seed list

# Remove a specific favorite
seed fav rust --remove

# Clear all favorites
seed fav --clear
```

---

## Icon Support (Pure Rust)

`seed` includes a pure Rust icon mapping engine with zero external dependencies and zero Unicode emojis:
- **`ascii` (Default)**: Clean, universal ASCII badges (`[FAV]`, `[OK]`, `[DIR]`, `-`, `+`). Works on all terminals and remote SSH sessions without requiring special fonts.
- **`nerdfont`**: Renders crisp Nerd Font glyphs (`` Rust, `` Python, `` Podcast, `` Video, `` Folders, `` Stars).

To toggle icon mode:
- **TUI**: In Tab 4 (`Project Paths & Config`), navigate to `Icon Mode` and press **`[Space]`**.
- **Config**: Set `icon_mode = "nerdfont"` in `~/.config/seed/config.toml`.

---

## License

Licensed under the [MIT](LICENSE) License.
