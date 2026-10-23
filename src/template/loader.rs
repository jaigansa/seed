use super::{builtin::find_builtin_template, Template};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_user_templates_dir() -> Option<PathBuf> {
    if let Ok(home) = std::env::var("HOME") {
        Some(PathBuf::from(home).join(".config").join("seed").join("templates"))
    } else {
        None
    }
}

pub fn get_local_templates_dir() -> PathBuf {
    PathBuf::from(".seed").join("templates")
}

pub fn init_user_templates_dir() -> Result<PathBuf> {
    if let Some(dir) = get_user_templates_dir() {
        fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create user templates directory: {}", dir.display()))?;
        Ok(dir)
    } else {
        anyhow::bail!("Could not determine user templates directory (HOME not set)")
    }
}

pub fn init_local_templates_dir() -> Result<PathBuf> {
    let dir = get_local_templates_dir();
    fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create local templates directory: {}", dir.display()))?;
    Ok(dir)
}

pub fn load_template_from_file(path: &Path) -> Result<Template> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read template file: {}", path.display()))?;

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let mut template = match ext.as_str() {
        "toml" => Template::from_toml(&content)
            .with_context(|| format!("Failed to parse TOML template: {}", path.display()))?,
        "yaml" | "yml" => Template::from_yaml(&content)
            .with_context(|| format!("Failed to parse YAML template: {}", path.display()))?,
        "json" => serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse JSON template: {}", path.display()))?,
        _ => {
            // Try YAML first, then TOML
            Template::from_yaml(&content)
                .or_else(|_| Template::from_toml(&content))
                .with_context(|| {
                    format!(
                        "Failed to parse template as YAML or TOML: {}",
                        path.display()
                    )
                })?
        }
    };

    if template.kind == super::TemplateKind::Preset {
        template.kind = super::TemplateKind::Manual;
    }
    template.source_path = Some(path.display().to_string());
    Ok(template)
}

pub fn resolve_template(name_or_path: &str) -> Result<Template> {
    // 1. Direct path check
    let direct_path = Path::new(name_or_path);
    if direct_path.exists() && direct_path.is_file() {
        return load_template_from_file(direct_path);
    }

    // 2. Check local templates dir (.seed/templates/<name>.[yaml|yml|toml])
    let local_dir = get_local_templates_dir();
    if let Some(t) = search_dir_for_template(&local_dir, name_or_path) {
        return Ok(t);
    }

    // 3. Check user templates dir (~/.config/seed/templates/<name>.[yaml|yml|toml])
    if let Some(user_dir) = get_user_templates_dir()
        && let Some(t) = search_dir_for_template(&user_dir, name_or_path) {
            return Ok(t);
        }

    // 4. Built-in templates
    if let Some(builtin) = find_builtin_template(name_or_path) {
        return Ok(builtin);
    }

    anyhow::bail!(
        "Template '{}' not found. Run 'seed list' to see available templates.",
        name_or_path
    );
}

fn search_dir_for_template(dir: &Path, name: &str) -> Option<Template> {
    if !dir.exists() || !dir.is_dir() {
        return None;
    }

    let extensions = ["yaml", "yml", "toml", "json"];
    for ext in &extensions {
        let candidate = dir.join(format!("{}.{}", name, ext));
        if candidate.is_file()
            && let Ok(t) = load_template_from_file(&candidate) {
                return Some(t);
            }
    }
    None
}

pub fn list_custom_templates() -> Vec<(String, PathBuf)> {
    let mut results = Vec::new();

    let mut check_dir = |dir: &Path| {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file()
                    && let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
                        results.push((stem.to_string(), p.clone()));
                    }
            }
        }
    };

    let local_dir = get_local_templates_dir();
    check_dir(&local_dir);

    if let Some(user_dir) = get_user_templates_dir() {
        check_dir(&user_dir);
    }

    results
}

pub fn save_manual_template(template: &Template, target_dir: &Path) -> Result<PathBuf> {
    fs::create_dir_all(target_dir)
        .with_context(|| format!("Failed to create template directory: {}", target_dir.display()))?;

    let safe_stem = template
        .name
        .trim()
        .to_lowercase()
        .replace([' ', '/', '\\'], "-");
    let file_name = format!("{}.yaml", safe_stem);
    let target_path = target_dir.join(file_name);

    let mut manual_tpl = template.clone();
    manual_tpl.kind = super::TemplateKind::Manual;
    manual_tpl.source_path = Some(target_path.display().to_string());

    let yaml = manual_tpl
        .to_yaml()
        .with_context(|| "Failed to serialize template to YAML")?;

    fs::write(&target_path, yaml)
        .with_context(|| format!("Failed to write template file to {}", target_path.display()))?;

    Ok(target_path)
}
