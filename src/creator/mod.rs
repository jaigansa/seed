use crate::template::Template;
use crate::utils::{current_year, get_license_text, init_git_repo, print_info, print_success};
use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CreateOptions {
    pub target_dir: PathBuf,
    pub project_name: String,
    pub author: String,
    pub license: String,
    pub init_git: bool,
    pub force: bool,
    pub extra_vars: HashMap<String, String>,
    pub quiet: bool,
}

pub struct CreationSummary {
    pub created_dirs: usize,
    pub created_files: usize,
    pub skipped_files: usize,
}

pub fn create_project(template: &Template, opts: &CreateOptions) -> Result<CreationSummary> {
    let target = &opts.target_dir;

    if target.exists() {
        if !opts.force {
            let mut entries = fs::read_dir(target)
                .with_context(|| format!("Failed to read target directory: {}", target.display()))?;
            if entries.next().is_some() {
                bail!(
                    "Target directory '{}' already exists and is not empty. Use --force to proceed anyway.",
                    target.display()
                );
            }
        }
    } else {
        fs::create_dir_all(target)
            .with_context(|| format!("Failed to create project directory: {}", target.display()))?;
    }

    // Prepare variables
    let year = current_year();
    let mut vars = template.variables.clone();
    vars.insert("project_name".into(), opts.project_name.clone());
    vars.insert("author".into(), opts.author.clone());
    vars.insert("license".into(), opts.license.clone());
    vars.insert("year".into(), year.to_string());
    vars.insert("date".into(), chrono::Utc::now().format("%Y-%m-%d").to_string());

    // Merge extra user variables
    for (k, v) in &opts.extra_vars {
        vars.insert(k.clone(), v.clone());
    }

    let rendered = template.render(&vars);

    let mut summary = CreationSummary {
        created_dirs: 0,
        created_files: 0,
        skipped_files: 0,
    };

    if !opts.quiet {
        println!(
            "\n{} Creating project '{}' using template '{}'...\n",
            "[SEED]".cyan().bold(),
            opts.project_name.bold(),
            template.name.green()
        );
    }

    // 1. Create directories
    for dir in &rendered.directories {
        let dir_path = target.join(dir);
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)
                .with_context(|| format!("Failed to create directory: {}", dir_path.display()))?;
            if !opts.quiet {
                println!("  {} {}/", "dir ".blue().bold(), dir);
            }
            summary.created_dirs += 1;
        }
    }

    // 2. Create template files
    for file in &rendered.files {
        let file_path = target.join(&file.path);

        if let Some(parent) = file_path.parent()
            && !parent.exists() {
                fs::create_dir_all(parent)?;
            }

        if file_path.exists() && !opts.force {
            if !opts.quiet {
                println!("  {} {} (already exists)", "skip".yellow(), file.path);
            }
            summary.skipped_files += 1;
            continue;
        }

        fs::write(&file_path, &file.content)
            .with_context(|| format!("Failed to write file: {}", file_path.display()))?;

        #[cfg(unix)]
        if file.executable {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = file_path.metadata() {
                let mut perms = metadata.permissions();
                perms.set_mode(0o755);
                let _ = fs::set_permissions(&file_path, perms);
            }
        }

        if !opts.quiet {
            println!("  {} {}", "file".green(), file.path);
        }
        summary.created_files += 1;
    }

    // 3. Create LICENSE file if not present in template and license is specified
    let license_path = target.join("LICENSE");
    if !license_path.exists() && !opts.license.is_empty() && opts.license != "None" {
        let license_content = get_license_text(&opts.license, &opts.author, year);
        fs::write(&license_path, license_content)?;
        if !opts.quiet {
            println!("  {} LICENSE ({})", "file".green(), opts.license);
        }
        summary.created_files += 1;
    }

    // 4. Git init
    if opts.init_git {
        let git_dir = target.join(".git");
        if !git_dir.exists() {
            match init_git_repo(target) {
                Ok(_) => {
                    if !opts.quiet {
                        print_info("Initialized empty Git repository");
                    }
                }
                Err(e) => {
                    if !opts.quiet {
                        print_info(&format!("Git initialization skipped: {}", e));
                    }
                }
            }
        }
    }

    if !opts.quiet {
        println!();
        print_success(&format!(
            "Project '{}' successfully seeded at {}",
            opts.project_name.bold(),
            target.display()
        ));
    }

    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_project_flow() {
        let tmp = tempdir().unwrap();
        let target = tmp.path().join("my-app");

        let tpl = Template::new("mini", "mini desc")
            .with_directory("src")
            .with_file("src/main.rs", "fn main() { println!(\"{{project_name}}\"); }", true);

        let opts = CreateOptions {
            target_dir: target.clone(),
            project_name: "my-app".into(),
            author: "Tester".into(),
            license: "MIT".into(),
            init_git: false,
            force: false,
            extra_vars: HashMap::new(),
            quiet: true,
        };

        let summary = create_project(&tpl, &opts).unwrap();
        assert_eq!(summary.created_dirs, 1);
        assert_eq!(summary.created_files, 2); // main.rs + LICENSE

        assert!(target.join("src").is_dir());
        assert!(target.join("src/main.rs").is_file());
        assert!(target.join("LICENSE").is_file());

        let main_content = fs::read_to_string(target.join("src/main.rs")).unwrap();
        assert!(main_content.contains("my-app"));
    }
}
