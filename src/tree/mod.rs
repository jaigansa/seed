use colored::Colorize;
use std::fs;
use std::path::Path;

pub struct TreeOptions {
    pub max_depth: Option<usize>,
    pub show_hidden: bool,
    pub show_sizes: bool,
    pub use_nerd_font: bool,
}

impl Default for TreeOptions {
    fn default() -> Self {
        Self {
            max_depth: Some(4),
            show_hidden: false,
            show_sizes: true,
            use_nerd_font: false,
        }
    }
}

pub struct TreeStats {
    pub directories: usize,
    pub files: usize,
    pub total_bytes: u64,
}

pub fn render_tree(root: &Path, opts: &TreeOptions) -> TreeStats {
    let mut stats = TreeStats {
        directories: 0,
        files: 0,
        total_bytes: 0,
    };

    println!("{}", root.display().to_string().cyan().bold());
    render_dir_node(root, "", 1, opts, &mut stats);

    println!(
        "\n{} directories, {} files ({})",
        stats.directories,
        stats.files,
        format_size(stats.total_bytes).dimmed()
    );

    stats
}

pub fn get_tree_lines(root: &Path, opts: &TreeOptions) -> (Vec<String>, TreeStats) {
    let mut stats = TreeStats {
        directories: 0,
        files: 0,
        total_bytes: 0,
    };
    let mut lines = Vec::new();
    lines.push(format!("[DIR] {}", root.display()));
    collect_dir_node(root, "", 1, opts, &mut stats, &mut lines);
    lines.push(format!(
        "\nTotal: {} directories, {} files ({})",
        stats.directories,
        stats.files,
        format_size(stats.total_bytes)
    ));
    (lines, stats)
}

fn collect_dir_node(
    dir: &Path,
    prefix: &str,
    current_depth: usize,
    opts: &TreeOptions,
    stats: &mut TreeStats,
    lines: &mut Vec<String>,
) {
    if let Some(max) = opts.max_depth
        && current_depth > max {
            return;
        }

    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    let mut items: Vec<_> = entries
        .flatten()
        .filter(|e| {
            if opts.show_hidden {
                return true;
            }
            let name = e.file_name().to_string_lossy().to_string();
            !name.starts_with('.') && name != "target" && name != "node_modules" && name != ".venv"
        })
        .collect();

    items.sort_by(|a, b| {
        let is_dir_a = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
        let is_dir_b = b.file_type().map(|t| t.is_dir()).unwrap_or(false);
        if is_dir_a != is_dir_b {
            is_dir_b.cmp(&is_dir_a)
        } else {
            a.file_name().cmp(&b.file_name())
        }
    });

    let total = items.len();
    for (i, entry) in items.iter().enumerate() {
        let is_last = i == total - 1;
        let branch = if is_last { "└── " } else { "├── " };
        let next_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

        let file_type = entry.file_type();
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();

        if let Ok(ft) = file_type {
            if ft.is_dir() {
                stats.directories += 1;
                let icon_str = if opts.use_nerd_font { crate::icons::folder_icon(true) } else { "" };
                lines.push(format!("{}{}{}{}/", prefix, branch, icon_str, file_name));
                collect_dir_node(&path, &next_prefix, current_depth + 1, opts, stats, lines);
            } else if ft.is_file() {
                stats.files += 1;
                let icon_str = if opts.use_nerd_font { crate::icons::file_icon(&path, true) } else { "" };
                let size_str = if opts.show_sizes {
                    if let Ok(meta) = entry.metadata() {
                        let len = meta.len();
                        stats.total_bytes += len;
                        format!(" ({})", format_size(len))
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                };
                lines.push(format!("{}{}{}{}{}", prefix, branch, icon_str, file_name, size_str));
            }
        }
    }
}

fn render_dir_node(
    dir: &Path,
    prefix: &str,
    current_depth: usize,
    opts: &TreeOptions,
    stats: &mut TreeStats,
) {
    if let Some(max) = opts.max_depth
        && current_depth > max {
            return;
        }

    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    let mut items: Vec<_> = entries
        .flatten()
        .filter(|e| {
            if opts.show_hidden {
                return true;
            }
            let name = e.file_name().to_string_lossy().to_string();
            !name.starts_with('.') && name != "target" && name != "node_modules" && name != ".venv"
        })
        .collect();

    // Sort: directories first, then alphabetically
    items.sort_by(|a, b| {
        let is_dir_a = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
        let is_dir_b = b.file_type().map(|t| t.is_dir()).unwrap_or(false);

        if is_dir_a != is_dir_b {
            is_dir_b.cmp(&is_dir_a)
        } else {
            a.file_name().cmp(&b.file_name())
        }
    });

    let total = items.len();
    for (i, entry) in items.iter().enumerate() {
        let is_last = i == total - 1;
        let branch = if is_last { "└── " } else { "├── " };
        let next_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

        let file_type = entry.file_type();
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();

        if let Ok(ft) = file_type {
            if ft.is_dir() {
                stats.directories += 1;
                let icon_str = if opts.use_nerd_font { crate::icons::folder_icon(true) } else { "" };
                println!("{}{}{}{}/", prefix, branch, icon_str, file_name.blue().bold());
                render_dir_node(&path, &next_prefix, current_depth + 1, opts, stats);
            } else if ft.is_file() {
                stats.files += 1;
                let icon_str = if opts.use_nerd_font { crate::icons::file_icon(&path, true) } else { "" };
                let size_str = if opts.show_sizes {
                    if let Ok(meta) = entry.metadata() {
                        let len = meta.len();
                        stats.total_bytes += len;
                        format!(" ({})", format_size(len))
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                };

                let colored_name = colorize_filename(&file_name);
                println!("{}{}{}{}{}", prefix, branch, icon_str, colored_name, size_str.dimmed());
            }
        }
    }
}

fn colorize_filename(name: &str) -> colored::ColoredString {
    if name.ends_with(".rs")
        || name.ends_with(".py")
        || name.ends_with(".ts")
        || name.ends_with(".js")
        || name.ends_with(".go")
        || name.ends_with(".c")
        || name.ends_with(".cpp")
    {
        name.green()
    } else if name.ends_with(".toml")
        || name.ends_with(".yaml")
        || name.ends_with(".yml")
        || name.ends_with(".json")
    {
        name.yellow()
    } else if name.ends_with(".md") || name.ends_with(".txt") {
        name.cyan()
    } else if name == "Cargo.lock" || name == "package-lock.json" {
        name.dimmed()
    } else if name.starts_with('.') {
        name.magenta()
    } else {
        name.white()
    }
}

pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(2048), "2.0 KB");
        assert_eq!(format_size(1024 * 1024), "1.0 MB");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.0 GB");
    }
}
