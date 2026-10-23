use anyhow::{bail, Context, Result};
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub workspace_root: String,
    #[serde(default = "default_personal_dir")]
    pub personal_dir: String,
    #[serde(default = "default_clients_dir")]
    pub clients_dir: String,
    #[serde(default = "default_resources_dir")]
    pub resources_dir: String,
    #[serde(default = "default_archive_dir")]
    pub archive_dir: String,
    pub default_author: String,
    pub default_license: String,
    pub default_template: String,
    pub custom_templates_dir: String,
    #[serde(default)]
    pub favorite_templates: Vec<String>,
    #[serde(default = "default_icon_mode")]
    pub icon_mode: String,
}

fn default_icon_mode() -> String {
    "nerdfont".to_string()
}

fn default_personal_dir() -> String {
    "personal".to_string()
}

fn default_clients_dir() -> String {
    "clients".to_string()
}

fn default_resources_dir() -> String {
    "resources".to_string()
}

fn default_archive_dir() -> String {
    "archive".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let workspace_root = format!("{}/Projects", home);
        let templates_dir = format!("{}/.config/seed/templates", home);

        let author = crate::utils::get_git_user()
            .or_else(|| std::env::var("USER").ok())
            .unwrap_or_else(|| "Developer".to_string());

        Self {
            workspace_root,
            personal_dir: default_personal_dir(),
            clients_dir: default_clients_dir(),
            resources_dir: default_resources_dir(),
            archive_dir: default_archive_dir(),
            default_author: author,
            default_license: "MIT".to_string(),
            default_template: "rust".to_string(),
            custom_templates_dir: templates_dir,
            favorite_templates: Vec::new(),
            icon_mode: default_icon_mode(),
        }
    }
}

pub struct WorkspaceSummary {
    pub created_dirs: usize,
    pub created_readme_files: usize,
}

impl AppConfig {
    pub fn is_favorite(&self, name: &str) -> bool {
        self.favorite_templates.iter().any(|fav| fav == name)
    }

    pub fn toggle_favorite(&mut self, name: &str) -> bool {
        if let Some(pos) = self.favorite_templates.iter().position(|fav| fav == name) {
            self.favorite_templates.remove(pos);
            false
        } else {
            self.favorite_templates.push(name.to_string());
            true
        }
    }

    pub fn use_nerd_font(&self) -> bool {
        self.icon_mode.eq_ignore_ascii_case("nerdfont") || self.icon_mode.eq_ignore_ascii_case("nerd-font")
    }

    pub fn cycle_icon_mode(&mut self) -> &str {
        if self.use_nerd_font() {
            self.icon_mode = "ascii".to_string();
        } else {
            self.icon_mode = "nerdfont".to_string();
        }
        &self.icon_mode
    }
    pub fn root_path(&self) -> PathBuf {
        PathBuf::from(&self.workspace_root)
    }

    pub fn personal_path(&self) -> PathBuf {
        self.root_path().join(&self.personal_dir)
    }

    pub fn clients_path(&self) -> PathBuf {
        self.root_path().join(&self.clients_dir)
    }

    pub fn resources_path(&self) -> PathBuf {
        self.root_path().join(&self.resources_dir)
    }

    pub fn archive_path(&self) -> PathBuf {
        self.root_path().join(&self.archive_dir)
    }

    /// Initializes the full master folder structure: personal, clients, resources, archive
    pub fn init_master_workspace(&self) -> Result<WorkspaceSummary> {
        let root = self.root_path();
        fs::create_dir_all(&root)
            .with_context(|| format!("Failed to create workspace root: {}", root.display()))?;

        let year = chrono::Utc::now().year();

        // Core workspace root categories (clean, no dummy subfolders that confuse project scanners)
        let structure_dirs = [
            self.personal_path(),
            self.clients_path(),
            self.resources_path(),
            self.archive_path().join(year.to_string()),
        ];

        let mut summary = WorkspaceSummary {
            created_dirs: 0,
            created_readme_files: 0,
        };

        for dir in &structure_dirs {
            if !dir.exists() {
                fs::create_dir_all(dir)?;
                summary.created_dirs += 1;
            }
        }

        // Create standard README guides for each main section
        let guides: Vec<(PathBuf, String)> = vec![
            (
                self.personal_path().join("README.md"),
                "# Personal Projects Workspace\n\nStore all your personal, portfolio, and experimental creative & coding projects here.\nProjects are created as `<workspace>/personal/<project_name>`.\n".to_string(),
            ),
            (
                self.clients_path().join("README.md"),
                "# Client Projects Workspace\n\nOrganized by client name:\n\n```text\nclients/\n  └── ClientName/\n        └── ProjectName/\n              ├── briefs_and_contracts/\n              ├── deliverables/\n              └── invoices/\n```\n".to_string(),
            ),
            (
                self.resources_path().join("README.md"),
                "# Creative Resource & Asset Library\n\nShared master assets used across your projects:\n- 3D models and meshes\n- PBR textures and materials\n- Royalty-free music and audio sound effects\n- Fonts and typography\n- Stock footage and vector icons\n".to_string(),
            ),
            (
                self.archive_path().join("README.md"),
                format!("# Project Archive\n\nCompleted or retired projects organized by year.\nCurrent active archive year: `{year}/`.\n"),
            ),
            (
                root.join("WORKSPACE.md"),
                format!(
                    r#"# Master Creative & Coding Workspace

Configured via **Seed** (v0.2.0).

## Core Structure
```text
{root}/
  ├── personal/    -> Personal builds, creative work, prototypes
  ├── clients/     -> Client commissions, deliverables, invoices
  ├── resources/   -> Master asset library (3D, Audio, Textures, Fonts)
  └── archive/     -> Cold storage for completed projects by year
```
"#,
                    root = root.display()
                ),
            ),
        ];

        for (path, content) in &guides {
            if !path.exists() {
                fs::write(path, content)?;
                summary.created_readme_files += 1;
            }
        }

        Ok(summary)
    }

    /// Moves a completed project into archive/<year>/<project_name>
    pub fn archive_project(&self, project_path: &Path) -> Result<PathBuf> {
        if !project_path.exists() {
            bail!("Project path '{}' does not exist", project_path.display());
        }

        let canonical_proj = project_path.canonicalize().unwrap_or_else(|_| project_path.to_path_buf());
        let proj_name = canonical_proj
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("project");

        let year = chrono::Utc::now().year();
        let archive_base = self.archive_path();

        // Guard against archiving something that is already in archive/
        if canonical_proj.starts_with(&archive_base) {
            bail!(
                "Project '{}' is already located inside the archive: {}",
                proj_name,
                canonical_proj.display()
            );
        }

        // Detect parent workspace containing archive/ or WORKSPACE.md
        let target_dir = if let Some(parent_ws) = find_parent_workspace(&canonical_proj) {
            parent_ws.join(&self.archive_dir).join(year.to_string())
        } else {
            archive_base.join(year.to_string())
        };

        // If project was inside clients/<client>/<proj>, preserve client namespace in archive
        let target_dir = if let Some(client_name) = detect_client_parent(&canonical_proj, &self.clients_path()) {
            target_dir.join("clients").join(client_name)
        } else {
            target_dir
        };

        fs::create_dir_all(&target_dir)?;

        let dest = target_dir.join(proj_name);
        if dest.exists() {
            bail!(
                "An archived project already exists at '{}'",
                dest.display()
            );
        }

        // Try direct rename; fall back to recursive copy + remove if across filesystem boundaries (EXDEV)
        if fs::rename(&canonical_proj, &dest).is_err() {
            copy_dir_all(&canonical_proj, &dest).with_context(|| {
                format!(
                    "Failed to copy project from '{}' to '{}'",
                    canonical_proj.display(),
                    dest.display()
                )
            })?;
            fs::remove_dir_all(&canonical_proj).with_context(|| {
                format!(
                    "Failed to remove source project directory after copy: '{}'",
                    canonical_proj.display()
                )
            })?;
        }

        Ok(dest)
    }
}

fn detect_client_parent(path: &Path, clients_base: &Path) -> Option<String> {
    let parent = path.parent()?;
    if let Ok(canon_clients) = clients_base.canonicalize()
        && let Ok(canon_parent) = parent.canonicalize()
            && canon_parent.parent() == Some(&canon_clients) {
                return canon_parent.file_name().and_then(|n| n.to_str()).map(|s| s.to_string());
            }
    None
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

fn find_parent_workspace(path: &Path) -> Option<PathBuf> {
    let mut current = path.parent();
    while let Some(dir) = current {
        if dir.join("archive").exists() || dir.join("WORKSPACE.md").exists() {
            return Some(dir.to_path_buf());
        }
        current = dir.parent();
    }
    None
}

pub fn get_config_file_path() -> PathBuf {
    if let Ok(custom) = std::env::var("SEED_CONFIG") {
        return PathBuf::from(custom);
    }
    let local = PathBuf::from(".seed").join("config.toml");
    if local.exists() {
        return local;
    }
    if let Ok(home) = std::env::var("HOME") {
        let global = PathBuf::from(home).join(".config").join("seed").join("config.toml");
        if global.exists() {
            return global;
        }
        // If parent exists and is writable, use global
        if let Some(p) = global.parent()
            && fs::create_dir_all(p).is_ok() {
                return global;
            }
    }
    local
}

pub fn load_config() -> AppConfig {
    let path = get_config_file_path();
    if path.exists()
        && let Ok(content) = fs::read_to_string(&path)
            && let Ok(cfg) = toml::from_str(&content) {
                return cfg;
            }
    let default_cfg = AppConfig::default();
    let _ = save_config(&default_cfg);
    default_cfg
}

pub fn save_config(cfg: &AppConfig) -> Result<()> {
    let path = get_config_file_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(cfg)?;
    fs::write(&path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_init_master_workspace_and_archive() {
        let tmp = tempdir().unwrap();
        let ws_root = tmp.path().join("MyCreativeHub");

        let cfg = AppConfig {
            workspace_root: ws_root.to_string_lossy().to_string(),
            personal_dir: "personal".into(),
            clients_dir: "clients".into(),
            resources_dir: "resources".into(),
            archive_dir: "archive".into(),
            default_author: "Maker".into(),
            default_license: "MIT".into(),
            default_template: "yt-video".into(),
            custom_templates_dir: tmp.path().join("templates").to_string_lossy().to_string(),
            favorite_templates: Vec::new(),
            icon_mode: "ascii".into(),
        };

        let summary = cfg.init_master_workspace().unwrap();
        assert!(summary.created_dirs >= 4);
        assert!(cfg.personal_path().exists());
        assert!(cfg.clients_path().exists());
        assert!(cfg.resources_path().exists());
        assert!(cfg.archive_path().exists());

        // Test archive a dummy project
        let dummy_proj = cfg.personal_path().join("done-project");
        fs::create_dir_all(&dummy_proj).unwrap();
        fs::write(dummy_proj.join("work.txt"), "finished").unwrap();

        let archived = cfg.archive_project(&dummy_proj).unwrap();
        assert!(archived.exists());
        assert!(archived.join("work.txt").exists());
        assert!(!dummy_proj.exists());
    }

    #[test]
    fn test_favorite_templates_toggle() {
        let mut cfg = AppConfig::default();
        assert!(!cfg.is_favorite("rust"));

        // Toggle on
        let added = cfg.toggle_favorite("rust");
        assert!(added);
        assert!(cfg.is_favorite("rust"));
        assert_eq!(cfg.favorite_templates, vec!["rust".to_string()]);

        // Toggle on another
        let added2 = cfg.toggle_favorite("python");
        assert!(added2);
        assert!(cfg.is_favorite("python"));
        assert_eq!(cfg.favorite_templates, vec!["rust".to_string(), "python".to_string()]);

        // Toggle off
        let removed = cfg.toggle_favorite("rust");
        assert!(!removed);
        assert!(!cfg.is_favorite("rust"));
        assert_eq!(cfg.favorite_templates, vec!["python".to_string()]);
    }

    #[test]
    fn test_icon_mode_cycle() {
        let mut cfg = AppConfig::default();
        assert_eq!(cfg.icon_mode, "nerdfont");
        assert!(cfg.use_nerd_font());

        cfg.cycle_icon_mode();
        assert_eq!(cfg.icon_mode, "ascii");
        assert!(!cfg.use_nerd_font());

        cfg.cycle_icon_mode();
        assert_eq!(cfg.icon_mode, "nerdfont");
        assert!(cfg.use_nerd_font());
    }
}
