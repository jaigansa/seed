use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "seed",
    author = "Jaiganesh",
    version = "0.2.0",
    about = "Project directory & file creator, scaffolder, and maintainer tool written in Rust",
    long_about = "A versatile CLI tool for creating structured project directories, scaffolding files from templates, auditing project health, and maintaining file standards."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Launch the interactive full-screen Terminal User Interface (TUI)
    Tui(TuiArgs),

    /// Initialize the complete master workspace structure (personal, clients, resources, archive)
    #[command(alias = "workspace")]
    InitWorkspace(InitWorkspaceArgs),

    /// Archive a completed project into archive/<year>/
    Archive(ArchiveArgs),

    /// Create a new project directory and scaffold files
    New(NewArgs),

    /// Interactively initialize a project in the current or new directory
    Init(InitArgs),

    /// Audit and check project structure, missing files, and health score
    #[command(alias = "doctor")]
    Check(CheckArgs),

    /// Maintain, sync, and repair project files and directories
    #[command(alias = "maintain")]
    Sync(SyncArgs),

    /// List all available built-in preset and manual templates
    List(ListArgs),

    /// Create a new manual template with custom directories, files, and outputs
    #[command(alias = "manual-template", alias = "create-template")]
    TemplateNew(TemplateNewArgs),

    /// Show detailed contents and structure of a template
    Info(InfoArgs),

    /// Visualize directory tree with file details and stats
    Tree(TreeArgs),

    /// Capture an existing directory structure into a reusable template
    Capture(CaptureArgs),

    /// Manage favorite templates pinned to the top of lists
    #[command(alias = "favorite")]
    Fav(FavArgs),

    /// Pin or unpin a project to the top of the workspace dashboard
    #[command(alias = "pin-project")]
    Pin(PinArgs),

    /// Initialize template folders (user and/or local)
    #[command(alias = "templates-init")]
    InitTemplates(InitTemplatesArgs),

    /// Configure or toggle icon display mode (nerdfont or ascii)
    #[command(alias = "icon")]
    Icons(IconsArgs),

    /// Generate shell auto-completions script for bash, zsh, fish, or powershell
    #[command(alias = "completion")]
    Completions(CompletionsArgs),
}

#[derive(Args, Debug, Default)]
pub struct CompletionsArgs {
    /// Target shell to generate completions for (bash, zsh, fish, powershell)
    #[arg(default_value = "bash")]
    pub shell: String,
}

#[derive(Args, Debug, Default)]
pub struct TuiArgs {
    /// Directory to manage in TUI (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: PathBuf,
}

#[derive(Args, Debug, Default)]
pub struct InitWorkspaceArgs {
    /// Workspace root directory (defaults to configured workspace_root)
    pub path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct ArchiveArgs {
    /// Project directory to archive
    pub path: PathBuf,
}

#[derive(Args, Debug)]
pub struct NewArgs {
    /// Name of the project to create
    pub name: String,

    /// Project category (personal, client, resource)
    #[arg(short = 'c', long, default_value = "personal")]
    pub category: String,

    /// Client name (if category is client)
    #[arg(long)]
    pub client: Option<String>,

    /// Template to use (e.g., 3dmodel, cad, logo, video-edit, photo-edit, vector-artwork, webdesign, coding, etc.)
    #[arg(short, long, default_value = "coding")]
    pub template: String,

    /// Create project in manual mode with a minimal blank starter structure
    #[arg(short = 'm', long)]
    pub manual: bool,

    /// Target directory (defaults to ./<name>)
    #[arg(short, long)]
    pub dir: Option<PathBuf>,

    /// Author name (defaults to git config user.name)
    #[arg(short, long)]
    pub author: Option<String>,

    /// License identifier (MIT, Apache-2.0, BSD-3, Unlicense, None)
    #[arg(short, long, default_value = "MIT")]
    pub license: String,

    /// Initialize a Git repository
    #[arg(short, long)]
    pub git: bool,

    /// Overwrite existing files if directory exists
    #[arg(short, long)]
    pub force: bool,

    /// Custom template variables in KEY=VALUE format
    #[arg(long = "var", value_parser = parse_key_val::<String, String>)]
    pub vars: Vec<(String, String)>,
}

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Directory to initialize (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Skip interactive prompts and use defaults
    #[arg(short, long)]
    pub non_interactive: bool,

    /// Template to use
    #[arg(short, long)]
    pub template: Option<String>,
}

#[derive(Args, Debug)]
pub struct CheckArgs {
    /// Directory to audit (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Compare against a specific template (auto-detected if omitted)
    #[arg(short, long)]
    pub template: Option<String>,
}

#[derive(Args, Debug)]
pub struct SyncArgs {
    /// Directory to maintain (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Template to sync missing files from
    #[arg(short, long)]
    pub template: Option<String>,

    /// Remove junk files (.DS_Store, Thumbs.db, *.tmp, etc.)
    #[arg(short, long)]
    pub clean: bool,

    /// Initialize Git repository if missing
    #[arg(short, long)]
    pub git: bool,

    /// Overwrite existing files with template versions
    #[arg(short, long)]
    pub force: bool,

    /// Project author for missing LICENSE
    #[arg(short, long)]
    pub author: Option<String>,

    /// Project license for missing LICENSE
    #[arg(short, long)]
    pub license: Option<String>,
}

#[derive(Args, Debug)]
pub struct InfoArgs {
    /// Template name or file path
    pub template: String,
}

#[derive(Args, Debug)]
pub struct TreeArgs {
    /// Directory to inspect (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Maximum tree depth to display
    #[arg(short = 'L', long, default_value_t = 4)]
    pub depth: usize,

    /// Include hidden files and directories
    #[arg(short = 'a', long)]
    pub all: bool,

    /// Hide file sizes
    #[arg(long)]
    pub no_sizes: bool,
}

#[derive(Args, Debug)]
pub struct CaptureArgs {
    /// Directory to capture into a template (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Name for the generated template
    #[arg(short, long)]
    pub name: Option<String>,

    /// Description for the generated template
    #[arg(long, default_value = "Exported project template")]
    pub desc: String,

    /// Destination file path (.yaml or .toml)
    #[arg(short, long, default_value = "template.yaml")]
    pub output: PathBuf,

    /// Do not replace directory name with {{project_name}} variable
    #[arg(long)]
    pub no_replace: bool,

    /// Max file size to include in KB (skips large files)
    #[arg(long, default_value_t = 128)]
    pub max_size_kb: usize,
}

#[derive(Args, Debug, Default)]
pub struct ListArgs {
    /// Show only built-in preset templates
    #[arg(short = 'p', long)]
    pub preset: bool,

    /// Show only manual/custom templates
    #[arg(short = 'm', long)]
    pub manual: bool,
}

#[derive(Args, Debug)]
pub struct TemplateNewArgs {
    /// Name of the manual template
    pub name: String,

    /// Short description of the template
    #[arg(short = 'd', long, default_value = "Custom manual template")]
    pub description: String,

    /// Comma-separated list of directories (e.g. "src,tests,docs")
    #[arg(long, default_value = "src,docs")]
    pub dirs: String,

    /// Comma-separated list of files (e.g. "README.md,.gitignore")
    #[arg(long, default_value = "README.md,.gitignore")]
    pub files: String,

    /// Comma-separated list of deliverables / outputs (e.g. "Deliverable 1,Docs")
    #[arg(long, default_value = "Deliverable")]
    pub outputs: String,

    /// Save to local workspace (.seed/templates) instead of global user config
    #[arg(long)]
    pub local: bool,
}

#[derive(Args, Debug, Default)]
pub struct FavArgs {
    /// Template name to toggle favorite status (or omit to view current favorites)
    pub template: Option<String>,

    /// Remove the specified template from favorites
    #[arg(short, long)]
    pub remove: bool,

    /// Clear all favorite templates
    #[arg(long)]
    pub clear: bool,
}

#[derive(Args, Debug, Default)]
pub struct PinArgs {
    /// Project name to pin or unpin (omit to list currently pinned projects)
    pub project: Option<String>,

    /// Unpin the specified project
    #[arg(short, long)]
    pub remove: bool,

    /// Clear all pinned projects
    #[arg(long)]
    pub clear: bool,
}

#[derive(Args, Debug, Default)]
pub struct IconsArgs {
    /// Icon mode to set: 'nerdfont' ('on'), 'ascii' ('off'), or 'toggle' (omit to show current status)
    pub mode: Option<String>,
}

#[derive(Args, Debug, Default)]
pub struct InitTemplatesArgs {
    /// Initialize only user templates directory (~/.config/seed/templates/)
    #[arg(long)]
    pub user: bool,

    /// Initialize only local templates directory (./.seed/templates/)
    #[arg(long)]
    pub local: bool,
}

fn parse_key_val<T, U>(
    s: &str,
) -> std::result::Result<(T, U), Box<dyn std::error::Error + Send + Sync>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
