use crate::template::{builtin::find_builtin_template, Template};
use crate::utils::{current_year, get_license_text, print_info, print_success};
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub enum IssueSeverity {
    Critical,
    Warning,
    Info,
}

#[derive(Debug)]
pub struct ProjectIssue {
    pub severity: IssueSeverity,
    pub title: String,
    pub message: String,
    pub fix_hint: Option<String>,
}

#[derive(Debug, Default)]
pub struct AuditReport {
    pub project_type: String,
    pub detected_template: Option<String>,
    pub issues: Vec<ProjectIssue>,
    pub junk_files: Vec<PathBuf>,
    pub empty_dirs: Vec<PathBuf>,
    pub empty_files: Vec<PathBuf>,
    pub total_files: usize,
    pub total_dirs: usize,
    pub health_score: u32,
}

/// Detect project type by examining marker files and creative asset structures
pub fn detect_project_type(root: &Path) -> Option<&'static str> {
    if root.join("Cargo.toml").exists() {
        Some("rust")
    } else if root.join("vite.config.ts").exists() || root.join("src/App.tsx").exists() {
        Some("react")
    } else if root.join("app/main.py").exists() {
        Some("fastapi")
    } else if root.join("notebooks").exists() && root.join("data").exists() {
        Some("data-science")
    } else if root.join("pyproject.toml").exists()
        || root.join("setup.py").exists()
        || root.join("requirements.txt").exists()
    {
        Some("python")
    } else if root.join("tsconfig.json").exists() || root.join("package.json").exists() {
        Some("typescript")
    } else if root.join("go.mod").exists() {
        Some("go")
    } else if root.join("CMakeLists.txt").exists() {
        Some("cpp")
    } else if root.join("pom.xml").exists() || root.join("build.gradle").exists() {
        Some("java")
    } else if root.join("docker-compose.yml").exists() || root.join("docker-compose.yaml").exists() {
        Some("docker")
    } else if root.join("episodes").exists() || root.join("show_notes").exists() {
        Some("podcast")
    } else if root.join("tracks").exists() || root.join("assets/music_stems").exists() {
        Some("music")
    } else if root.join("manuscript").exists() {
        Some("book")
    } else if root.join("design/gdd").exists() {
        Some("game")
    } else if root.join("slides").exists() {
        Some("presentation")
    } else if root.join("paper.tex").exists() || root.join("references.bib").exists() {
        Some("research-paper")
    } else if root.join("issues").exists() {
        Some("newsletter")
    } else if root.join("case_studies").exists() {
        Some("portfolio")
    } else if root.join("lib/utils.sh").exists() {
        Some("bash")
    } else if root.join("src/main.c").exists() {
        Some("c")
    } else if root.join("briefs_and_contracts").exists()
        || root.join("deliverables").exists()
        || root.join("invoices").exists()
    {
        Some("client-project")
    } else if root.join("models/specs.md").exists()
        || root.join("models/high_poly").exists()
        || root.join("exports/gltf").exists()
        || root.join("scenes").exists()
    {
        Some("3dmodel")
    } else if root.join("bom/bill_of_materials.md").exists()
        || root.join("cad/parts").exists()
        || root.join("cad").exists()
        || root.join("specs/tolerances.md").exists()
        || root.join("slicer").exists()
    {
        Some("cad")
    } else if root.join("guidelines/brand_guidelines.md").exists()
        || root.join("source/color_palettes").exists()
        || root.join("concepts/sketches").exists()
    {
        Some("logo")
    } else if root.join("transcripts/script_and_cue.md").exists()
        || root.join("projects/timeline").exists()
        || root.join("footage/raw").exists()
        || root.join("scripts/script.md").exists()
        || root.join("assets/footage").exists()
    {
        Some("video-edit")
    } else if root.join("metadata/shot_list.md").exists()
        || root.join("presets/grading_notes.md").exists()
        || root.join("raw/selected").exists()
        || root.join("edits/gimp_xcf").exists()
    {
        Some("photo-edit")
    } else if root.join("source/components/design_tokens.md").exists()
        || root.join("branding_guidelines.md").exists()
        || root.join("source/svg").exists()
    {
        Some("vector-artwork")
    } else if root.join("design_system/design_tokens.md").exists()
        || root.join("prototype/src/index.html").exists()
        || root.join("wireframes").exists()
    {
        Some("webdesign")
    } else if root.join("docs/architecture/overview.md").exists()
        || root.join("scripts/build.sh").exists()
    {
        Some("coding")
    } else if root.join("shaders").exists() || root.join("src/sketch.js").exists() {
        Some("creative-coding")
    } else if root.join("index.html").exists() {
        Some("web")
    } else {
        // Fallback: check file extensions in shallow scan
        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries.flatten() {
                let p = entry.path();
                if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                    match ext {
                        "blend" | "gltf" | "glb" | "fbx" | "obj" => return Some("3dmodel"),
                        "kdenlive" | "mlt" | "prproj" | "drp" => return Some("video-edit"),
                        "fcstd" | "scad" | "step" | "3mf" | "stl" => return Some("cad"),
                        "svg" => return Some("vector-artwork"),
                        "xcf" | "kra" | "ora" | "cr3" | "nef" | "arw" => return Some("photo-edit"),
                        _ => {}
                    }
                }
            }
        }
        None
    }
}

const JUNK_PATTERNS: &[&str] = &[
    ".DS_Store",
    "Thumbs.db",
    "desktop.ini",
    "npm-debug.log",
    "yarn-debug.log",
    "yarn-error.log",
    ".directory",
];

const JUNK_EXTENSIONS: &[&str] = &["swp", "swo", "tmp", "bak", "orig", "pyc"];

pub fn is_junk_file(path: &Path) -> bool {
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        if JUNK_PATTERNS.contains(&file_name) {
            return true;
        }
        if file_name.starts_with(".#") || (file_name.starts_with("#") && file_name.ends_with("#")) {
            return true;
        }
    }

    if let Some(ext) = path.extension().and_then(|e| e.to_str())
        && JUNK_EXTENSIONS.contains(&ext) {
            return true;
        }

    false
}

/// Run a comprehensive audit on a project directory
pub fn audit_project(root: &Path, expected_template: Option<&Template>) -> Result<AuditReport> {
    let mut report = AuditReport::default();

    if !root.exists() || !root.is_dir() {
        anyhow::bail!("Directory '{}' does not exist", root.display());
    }

    let detected = detect_project_type(root);
    report.project_type = detected.unwrap_or("generic").to_string();

    let template = match expected_template {
        Some(t) => Some(t.clone()),
        None => detected.and_then(find_builtin_template),
    };

    if let Some(ref t) = template {
        report.detected_template = Some(t.name.clone());
    }

    // Standard baseline files
    let standard_files = [
        ("README.md", IssueSeverity::Warning, "README.md is missing. Documentation is essential for project maintainability."),
        (".gitignore", IssueSeverity::Warning, ".gitignore is missing. Version control may track unwanted build artifacts."),
        ("LICENSE", IssueSeverity::Info, "No LICENSE file found. Open-source or shared code should declare licensing."),
    ];

    for (file, severity, msg) in standard_files {
        let p = root.join(file);
        if !p.exists() {
            report.issues.push(ProjectIssue {
                severity,
                title: format!("Missing standard file '{}'", file),
                message: msg.to_string(),
                fix_hint: Some(format!("Run 'seed sync' to generate a default {}", file)),
            });
        }
    }

    // Git repository check
    if !root.join(".git").exists() {
        report.issues.push(ProjectIssue {
            severity: IssueSeverity::Info,
            title: "Not a Git repository".into(),
            message: "Project is not tracked in Git version control.".into(),
            fix_hint: Some("Run 'git init' or 'seed sync --git'".into()),
        });
    }

    // If template is matched, check template's required files and directories
    if let Some(ref tpl) = template {
        for dir in &tpl.directories {
            // Note: If template directory has variables, we skip strict match if unresolved
            if !dir.contains("{{") {
                let dir_path = root.join(dir);
                if !dir_path.exists() {
                    report.issues.push(ProjectIssue {
                        severity: IssueSeverity::Warning,
                        title: format!("Missing template directory '{}/'", dir),
                        message: format!("Template '{}' expects directory '{}/'", tpl.name, dir),
                        fix_hint: Some(format!("mkdir -p {}", dir_path.display())),
                    });
                }
            }
        }

        for file in &tpl.files {
            if file.required && !file.path.contains("{{") {
                let file_path = root.join(&file.path);
                if !file_path.exists() {
                    report.issues.push(ProjectIssue {
                        severity: IssueSeverity::Critical,
                        title: format!("Missing required file '{}'", file.path),
                        message: format!("Required by template '{}'", tpl.name),
                        fix_hint: Some(format!("Run 'seed sync --template {}'", tpl.name)),
                    });
                }
            }
        }
    }

    // Walk directory to check junk files, empty files, empty dirs
    let mut all_dirs = std::collections::HashSet::new();
    let mut non_empty_parents = std::collections::HashSet::new();

    for entry in WalkDir::new(root).into_iter().filter_entry(|e| {
        // Skip .git and target/node_modules for junk scanning
        let file_name = e.file_name().to_string_lossy();
        file_name != ".git" && file_name != "target" && file_name != "node_modules" && file_name != ".venv"
    }) {
        let entry = entry.context("Failed reading entry during audit")?;
        let path = entry.path();

        if path == root {
            continue;
        }

        if let Some(parent) = path.parent() {
            non_empty_parents.insert(parent.to_path_buf());
        }

        if path.is_file() {
            report.total_files += 1;

            if is_junk_file(path)
                && let Ok(rel) = path.strip_prefix(root) {
                    report.junk_files.push(rel.to_path_buf());
                }

            if let Ok(meta) = path.metadata()
                && meta.len() == 0 {
                    // Exclude intentional empty files like __init__.py or .gitkeep
                    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if name != "__init__.py" && name != ".gitkeep"
                        && let Ok(rel) = path.strip_prefix(root) {
                            report.empty_files.push(rel.to_path_buf());
                        }
                }
        } else if path.is_dir() {
            report.total_dirs += 1;
            all_dirs.insert(path.to_path_buf());
        }
    }

    // Find empty directories (directories in all_dirs with zero child entries)
    let mut empty_dirs_vec: Vec<_> = all_dirs
        .into_iter()
        .filter(|dir| !non_empty_parents.contains(dir))
        .filter_map(|dir| dir.strip_prefix(root).ok().map(|p| p.to_path_buf()))
        .collect();
    empty_dirs_vec.sort();
    report.empty_dirs = empty_dirs_vec;

    // Calculate health score (0-100)
    let mut deductions = 0;
    for issue in &report.issues {
        match issue.severity {
            IssueSeverity::Critical => deductions += 25,
            IssueSeverity::Warning => deductions += 10,
            IssueSeverity::Info => deductions += 3,
        }
    }
    deductions += (report.junk_files.len() as u32) * 5;
    deductions += (report.empty_files.len() as u32) * 2;

    report.health_score = 100_u32.saturating_sub(deductions);

    Ok(report)
}

/// Print formatted audit report to console
pub fn print_audit_report(root: &Path, report: &AuditReport) {
    println!("\n{}", "==================================================".dimmed());
    println!(
        "{} Project Health & Maintenance Report: {}",
        "[AUDIT]".cyan().bold(),
        root.display().to_string().bold()
    );
    println!("{}\n", "==================================================".dimmed());

    println!("  • Project Profile : {}", report.project_type.green().bold());
    if let Some(ref tpl) = report.detected_template {
        println!("  • Matched Template: {}", tpl.cyan());
    }
    println!("  • Directories     : {}", report.total_dirs);
    println!("  • Tracked Files   : {}", report.total_files);

    let score_colored = if report.health_score >= 90 {
        format!("{}/100 (Excellent)", report.health_score).green().bold()
    } else if report.health_score >= 70 {
        format!("{}/100 (Good)", report.health_score).yellow().bold()
    } else {
        format!("{}/100 (Needs Maintenance)", report.health_score).red().bold()
    };
    println!("  • Health Score    : {}\n", score_colored);

    // Issues
    if report.issues.is_empty() {
        print_success("No structural issues detected!");
    } else {
        println!("{}", "Issues Detected:".bold());
        for issue in &report.issues {
            let tag = match issue.severity {
                IssueSeverity::Critical => "[CRITICAL]".red().bold(),
                IssueSeverity::Warning => "[WARNING] ".yellow().bold(),
                IssueSeverity::Info => "[INFO]    ".blue(),
            };
            println!("  {} {}", tag, issue.title.bold());
            println!("             {}", issue.message.dimmed());
            if let Some(ref hint) = issue.fix_hint {
                println!("             Fix: {}", hint.cyan());
            }
        }
        println!();
    }

    // Junk files
    if !report.junk_files.is_empty() {
        println!(
            "{} Found {} junk / artifact file(s):",
            "[CLEAN]".yellow().bold(),
            report.junk_files.len()
        );
        for f in &report.junk_files {
            println!("  - {}", f.display().to_string().yellow());
        }
        println!("  (Run '{}' to remove them)\n", "seed sync --clean".bold());
    }

    // Empty files
    if !report.empty_files.is_empty() {
        println!(
            "{} Found {} zero-byte file(s):",
            "[EMPTY]".dimmed(),
            report.empty_files.len()
        );
        for f in &report.empty_files {
            println!("  - {}", f.display());
        }
        println!();
    }
}

#[derive(Debug, Default)]
pub struct SyncOptions {
    pub clean_junk: bool,
    pub init_git: bool,
    pub force: bool,
    pub license: Option<String>,
    pub author: Option<String>,
}

#[derive(Debug, Default)]
pub struct SyncResult {
    pub created_files: usize,
    pub created_dirs: usize,
    pub cleaned_files: usize,
}

/// Maintain and synchronize project structure
pub fn sync_project(
    root: &Path,
    template: Option<&Template>,
    opts: &SyncOptions,
) -> Result<SyncResult> {
    let mut result = SyncResult::default();

    println!(
        "\n{} Maintaining project at '{}'...\n",
        "[SYNC]".cyan().bold(),
        root.display().to_string().bold()
    );

    let proj_name = root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("project")
        .to_string();

    let author = opts
        .author
        .clone()
        .or_else(crate::utils::get_git_user)
        .unwrap_or_else(|| "Maintainer".to_string());

    let license = opts.license.clone().unwrap_or_else(|| "MIT".to_string());

    // If template specified, create any missing files/directories
    if let Some(tpl) = template {
        let mut vars = tpl.variables.clone();
        vars.insert("project_name".into(), proj_name.clone());
        vars.insert("author".into(), author.clone());
        vars.insert("license".into(), license.clone());
        vars.insert("year".into(), current_year().to_string());
        vars.insert("date".into(), chrono::Utc::now().format("%Y-%m-%d").to_string());

        let rendered = tpl.render(&vars);

        for dir in &rendered.directories {
            let dir_path = root.join(dir);
            if !dir_path.exists() {
                fs::create_dir_all(&dir_path)?;
                println!("  {} Created missing directory '{}'", "dir ".blue(), dir);
                result.created_dirs += 1;
            }
        }

        for file in &rendered.files {
            let file_path = root.join(&file.path);
            if !file_path.exists() {
                if let Some(p) = file_path.parent() {
                    fs::create_dir_all(p)?;
                }
                fs::write(&file_path, &file.content)?;
                println!("  {} Restored missing file '{}'", "file".green(), file.path);
                result.created_files += 1;
            } else if opts.force {
                fs::write(&file_path, &file.content)?;
                println!("  {} Overwrote file '{}'", "file".yellow(), file.path);
                result.created_files += 1;
            }
        }
    }

    // Ensure README.md exists
    let readme_path = root.join("README.md");
    if !readme_path.exists() {
        let content = format!("# {}\n\nA maintained project.\n", proj_name);
        fs::write(&readme_path, content)?;
        println!("  {} Created default README.md", "file".green());
        result.created_files += 1;
    }

    // Ensure .gitignore exists
    let gitignore_path = root.join(".gitignore");
    if !gitignore_path.exists() {
        let content = ".DS_Store\nThumbs.db\n*.tmp\n*.bak\n/target\nnode_modules/\n.venv/\n";
        fs::write(&gitignore_path, content)?;
        println!("  {} Created default .gitignore", "file".green());
        result.created_files += 1;
    }

    // Ensure LICENSE exists
    let license_path = root.join("LICENSE");
    if !license_path.exists() {
        let content = get_license_text(&license, &author, current_year());
        fs::write(&license_path, content)?;
        println!("  {} Created default LICENSE ({})", "file".green(), license);
        result.created_files += 1;
    }

    // Clean junk files if requested
    if opts.clean_junk {
        for entry in WalkDir::new(root).into_iter().filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            name != ".git" && name != "target" && name != "node_modules"
        }).flatten() {
            let p = entry.path();
            if p.is_file() && is_junk_file(p)
                && let Ok(()) = fs::remove_file(p) {
                    println!("  {} Removed junk file: {}", "junk".red(), p.display());
                    result.cleaned_files += 1;
                }
        }
    }

    // Git init if requested and missing
    if opts.init_git && !root.join(".git").exists() {
        let _ = crate::utils::init_git_repo(root);
        print_info("Initialized Git repository");
    }

    println!();
    print_success(&format!(
        "Maintenance complete! Added {} files, created {} dirs, cleaned {} junk files.",
        result.created_files, result.created_dirs, result.cleaned_files
    ));

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_is_junk_file() {
        assert!(is_junk_file(Path::new(".DS_Store")));
        assert!(is_junk_file(Path::new("subdir/Thumbs.db")));
        assert!(is_junk_file(Path::new("file.tmp")));
        assert!(is_junk_file(Path::new("file.bak")));
        assert!(is_junk_file(Path::new("file.swp")));
        assert!(!is_junk_file(Path::new("main.rs")));
        assert!(!is_junk_file(Path::new("Cargo.toml")));
    }

    #[test]
    fn test_detect_project_type() {
        let tmp = tempdir().unwrap();
        assert_eq!(detect_project_type(tmp.path()), None);

        fs::write(tmp.path().join("Cargo.toml"), "[package]").unwrap();
        assert_eq!(detect_project_type(tmp.path()), Some("rust"));

        fs::write(tmp.path().join("CMakeLists.txt"), "project()").unwrap();
        assert_eq!(detect_project_type(tmp.path()), Some("rust")); // Cargo.toml checked first

        let tmp2 = tempdir().unwrap();
        fs::write(tmp2.path().join("CMakeLists.txt"), "project()").unwrap();
        assert_eq!(detect_project_type(tmp2.path()), Some("cpp"));

        let tmp3 = tempdir().unwrap();
        fs::create_dir_all(tmp3.path().join("episodes")).unwrap();
        assert_eq!(detect_project_type(tmp3.path()), Some("podcast"));
    }

    #[test]
    fn test_audit_and_sync_project() {
        let tmp = tempdir().unwrap();
        let root = tmp.path();

        // Initially empty, should report missing standard files
        let report = audit_project(root, None).unwrap();
        assert!(report.health_score < 100);
        assert!(!report.issues.is_empty());

        // Create empty dir and test empty_dirs detection
        let empty_sub = root.join("empty_folder");
        fs::create_dir_all(&empty_sub).unwrap();
        let report_with_empty = audit_project(root, None).unwrap();
        assert_eq!(report_with_empty.empty_dirs.len(), 1);
        assert_eq!(report_with_empty.empty_dirs[0], PathBuf::from("empty_folder"));

        // Create junk file
        fs::write(root.join(".DS_Store"), "junk").unwrap();
        let report_with_junk = audit_project(root, None).unwrap();
        assert_eq!(report_with_junk.junk_files.len(), 1);

        // Run sync with clean_junk = true
        let sync_opts = SyncOptions {
            clean_junk: true,
            init_git: false,
            force: false,
            license: Some("MIT".into()),
            author: Some("Tester".into()),
        };
        let sync_res = sync_project(root, None, &sync_opts).unwrap();
        assert_eq!(sync_res.cleaned_files, 1);
        assert!(!root.join(".DS_Store").exists());
        assert!(root.join("README.md").exists());
        assert!(root.join(".gitignore").exists());
        assert!(root.join("LICENSE").exists());
    }
}
