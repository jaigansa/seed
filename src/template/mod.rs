pub mod builtin;
pub mod loader;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TemplateKind {
    #[default]
    Preset,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateFile {
    pub path: String,
    pub content: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub executable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub description: String,
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub kind: TemplateKind,
    #[serde(default)]
    pub source_path: Option<String>,
    #[serde(default)]
    pub variables: HashMap<String, String>,
    #[serde(default)]
    pub directories: Vec<String>,
    #[serde(default)]
    pub files: Vec<TemplateFile>,
    #[serde(default)]
    pub outputs: Vec<String>,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

impl Template {
    #[cfg(test)]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            version: default_version(),
            kind: TemplateKind::Preset,
            source_path: None,
            variables: HashMap::new(),
            directories: Vec::new(),
            files: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn new_manual(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            version: default_version(),
            kind: TemplateKind::Manual,
            source_path: None,
            variables: HashMap::new(),
            directories: Vec::new(),
            files: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn is_preset(&self) -> bool {
        self.kind == TemplateKind::Preset
    }

    pub fn is_manual(&self) -> bool {
        self.kind == TemplateKind::Manual
    }

    #[cfg(test)]
    pub fn with_source_path(mut self, path: impl Into<String>) -> Self {
        self.source_path = Some(path.into());
        self
    }

    #[cfg(test)]
    pub fn with_directory(mut self, dir: impl Into<String>) -> Self {
        self.directories.push(dir.into());
        self
    }

    #[cfg(test)]
    pub fn with_file(
        mut self,
        path: impl Into<String>,
        content: impl Into<String>,
        required: bool,
    ) -> Self {
        self.files.push(TemplateFile {
            path: path.into(),
            content: content.into(),
            required,
            executable: false,
        });
        self
    }

    /// Render template with the given variable substitutions
    pub fn render(&self, vars: &HashMap<String, String>) -> Template {
        let mut rendered = self.clone();

        // Render directories
        rendered.directories = self
            .directories
            .iter()
            .map(|dir| render_str(dir, vars))
            .collect();

        // Render files
        rendered.files = self
            .files
            .iter()
            .map(|f| TemplateFile {
                path: render_str(&f.path, vars),
                content: render_str(&f.content, vars),
                required: f.required,
                executable: f.executable,
            })
            .collect();

        // Render outputs
        rendered.outputs = self
            .outputs
            .iter()
            .map(|out| render_str(out, vars))
            .collect();

        rendered
    }

    pub fn to_yaml(&self) -> Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }

    pub fn from_yaml(s: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(s)?)
    }

    pub fn to_toml(&self) -> Result<String> {
        Ok(toml::to_string_pretty(self)?)
    }

    pub fn from_toml(s: &str) -> Result<Self> {
        Ok(toml::from_str(s)?)
    }
}

pub fn render_str(template: &str, vars: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (k, v) in vars {
        let needle1 = format!("{{{{{}}}}}", k);
        let needle2 = format!("{{{{ {} }}}}", k);
        result = result.replace(&needle1, v);
        result = result.replace(&needle2, v);
    }
    result
}

/// Sorts templates such that templates whose names appear in `favorites`
/// are moved to the top of the list in the specified order, followed by all remaining templates.
pub fn sort_templates_with_favorites(templates: Vec<Template>, favorites: &[String]) -> Vec<Template> {
    if favorites.is_empty() {
        return templates;
    }

    let mut fav_list = Vec::new();
    let mut other_list = Vec::new();

    for tpl in templates {
        if favorites.iter().any(|f| f == &tpl.name) {
            fav_list.push(tpl);
        } else {
            other_list.push(tpl);
        }
    }

    fav_list.sort_by_key(|t| {
        favorites.iter().position(|f| f == &t.name).unwrap_or(usize::MAX)
    });

    fav_list.extend(other_list);
    fav_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_templates_with_favorites() {
        let t1 = Template::new("rust", "Rust");
        let t2 = Template::new("python", "Python");
        let t3 = Template::new("react", "React");
        let t4 = Template::new("go", "Go");

        let list = vec![t1, t2, t3, t4];
        let favs = vec!["react".to_string(), "rust".to_string()];

        let sorted = sort_templates_with_favorites(list, &favs);
        let names: Vec<_> = sorted.iter().map(|t| t.name.as_str()).collect();
        assert_eq!(names, vec!["react", "rust", "python", "go"]);
    }

    #[test]
    fn test_render_variables() {
        let mut vars = HashMap::new();
        vars.insert("project_name".to_string(), "my-cool-app".to_string());
        vars.insert("author".to_string(), "Jane Doe".to_string());

        let tpl = Template::new("test", "test desc")
            .with_directory("src/{{project_name}}")
            .with_file(
                "src/{{project_name}}/main.py",
                "# Author: {{author}}\nprint('Hello from {{project_name}}')",
                true,
            );

        let rendered = tpl.render(&vars);
        assert_eq!(rendered.directories[0], "src/my-cool-app");
        assert_eq!(rendered.files[0].path, "src/my-cool-app/main.py");
        assert!(rendered.files[0].content.contains("Jane Doe"));
        assert!(rendered.files[0].content.contains("my-cool-app"));
    }

    #[test]
    fn test_preset_and_manual_template_kind() {
        let preset = Template::new("rust", "Rust preset");
        assert!(preset.is_preset());
        assert!(!preset.is_manual());
        assert_eq!(preset.kind, TemplateKind::Preset);

        let manual = Template::new_manual("my-workflow", "Custom workflow")
            .with_source_path("~/.config/seed/templates/my-workflow.yaml");
        assert!(!manual.is_preset());
        assert!(manual.is_manual());
        assert_eq!(manual.kind, TemplateKind::Manual);
        assert_eq!(
            manual.source_path.as_deref(),
            Some("~/.config/seed/templates/my-workflow.yaml")
        );

        // Serialization roundtrip
        let yaml = manual.to_yaml().unwrap();
        let deserialized = Template::from_yaml(&yaml).unwrap();
        assert!(deserialized.is_manual());
    }
}
