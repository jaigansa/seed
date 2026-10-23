use crate::template::{Template, TemplateFile};
use crate::utils::print_success;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct CaptureOptions {
    pub name: String,
    pub description: String,
    pub output_path: PathBuf,
    pub replace_name_with_var: bool,
    pub max_file_size_kb: usize,
}

pub fn capture_directory_as_template(root: &Path, opts: &CaptureOptions) -> Result<()> {
    if !root.exists() || !root.is_dir() {
        anyhow::bail!("Directory '{}' does not exist", root.display());
    }

    let project_dir_name = root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("project");

    let mut template = Template::new_manual(&opts.name, &opts.description);
    let max_bytes = (opts.max_file_size_kb * 1024) as u64;

    for entry in WalkDir::new(root).into_iter().filter_entry(|e| {
        let name = e.file_name().to_string_lossy();
        !name.starts_with(".git")
            && name != "target"
            && name != "node_modules"
            && name != ".venv"
            && name != "dist"
            && name != "build"
            && !name.ends_with(".lock")
    }) {
        let entry = entry.context("Failed walking directory")?;
        let path = entry.path();

        if path == root {
            continue;
        }

        let Ok(rel_path) = path.strip_prefix(root) else {
            continue;
        };
        let mut rel_str = rel_path.to_string_lossy().to_string();

        if opts.replace_name_with_var && !project_dir_name.is_empty() {
            rel_str = rel_str.replace(project_dir_name, "{{project_name}}");
        }

        if path.is_dir() {
            template.directories.push(rel_str);
        } else if path.is_file() {
            let metadata = path.metadata().context("Failed to get file metadata")?;
            if metadata.len() > max_bytes {
                eprintln!(
                    "Skipping large file '{}' ({} KB > {} KB max)",
                    rel_str,
                    metadata.len() / 1024,
                    opts.max_file_size_kb
                );
                continue;
            }

            // Check if file is valid UTF-8 text
            match fs::read_to_string(path) {
                Ok(mut content) => {
                    if opts.replace_name_with_var && !project_dir_name.is_empty() {
                        content = content.replace(project_dir_name, "{{project_name}}");
                    }

                    let is_executable = {
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            metadata.permissions().mode() & 0o111 != 0
                        }
                        #[cfg(not(unix))]
                        {
                            false
                        }
                    };

                    template.files.push(TemplateFile {
                        path: rel_str,
                        content,
                        required: true,
                        executable: is_executable,
                    });
                }
                Err(_) => {
                    eprintln!("Skipping binary file '{}'", rel_str);
                }
            }
        }
    }

    // Sort entries for consistency
    template.directories.sort();
    template.files.sort_by(|a, b| a.path.cmp(&b.path));

    let output_str = if opts
        .output_path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("toml"))
        .unwrap_or(false)
    {
        template.to_toml()?
    } else {
        template.to_yaml()?
    };

    if let Some(parent) = opts.output_path.parent()
        && !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }

    fs::write(&opts.output_path, output_str).with_context(|| {
        format!(
            "Failed to write template output to '{}'",
            opts.output_path.display()
        )
    })?;

    print_success(&format!(
        "Captured template '{}' ({} directories, {} files) to {}",
        template.name,
        template.directories.len(),
        template.files.len(),
        opts.output_path.display()
    ));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_capture_directory() {
        let tmp = tempdir().unwrap();
        let src_dir = tmp.path().join("source_proj");
        fs::create_dir_all(src_dir.join("src")).unwrap();
        fs::write(src_dir.join("src/main.rs"), "fn main() {}").unwrap();
        fs::write(src_dir.join("README.md"), "# source_proj").unwrap();

        let out_file = tmp.path().join("captured.yaml");

        let opts = CaptureOptions {
            name: "test-capture".into(),
            description: "testing capture".into(),
            output_path: out_file.clone(),
            replace_name_with_var: true,
            max_file_size_kb: 100,
        };

        capture_directory_as_template(&src_dir, &opts).unwrap();

        assert!(out_file.exists());
        let content = fs::read_to_string(&out_file).unwrap();
        let tpl = Template::from_yaml(&content).unwrap();
        assert_eq!(tpl.name, "test-capture");
        assert!(tpl.files.iter().any(|f| f.path == "src/main.rs"));
        assert!(tpl.files.iter().any(|f| f.path == "README.md"));
    }
}
