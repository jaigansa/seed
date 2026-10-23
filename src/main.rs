mod capture;
mod cli;
mod config;
mod creator;
mod icons;
mod maintainer;
mod template;
mod tree;
mod tui;
mod utils;
mod wizard;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use creator::{create_project, CreateOptions};
use maintainer::{audit_project, print_audit_report, sync_project, SyncOptions};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use template::{
    builtin::get_builtin_templates,
    loader::{
        get_local_templates_dir, get_user_templates_dir, init_local_templates_dir,
        init_user_templates_dir, list_custom_templates, resolve_template, save_manual_template,
    },
    Template, TemplateFile,
};
use utils::get_git_user;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let command = cli.command.unwrap_or_else(|| Commands::Tui(cli::TuiArgs::default()));

    match command {
        Commands::Tui(args) => {
            tui::run_tui(args.path)?;
        }

        Commands::InitWorkspace(args) => {
            let mut cfg = config::load_config();
            if let Some(p) = args.path {
                cfg.workspace_root = p.to_string_lossy().to_string();
                let _ = config::save_config(&cfg);
            }
            let summary = cfg.init_master_workspace()?;
            println!(
                "\n{} Master Workspace initialized at '{}'!\n  + Created {} directories\n  - Placed {} structure guides\n",
                "[SEED]".green().bold(),
                cfg.workspace_root.bold(),
                summary.created_dirs,
                summary.created_readme_files
            );
            println!("Structure:\n  ├── {}/ (Personal work, experiments, portfolio)", cfg.personal_dir.cyan());
            println!("  ├── {}/ (Client projects & deliverables)", cfg.clients_dir.cyan());
            println!("  ├── {}/ (3D, audio, textures, fonts, stock)", cfg.resources_dir.cyan());
            println!("  └── {}/ (Yearly cold-storage archives)\n", cfg.archive_dir.cyan());
        }

        Commands::Archive(args) => {
            let cfg = config::load_config();
            let dest = cfg.archive_project(&args.path)?;
            println!(
                "{} Archived project to '{}'",
                "[ARCHIVE]".cyan().bold(),
                dest.display().to_string().bold()
            );
        }

        Commands::New(args) => {
            let cfg = config::load_config();
            let template_name = if args.manual && args.template == "yt-video" {
                "manual"
            } else {
                &args.template
            };
            let template = resolve_template(template_name)?;
            let (project_name, target_dir) = match args.dir {
                Some(dir) => (args.name, dir),
                None => {
                    let path = PathBuf::from(&args.name);
                    let clean_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(&args.name)
                        .to_string();

                    // If path is simple name, route according to category
                    if !args.name.contains('/') && !args.name.contains('\\') {
                        let cat_dir = match args.category.to_lowercase().as_str() {
                            "client" => {
                                let c_name = args.client.clone().unwrap_or_else(|| "unnamed_client".to_string());
                                cfg.clients_path().join(c_name).join(&clean_name)
                            }
                            "resource" => cfg.resources_path().join(&clean_name),
                            _ => cfg.personal_path().join(&clean_name),
                        };
                        (clean_name, cat_dir)
                    } else {
                        (clean_name, path)
                    }
                }
            };

            let author = args
                .author
                .unwrap_or_else(|| cfg.default_author.clone());

            let mut extra_vars: HashMap<String, String> = HashMap::new();
            if let Some(ref c) = args.client {
                extra_vars.insert("client_name".to_string(), c.clone());
            }
            for (k, v) in args.vars {
                extra_vars.insert(k, v);
            }

            let opts = CreateOptions {
                target_dir,
                project_name,
                author,
                license: args.license,
                init_git: args.git,
                force: args.force,
                extra_vars,
                quiet: false,
            };

            create_project(&template, &opts)?;
        }

        Commands::Init(args) => {
            if args.non_interactive {
                let template_name = args.template.unwrap_or_else(|| "minimal".to_string());
                let template = resolve_template(&template_name)?;
                let proj_name = args
                    .path
                    .canonicalize()
                    .ok()
                    .and_then(|p| {
                        p.file_name()
                            .and_then(|n| n.to_str())
                            .map(|s| s.to_string())
                    })
                    .unwrap_or_else(|| "project".to_string());

                let author = get_git_user().unwrap_or_else(|| "Developer".to_string());

                let opts = CreateOptions {
                    target_dir: args.path,
                    project_name: proj_name,
                    author,
                    license: "MIT".to_string(),
                    init_git: true,
                    force: false,
                    extra_vars: HashMap::new(),
                    quiet: false,
                };

                create_project(&template, &opts)?;
            } else {
                wizard::run_interactive_init(args.path, args.template)?;
            }
        }

        Commands::Check(args) => {
            let template = match args.template {
                Some(ref t) => Some(resolve_template(t)?),
                None => None,
            };

            let report = audit_project(&args.path, template.as_ref())?;
            print_audit_report(&args.path, &report);
        }

        Commands::Sync(args) => {
            let template = match args.template {
                Some(ref t) => Some(resolve_template(t)?),
                None => {
                    // Try to auto-detect template from directory
                    maintainer::detect_project_type(&args.path)
                        .and_then(template::builtin::find_builtin_template)
                }
            };

            let opts = SyncOptions {
                clean_junk: args.clean,
                init_git: args.git,
                force: args.force,
                license: args.license,
                author: args.author,
            };

            sync_project(&args.path, template.as_ref(), &opts)?;
        }

        Commands::List(args) => {
            let cfg = config::load_config();
            println!(
                "\n{}\n",
                "Available Templates for Project Creation:".bold()
            );

            let all_builtins = get_builtin_templates();
            let sorted_builtins = template::sort_templates_with_favorites(all_builtins, &cfg.favorite_templates);
            let use_nf = cfg.use_nerd_font();

            if !args.manual {
                println!("{}", "Built-in Preset Templates:".underline().bold());
                for tpl in sorted_builtins {
                    let is_fav = cfg.is_favorite(&tpl.name);
                    let fav_badge = if is_fav {
                        format!(" {}", crate::icons::fav_badge(true, use_nf).yellow().bold())
                    } else {
                        "".to_string()
                    };
                    let out_str = if !tpl.outputs.is_empty() {
                        format!(" -> Deliverables: {}", tpl.outputs.join(", "))
                    } else {
                        String::new()
                    };
                    println!(
                        "  {:<15} [Preset] {:<47} (dirs: {}, files: {}){}{}",
                        tpl.name.green().bold(),
                        tpl.description.dimmed(),
                        tpl.directories.len(),
                        tpl.files.len(),
                        fav_badge,
                        out_str.cyan()
                    );
                }
            }

            if !args.preset {
                let custom = list_custom_templates();
                if !custom.is_empty() {
                    println!("\n{}", "Manual User Templates:".underline().bold());
                    for (name, path) in custom {
                        let is_fav = cfg.is_favorite(&name);
                        let fav_badge = if is_fav {
                            format!(" {}", crate::icons::fav_badge(true, use_nf).yellow().bold())
                        } else {
                            "".to_string()
                        };
                        println!(
                            "  {:<15} [Manual] {}{}",
                            name.cyan().bold(),
                            path.display().to_string().dimmed(),
                            fav_badge
                        );
                    }
                } else if args.manual {
                    println!(
                        "\n{}",
                        "No manual templates found. Use 'seed template-new <name>' or create files in ~/.config/seed/templates/"
                            .dimmed()
                    );
                } else {
                    println!(
                        "\n{}",
                        "Manual templates can be created with 'seed template-new <name>' or placed in ~/.config/seed/templates/"
                            .dimmed()
                    );
                }
            }
            println!();
        }

        Commands::TemplateNew(args) => {
            let cfg = config::load_config();
            let trimmed_name = args.name.trim();
            if trimmed_name.is_empty() {
                anyhow::bail!("Template name cannot be empty");
            }

            let dirs: Vec<String> = args
                .dirs
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let dirs = if dirs.is_empty() { vec!["src".to_string()] } else { dirs };

            let file_names: Vec<String> = args
                .files
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let file_names = if file_names.is_empty() { vec!["README.md".to_string()] } else { file_names };

            let files: Vec<TemplateFile> = file_names
                .into_iter()
                .map(|path| {
                    let content = if path.ends_with(".md") {
                        format!("# {{{{project_name}}}}\n\n{}\n", args.description)
                    } else if path.ends_with(".gitignore") {
                        "*.tmp\n*.bak\n.DS_Store\n".to_string()
                    } else if path.ends_with(".json") {
                        "{\n  \"name\": \"{{project_name}}\"\n}\n".to_string()
                    } else {
                        format!("// Starter file: {}\n", path)
                    };
                    TemplateFile {
                        path,
                        content,
                        required: true,
                        executable: false,
                    }
                })
                .collect();

            let outputs: Vec<String> = args
                .outputs
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let outputs = if outputs.is_empty() { vec!["Deliverable".to_string()] } else { outputs };

            let mut tpl = Template::new_manual(trimmed_name, &args.description);
            tpl.directories = dirs.clone();
            tpl.files = files;
            tpl.outputs = outputs.clone();

            let target_dir = if args.local {
                get_local_templates_dir()
            } else if let Some(user_dir) = get_user_templates_dir() {
                user_dir
            } else {
                PathBuf::from(&cfg.custom_templates_dir)
            };

            let saved_path = save_manual_template(&tpl, &target_dir)?;
            println!(
                "\n{} Created manual template '{}' successfully!\n  Saved to: {}\n  Directories: {}\n  Deliverables: {}\n",
                "[TEMPLATE]".green().bold(),
                tpl.name.bold(),
                saved_path.display().to_string().cyan(),
                dirs.join(", "),
                outputs.join(", ")
            );
        }

        Commands::Info(args) => {
            let tpl = resolve_template(&args.template)?;
            let cfg = config::load_config();
            let use_nf = cfg.use_nerd_font();
            println!("\nTemplate: {}", tpl.name.bold());
            println!("  Description: {}", tpl.description);
            println!("  Version    : {}", tpl.version);

            if !tpl.outputs.is_empty() {
                println!("\n  Target Deliverables & Outputs ({}):", tpl.outputs.len());
                for out in &tpl.outputs {
                    println!("    • {}", out.yellow().bold());
                }
            }

            if !tpl.variables.is_empty() {
                println!("\n  Default Variables:");
                for (k, v) in &tpl.variables {
                    println!("    • {}: {}", k.cyan(), v);
                }
            }

            println!("\n  Directories ({}):", tpl.directories.len());
            let d_icon = if use_nf { crate::icons::folder_icon(true) } else { "+ " };
            for d in &tpl.directories {
                println!("    {} {}/", d_icon.trim(), d);
            }

            println!("\n  Files ({}):", tpl.files.len());
            for f in &tpl.files {
                let f_icon = if use_nf { crate::icons::file_icon(Path::new(&f.path), true) } else { "- " };
                let req = if f.required {
                    "[required]".yellow()
                } else {
                    "[optional]".dimmed()
                };
                let exec = if f.executable { " [exec]".green() } else { "".normal() };
                println!("    {} {:<35} {}{}", f_icon.trim(), f.path, req, exec);
            }
            println!();
        }

        Commands::Tree(args) => {
            let cfg = config::load_config();
            let opts = tree::TreeOptions {
                max_depth: Some(args.depth),
                show_hidden: args.all,
                show_sizes: !args.no_sizes,
                use_nerd_font: cfg.use_nerd_font(),
            };
            tree::render_tree(&args.path, &opts);
        }

        Commands::Capture(args) => {
            let default_name = args
                .name
                .or_else(|| {
                    args.path
                        .canonicalize()
                        .ok()
                        .and_then(|p| p.file_name().and_then(|n| n.to_str()).map(|s| s.to_string()))
                })
                .unwrap_or_else(|| "custom-template".to_string());

            let opts = capture::CaptureOptions {
                name: default_name,
                description: args.desc,
                output_path: args.output,
                replace_name_with_var: !args.no_replace,
                max_file_size_kb: args.max_size_kb,
            };

            capture::capture_directory_as_template(&args.path, &opts)?;
        }

        Commands::InitTemplates(args) => {
            if args.user && args.local {
                let u = init_user_templates_dir()?;
                let l = init_local_templates_dir()?;
                println!(
                    "\n{} Initialized template folders:\n  - {}\n  - {}\n",
                    "[OK]".green().bold(),
                    u.display().to_string().cyan(),
                    l.display().to_string().cyan()
                );
            } else if args.user {
                let u = init_user_templates_dir()?;
                println!("\n{} Initialized user templates folder:\n  {}\n", "[OK]".green().bold(), u.display().to_string().cyan());
            } else if args.local {
                let l = init_local_templates_dir()?;
                println!("\n{} Initialized local templates folder:\n  {}\n", "[OK]".green().bold(), l.display().to_string().cyan());
            } else {
                let u = init_user_templates_dir()?;
                let l = init_local_templates_dir()?;
                println!(
                    "\n{} Initialized template folders:\n  - {}\n  - {}\n",
                    "[OK]".green().bold(),
                    u.display().to_string().cyan(),
                    l.display().to_string().cyan()
                );
            }
        }

        Commands::Fav(args) => {
            let mut cfg = config::load_config();

            if args.clear {
                cfg.favorite_templates.clear();
                config::save_config(&cfg)?;
                println!("\n{} Cleared all favorite templates.\n", "[FAV]".cyan().bold());
            } else if let Some(ref name) = args.template {
                if args.remove {
                    if cfg.is_favorite(name) {
                        cfg.toggle_favorite(name);
                        config::save_config(&cfg)?;
                        println!("\n{} Removed '{}' from favorite templates.\n", "[FAV]".yellow().bold(), name);
                    } else {
                        println!("\nTemplate '{}' is not in favorites.\n", name);
                    }
                } else {
                    if resolve_template(name).is_err() {
                        eprintln!("Warning: '{}' was not found in built-in or custom templates.", name);
                    }
                    let added = cfg.toggle_favorite(name);
                    config::save_config(&cfg)?;
                    if added {
                        println!(
                            "\n{} Pinned '{}' to favorites! It will now appear at the very top of template lists.\n",
                            "[FAV]".green().bold(),
                            name.bold()
                        );
                    } else {
                        println!(
                            "\n{} Removed '{}' from favorites.\n",
                            "[FAV]".yellow().bold(),
                            name
                        );
                    }
                }
            } else {
                if cfg.favorite_templates.is_empty() {
                    println!("\nNo favorite templates pinned yet.");
                    println!("To pin a template to the top of the list:");
                    println!("  CLI: seed fav <template-name>");
                    println!("  TUI: Press [F] on any template\n");
                } else {
                    println!("\n{}", "Pinned Favorite Templates (moved to top of list):".bold());
                    for (i, fav) in cfg.favorite_templates.iter().enumerate() {
                        println!("  {}. {:<14} {}", i + 1, fav.green().bold(), "[FAV]".yellow().bold());
                    }
                    println!("\nTip: Use 'seed fav <name>' to toggle or 'seed fav --clear' to reset.\n");
                }
            }
        }

        Commands::Icons(args) => {
            let mut cfg = config::load_config();
            if let Some(ref mode) = args.mode {
                let m = mode.to_lowercase();
                if m == "nerdfont" || m == "nerd-font" || m == "on" || m == "true" || m == "1" {
                    cfg.icon_mode = "nerdfont".to_string();
                    config::save_config(&cfg)?;
                    println!("\n{} Icon mode set to '{}' (Nerd Font glyphs enabled).\n", "[OK]".green().bold(), "nerdfont".cyan().bold());
                } else if m == "ascii" || m == "off" || m == "false" || m == "0" {
                    cfg.icon_mode = "ascii".to_string();
                    config::save_config(&cfg)?;
                    println!("\n{} Icon mode set to '{}' (Standard ASCII characters enabled).\n", "[OK]".green().bold(), "ascii".cyan().bold());
                } else if m == "toggle" {
                    cfg.cycle_icon_mode();
                    config::save_config(&cfg)?;
                    println!("\n{} Icon mode toggled to '{}'.\n", "[OK]".green().bold(), cfg.icon_mode.cyan().bold());
                } else {
                    eprintln!("Unknown icon mode '{}'. Valid choices: 'nerdfont' ('on'), 'ascii' ('off'), 'toggle'", mode);
                }
            } else {
                let use_nf = cfg.use_nerd_font();
                println!("\n{}", "Seed Icon Configuration:".bold().underline());
                println!("  Current mode: {} (Nerd Font: {})", cfg.icon_mode.cyan().bold(), if use_nf { "ACTIVE".green().bold() } else { "DISABLED".yellow() });
                println!("\n  Preview:");
                println!("    Folder:   {}{}", crate::icons::folder_icon(use_nf), "my-project/");
                println!("    Rust:     {}{}", crate::icons::file_icon(Path::new("main.rs"), use_nf), "main.rs");
                println!("    Python:   {}{}", crate::icons::file_icon(Path::new("app.py"), use_nf), "app.py");
                println!("    Favorite: {}{}", crate::icons::fav_badge(true, use_nf), "pinned template");
                println!("\n  To switch mode:");
                println!("    seed icons nerdfont   (Enable Nerd Font glyphs)");
                println!("    seed icons ascii      (Use standard ASCII)");
                println!("    seed icons toggle     (Toggle between modes)");
                println!("\n  Prerequisite for Nerd Font:");
                println!("    Ensure your terminal font is set to a Nerd Font (e.g. JetBrainsMono NF, FiraCode NF).");
                println!();
            }
        }
    }

    Ok(())
}
