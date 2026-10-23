use crate::creator::{create_project, CreateOptions};
use crate::template::{
    builtin::get_builtin_templates,
    loader::{list_custom_templates, resolve_template},
};
use crate::utils::get_git_user;
use anyhow::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

pub fn run_interactive_init(
    target_dir: PathBuf,
    preselected_template: Option<String>,
) -> Result<()> {
    println!(
        "\n{} {}",
        "Welcome to Seed Project Initializer!".green().bold(),
        "Let's set up your project.".dimmed()
    );

    let default_name = target_dir
        .canonicalize()
        .ok()
        .and_then(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "my-project".to_string());

    let theme = ColorfulTheme::default();

    // 1. Template selection
    let template = match preselected_template {
        Some(t) => resolve_template(&t)?,
        None => {
            let builtins = get_builtin_templates();
            let mut options: Vec<String> = builtins
                .iter()
                .map(|t| format!("{:<12} - {}", t.name.bold(), t.description))
                .collect();

            let custom = list_custom_templates();
            for (name, path) in &custom {
                options.push(format!("{:<12} - Custom ({})", name.bold(), path.display()));
            }

            let selection = Select::with_theme(&theme)
                .with_prompt("Select project template")
                .default(0)
                .items(&options)
                .interact()?;

            if selection < builtins.len() {
                builtins[selection].clone()
            } else {
                let custom_idx = selection - builtins.len();
                resolve_template(&custom[custom_idx].0)?
            }
        }
    };

    // 2. Project name
    let project_name: String = Input::with_theme(&theme)
        .with_prompt("Project name")
        .default(default_name)
        .interact_text()?;

    // 3. Author
    let detected_author = get_git_user()
        .or_else(|| env::var("USER").ok())
        .unwrap_or_else(|| "Developer".to_string());

    let author: String = Input::with_theme(&theme)
        .with_prompt("Author name")
        .default(detected_author)
        .interact_text()?;

    // 4. License
    let licenses = vec!["MIT", "Apache-2.0", "BSD-3-Clause", "Unlicense", "None"];
    let lic_selection = Select::with_theme(&theme)
        .with_prompt("Select license")
        .default(0)
        .items(&licenses)
        .interact()?;
    let license = licenses[lic_selection].to_string();

    // 5. Git init
    let init_git = Confirm::with_theme(&theme)
        .with_prompt("Initialize Git repository?")
        .default(true)
        .interact()?;

    let opts = CreateOptions {
        target_dir,
        project_name,
        author,
        license,
        init_git,
        force: false,
        extra_vars: HashMap::new(),
        quiet: false,
    };

    create_project(&template, &opts)?;
    Ok(())
}
