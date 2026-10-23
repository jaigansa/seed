pub mod ui;

use crate::capture::{capture_directory_as_template, CaptureOptions};
use crate::config::{load_config, save_config, AppConfig};
use crate::creator::{create_project, CreateOptions};
use crate::maintainer::{audit_project, AuditReport};
use crate::template::{
    builtin::get_builtin_templates,
    loader::{self, get_local_templates_dir, get_user_templates_dir, load_template_from_file},
    Template, TemplateFile,
};
use crate::tree::{get_tree_lines, TreeOptions};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectCategory {
    Personal,
    Client(String),
    Resource,
    Archive(String),
}

#[derive(Debug, Clone)]
pub struct ProjectItem {
    pub name: String,
    pub category: ProjectCategory,
    pub path: PathBuf,
    pub file_count: usize,
    pub dir_count: usize,
    pub total_bytes: u64,
    pub detected_type: String,
    pub preview_files: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MarkdownModalMode {
    Preview,
    Edit,
}

#[derive(Debug, Clone)]
pub struct MarkdownEditorModal {
    pub project_name: String,
    pub file_path: PathBuf,
    pub lines: Vec<String>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub scroll_offset: usize,
    pub is_dirty: bool,
    pub mode: MarkdownModalMode,
}

#[derive(Debug, Clone)]
pub struct ManualTemplateForm {
    pub name: String,
    pub description: String,
    pub directories: String,
    pub files: String,
    pub outputs: String,
    pub save_to_workspace: bool,
    pub active_field: usize, // 0..=7
}

impl Default for ManualTemplateForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: "Custom manual template".to_string(),
            directories: "src, docs, tests".to_string(),
            files: "README.md, .gitignore".to_string(),
            outputs: "Deliverable".to_string(),
            save_to_workspace: false,
            active_field: 0,
        }
    }
}

pub struct App {
    pub current_tab: usize,
    pub config: AppConfig,
    pub templates: Vec<Template>,

    // Tab 0: Create New Project
    pub create_pane: usize, // 0 = Template List, 1 = Form
    pub create_template_idx: usize,
    pub create_template_filter_idx: usize, // 0 = All, 1 = Preset, 2 = Manual
    pub create_category_idx: usize, // 0 = Personal, 1 = Client, 2 = Resource
    pub create_client_name: String,
    pub create_field_idx: usize,
    pub create_project_name: String,
    pub create_custom_path: String,
    pub create_author: String,
    pub create_license_idx: usize,
    pub create_init_git: bool,

    // Tab 1: View Projects
    pub projects_list: Vec<ProjectItem>,
    pub selected_project_idx: usize,
    pub projects_filter_idx: usize, // 0 = All, 1 = Personal, 2 = Clients, 3 = Resources, 4 = Archive
    pub search_query: String,
    pub is_searching: bool,

    // Robust Modals
    pub show_help_modal: bool,
    pub confirm_archive: Option<(String, PathBuf)>,
    pub tree_popup: Option<(String, Vec<String>, usize)>, // (name, lines, scroll_offset)
    pub doctor_popup: Option<(String, AuditReport)>,
    pub manual_template_modal: Option<ManualTemplateForm>,
    pub md_editor_modal: Option<MarkdownEditorModal>,

    // Tab 2: Templates Section
    pub templates_view_idx: usize,
    pub templates_filter_idx: usize, // 0 = All, 1 = Preset, 2 = Manual

    // Tab 3: Config & Project Paths
    pub config_field_idx: usize,

    // Notifications
    pub status_message: Option<String>,
    pub popup_message: Option<String>,
}

impl App {
    pub fn new(initial_path: PathBuf) -> Result<Self> {
        let config = load_config();
        let templates = load_all_templates(&config);

        let initial_name = if initial_path == *"." || initial_path.as_os_str().is_empty() {
            "my-project".to_string()
        } else {
            initial_path
                .file_name()
                .and_then(|n| n.to_str())
                .filter(|n| *n != "." && !n.is_empty())
                .unwrap_or("my-project")
                .to_string()
        };

        let mut app = Self {
            current_tab: 0,
            create_pane: 0,
            create_template_idx: 0,
            create_template_filter_idx: 0,
            create_category_idx: 0,
            create_client_name: "AcmeClient".to_string(),
            create_field_idx: 0,
            create_project_name: initial_name,
            create_custom_path: "".to_string(),
            create_author: config.default_author.clone(),
            create_license_idx: 0,
            create_init_git: true,

            projects_list: Vec::new(),
            selected_project_idx: 0,
            projects_filter_idx: 0,
            search_query: String::new(),
            is_searching: false,

            show_help_modal: false,
            confirm_archive: None,
            tree_popup: None,
            doctor_popup: None,
            manual_template_modal: None,
            md_editor_modal: None,

            templates_view_idx: 0,
            templates_filter_idx: 0,
            config_field_idx: 0,

            config,
            templates,

            status_message: Some("Ready.".to_string()),
            popup_message: None,
        };

        app.update_destination_path();
        app.reload_projects();

        if let Some(pos) = app.templates.iter().position(|t| t.name == app.config.default_template) {
            app.create_template_idx = pos;
        }

        Ok(app)
    }

    pub fn update_destination_path(&mut self) {
        let base = match self.create_category_idx {
            1 => {
                let c = if self.create_client_name.is_empty() { "Client" } else { &self.create_client_name };
                self.config.clients_path().join(c)
            }
            2 => self.config.resources_path(),
            _ => self.config.personal_path(),
        };
        self.create_custom_path = base.join(&self.create_project_name).to_string_lossy().to_string();
    }

    pub fn reload_projects(&mut self) {
        self.projects_list = scan_workspace_projects(&self.config);
        let count = self.filtered_projects().len();
        if count == 0 {
            self.selected_project_idx = 0;
        } else if self.selected_project_idx >= count {
            self.selected_project_idx = count - 1;
        }
    }

    pub fn filtered_projects(&self) -> Vec<&ProjectItem> {
        let q = self.search_query.trim().to_lowercase();
        self.projects_list
            .iter()
            .filter(|p| {
                let cat_match = match self.projects_filter_idx {
                    1 => matches!(p.category, ProjectCategory::Personal),
                    2 => matches!(p.category, ProjectCategory::Client(_)),
                    3 => matches!(p.category, ProjectCategory::Resource),
                    4 => matches!(p.category, ProjectCategory::Archive(_)),
                    _ => true,
                };
                if !cat_match {
                    return false;
                }
                if q.is_empty() {
                    return true;
                }
                p.name.to_lowercase().contains(&q)
                    || p.detected_type.to_lowercase().contains(&q)
                    || match &p.category {
                        ProjectCategory::Client(c) => c.to_lowercase().contains(&q),
                        ProjectCategory::Archive(y) => y.to_lowercase().contains(&q),
                        _ => false,
                    }
            })
            .collect()
    }

    pub fn reload_templates(&mut self) {
        self.templates = load_all_templates(&self.config);
    }

    pub fn filtered_templates_for_create(&self) -> Vec<(usize, &Template)> {
        self.templates
            .iter()
            .enumerate()
            .filter(|(_, t)| match self.create_template_filter_idx {
                1 => t.is_preset(),
                2 => t.is_manual(),
                _ => true,
            })
            .collect()
    }

    pub fn filtered_templates_for_view(&self) -> Vec<(usize, &Template)> {
        self.templates
            .iter()
            .enumerate()
            .filter(|(_, t)| match self.templates_filter_idx {
                1 => t.is_preset(),
                2 => t.is_manual(),
                _ => true,
            })
            .collect()
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        // -1. Interactive Help / Shortcut Cheat Sheet Modal
        if self.show_help_modal {
            match key.code {
                KeyCode::Esc
                | KeyCode::Enter
                | KeyCode::Char('?')
                | KeyCode::Char('h')
                | KeyCode::Char('q')
                | KeyCode::Char(' ')
                | KeyCode::F(1) => {
                    self.show_help_modal = false;
                }
                _ => {}
            }
            return false;
        }

        // 0. Manual Template Creator Modal
        if self.manual_template_modal.is_some() {
            return self.handle_manual_template_keys(key);
        }

        // 0.5. In-TUI Markdown Editor / Viewer Modal
        if self.md_editor_modal.is_some() {
            return self.handle_md_editor_keys(key);
        }

        // 1. Confirm Archive Modal (Protects against accidental archiving)
        if let Some((_, path)) = self.confirm_archive.clone() {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                    self.confirm_archive = None;
                    match self.config.archive_project(&path) {
                        Ok(dest) => {
                            self.popup_message = Some(format!(
                                "[OK] Project Successfully Archived!\n\nMoved: {}\nTo:    {}",
                                path.display(),
                                dest.display()
                            ));
                            self.reload_projects();
                        }
                        Err(e) => {
                            self.popup_message = Some(format!("[ERROR] Failed to archive project:\n{}", e));
                        }
                    }
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.confirm_archive = None;
                }
                _ => {}
            }
            return false;
        }

        // 2. Doctor Health Gauge Modal
        if self.doctor_popup.is_some() {
            if key.code == KeyCode::Esc || key.code == KeyCode::Enter || key.code == KeyCode::Char(' ') || key.code == KeyCode::Char('d') {
                self.doctor_popup = None;
            }
            return false;
        }

        // 3. Tree Viewer Modal
        if let Some((_, lines, scroll)) = self.tree_popup.as_mut() {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    if *scroll > 0 {
                        *scroll -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if *scroll + 1 < lines.len() {
                        *scroll += 1;
                    }
                }
                KeyCode::PageUp => {
                    *scroll = scroll.saturating_sub(10);
                }
                KeyCode::PageDown => {
                    *scroll = (*scroll + 10).min(lines.len().saturating_sub(1));
                }
                KeyCode::Esc | KeyCode::Enter | KeyCode::Char('t') | KeyCode::Char('q') => {
                    self.tree_popup = None;
                }
                _ => {}
            }
            return false;
        }

        // 4. Standard Message Popup
        if self.popup_message.is_some() {
            if key.code == KeyCode::Esc || key.code == KeyCode::Enter || key.code == KeyCode::Char(' ') {
                self.popup_message = None;
            }
            return false;
        }

        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return true;
        }

        // If on Tab 1 (New Project), Tab and BackTab navigate containers and fields without jumping tabs
        if self.current_tab == 1 {
            if key.code == KeyCode::Tab {
                if self.create_pane == 0 {
                    self.create_pane = 1;
                    self.create_field_idx = 0;
                } else {
                    let is_client = self.create_category_idx == 1;
                    let max_fields = if is_client { 6 } else { 5 };
                    if self.create_field_idx < max_fields {
                        self.create_field_idx += 1;
                    } else {
                        self.create_pane = 0;
                    }
                }
                return false;
            } else if key.code == KeyCode::BackTab {
                if self.create_pane == 1 {
                    if self.create_field_idx > 0 {
                        self.create_field_idx -= 1;
                    } else {
                        self.create_pane = 0;
                    }
                } else {
                    self.create_pane = 1;
                    let is_client = self.create_category_idx == 1;
                    self.create_field_idx = if is_client { 6 } else { 5 };
                }
                return false;
            }
        } else if self.current_tab == 3 {
            if key.code == KeyCode::Tab {
                if self.config_field_idx < 6 {
                    self.config_field_idx += 1;
                } else {
                    self.config_field_idx = 0;
                }
                return false;
            } else if key.code == KeyCode::BackTab {
                if self.config_field_idx > 0 {
                    self.config_field_idx -= 1;
                } else {
                    self.config_field_idx = 6;
                }
                return false;
            }
        } else if !self.is_editing_text() {
            match key.code {
                KeyCode::Tab => {
                    self.current_tab = (self.current_tab + 1) % 4;
                    if self.current_tab == 0 {
                        self.reload_projects();
                    }
                    return false;
                }
                KeyCode::BackTab => {
                    self.current_tab = if self.current_tab == 0 { 3 } else { self.current_tab - 1 };
                    if self.current_tab == 0 {
                        self.reload_projects();
                    }
                    return false;
                }
                _ => {}
            }
        }

        // Global help modal toggle when not typing text
        if !self.is_editing_text() && (key.code == KeyCode::Char('?') || key.code == KeyCode::F(1)) {
            self.show_help_modal = true;
            return false;
        }

        // Major tab switching via '1'..='4' when not typing text
        if !self.is_editing_text() {
            match key.code {
                KeyCode::Char('1') => {
                    self.current_tab = 0;
                    self.reload_projects();
                    return false;
                }
                KeyCode::Char('2') => {
                    self.current_tab = 1;
                    return false;
                }
                KeyCode::Char('3') => {
                    self.current_tab = 2;
                    return false;
                }
                KeyCode::Char('4') => {
                    self.current_tab = 3;
                    return false;
                }
                _ => {}
            }
        }

        match self.current_tab {
            0 => self.handle_view_projects_keys(key),
            1 => self.handle_create_keys(key),
            2 => self.handle_templates_keys(key),
            3 => self.handle_config_keys(key),
            _ => false,
        }
    }

    pub fn is_editing_text(&self) -> bool {
        if let Some(ref m) = self.md_editor_modal {
            return m.mode == MarkdownModalMode::Edit;
        }
        match self.current_tab {
            0 => self.is_searching,
            1 => {
                if self.create_pane == 0 {
                    return false;
                }
                let is_client = self.create_category_idx == 1;
                if is_client {
                    self.create_field_idx >= 1 && self.create_field_idx <= 3
                } else {
                    self.create_field_idx >= 1 && self.create_field_idx <= 2
                }
            }
            3 => self.config_field_idx == 0 || self.config_field_idx == 1 || self.config_field_idx == 3,
            _ => false,
        }
    }

    // ==========================================
    // TAB 0: CREATE NEW PROJECT HANDLERS
    // ==========================================
    fn handle_create_keys(&mut self, key: KeyEvent) -> bool {
        let is_client = self.create_category_idx == 1;
        let max_fields = if is_client { 6 } else { 5 };
        if self.create_field_idx > max_fields {
            self.create_field_idx = max_fields;
        }
        let is_text_field = if is_client {
            self.create_pane == 1 && self.create_field_idx >= 1 && self.create_field_idx <= 3
        } else {
            self.create_pane == 1 && self.create_field_idx >= 1 && self.create_field_idx <= 2
        };

        // 1. Template List Container (Pane 0)
        if self.create_pane == 0 {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => return true,
                KeyCode::Char('f') | KeyCode::Char('F') => {
                    if let Some(current_tpl) = self.templates.get(self.create_template_idx).cloned() {
                        let now_fav = self.config.toggle_favorite(&current_tpl.name);
                        let _ = save_config(&self.config);
                        self.reload_templates();
                        if let Some(new_pos) = self.templates.iter().position(|t| t.name == current_tpl.name) {
                            self.create_template_idx = new_pos;
                        }
                        if now_fav {
                            self.status_message = Some(format!("Pinned '{}' to top favorites.", current_tpl.name));
                        } else {
                            self.status_message = Some(format!("Removed '{}' from favorites.", current_tpl.name));
                        }
                    }
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    self.create_template_filter_idx = if self.create_template_filter_idx == 0 {
                        2
                    } else {
                        self.create_template_filter_idx - 1
                    };
                    let filtered = self.filtered_templates_for_create();
                    if !filtered.is_empty() && !filtered.iter().any(|&(idx, _)| idx == self.create_template_idx) {
                        self.create_template_idx = filtered[0].0;
                    }
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    self.create_template_filter_idx = (self.create_template_filter_idx + 1) % 3;
                    let filtered = self.filtered_templates_for_create();
                    if !filtered.is_empty() && !filtered.iter().any(|&(idx, _)| idx == self.create_template_idx) {
                        self.create_template_idx = filtered[0].0;
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    let filtered = self.filtered_templates_for_create();
                    if let Some(pos) = filtered.iter().position(|&(idx, _)| idx == self.create_template_idx) {
                        if pos > 0 {
                            self.create_template_idx = filtered[pos - 1].0;
                        }
                    } else if !filtered.is_empty() {
                        self.create_template_idx = filtered[0].0;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let filtered = self.filtered_templates_for_create();
                    if let Some(pos) = filtered.iter().position(|&(idx, _)| idx == self.create_template_idx) {
                        if pos + 1 < filtered.len() {
                            self.create_template_idx = filtered[pos + 1].0;
                        }
                    } else if !filtered.is_empty() {
                        self.create_template_idx = filtered[0].0;
                    }
                }
                KeyCode::Enter | KeyCode::Tab => {
                    self.create_pane = 1;
                    self.create_field_idx = 0;
                }
                _ => {}
            }
            return false;
        }

        // 2. Project Details Form Container (Pane 1)
        match key.code {
            KeyCode::Char('q') if !is_text_field => {
                return true;
            }
            KeyCode::Char('f') | KeyCode::Char('F') if !is_text_field => {
                if let Some(current_tpl) = self.templates.get(self.create_template_idx).cloned() {
                    let now_fav = self.config.toggle_favorite(&current_tpl.name);
                    let _ = save_config(&self.config);
                    self.reload_templates();
                    if let Some(new_pos) = self.templates.iter().position(|t| t.name == current_tpl.name) {
                        self.create_template_idx = new_pos;
                    }
                    if now_fav {
                        self.status_message = Some(format!("Pinned '{}' to top favorites.", current_tpl.name));
                    } else {
                        self.status_message = Some(format!("Removed '{}' from favorites.", current_tpl.name));
                    }
                }
            }
            KeyCode::Left if !is_text_field => {
                self.create_pane = 0;
            }
            KeyCode::Up => {
                if self.create_field_idx > 0 {
                    self.create_field_idx -= 1;
                } else {
                    self.create_pane = 0;
                }
            }
            KeyCode::Down => {
                if self.create_field_idx < max_fields {
                    self.create_field_idx += 1;
                }
            }
            KeyCode::Char(' ') => {
                let lic_field = if is_client { 4 } else { 3 };
                let git_field = if is_client { 5 } else { 4 };
                let btn_field = if is_client { 6 } else { 5 };

                if self.create_field_idx == 0 {
                    self.create_category_idx = (self.create_category_idx + 1) % 3;
                    self.update_destination_path();
                    let max_f = if self.create_category_idx == 1 { 6 } else { 5 };
                    if self.create_field_idx > max_f {
                        self.create_field_idx = max_f;
                    }
                } else if self.create_field_idx == lic_field {
                    self.create_license_idx = (self.create_license_idx + 1) % 5;
                } else if self.create_field_idx == git_field {
                    self.create_init_git = !self.create_init_git;
                } else if self.create_field_idx == btn_field {
                    self.execute_create_project();
                } else if is_text_field {
                    self.append_char_create(' ');
                    self.update_destination_path();
                }
            }
            KeyCode::Backspace => {
                if is_text_field {
                    self.backspace_char_create(is_client);
                    self.update_destination_path();
                }
            }
            KeyCode::Char(c) => {
                if is_text_field {
                    self.append_char_create(c);
                    self.update_destination_path();
                }
            }
            KeyCode::Enter => {
                let btn_field = if is_client { 6 } else { 5 };
                if self.create_field_idx == btn_field {
                    self.execute_create_project();
                } else if self.create_field_idx < max_fields {
                    self.create_field_idx += 1;
                }
            }
            _ => {}
        }
        false
    }

    fn append_char_create(&mut self, c: char) {
        let is_client = self.create_category_idx == 1;
        if is_client {
            match self.create_field_idx {
                1 => self.create_project_name.push(c),
                2 => self.create_client_name.push(c),
                3 => self.create_author.push(c),
                _ => {}
            }
        } else {
            match self.create_field_idx {
                1 => self.create_project_name.push(c),
                2 => self.create_author.push(c),
                _ => {}
            }
        }
    }

    fn backspace_char_create(&mut self, is_client: bool) {
        if is_client {
            match self.create_field_idx {
                1 => { self.create_project_name.pop(); }
                2 => { self.create_client_name.pop(); }
                3 => { self.create_author.pop(); }
                _ => {}
            }
        } else {
            match self.create_field_idx {
                1 => { self.create_project_name.pop(); }
                2 => { self.create_author.pop(); }
                _ => {}
            }
        }
    }

    fn execute_create_project(&mut self) {
        let clean_name = self.create_project_name.trim();
        if clean_name.is_empty() {
            self.popup_message = Some("[ERROR] Project name cannot be empty. Please enter a valid name.".to_string());
            return;
        }

        if self.create_category_idx == 1 && self.create_client_name.trim().is_empty() {
            self.popup_message = Some("[ERROR] Client name cannot be empty for client projects.".to_string());
            return;
        }

        let filtered = self.filtered_templates_for_create();
        if filtered.is_empty() {
            self.popup_message = Some("[ERROR] No templates available in the active category filter.\nPress [←/→] to select All or Preset templates.".to_string());
            return;
        }

        let tpl = if let Some(&(_, t)) = filtered.iter().find(|&&(idx, _)| idx == self.create_template_idx) {
            t.clone()
        } else if let Some(&(_, t)) = filtered.first() {
            t.clone()
        } else if let Some(t) = self.templates.get(self.create_template_idx).cloned() {
            t
        } else {
            self.popup_message = Some("[ERROR] Please select a valid template before creating project.".to_string());
            return;
        };

        let licenses = ["MIT", "Apache-2.0", "BSD-3-Clause", "Unlicense", "None"];
        let license = licenses[self.create_license_idx % licenses.len()].to_string();

        let target_dir = PathBuf::from(&self.create_custom_path);

        let mut extra_vars: HashMap<String, String> = HashMap::new();
        if self.create_category_idx == 1 {
            extra_vars.insert("client_name".to_string(), self.create_client_name.clone());
        }

        let opts = CreateOptions {
            target_dir: target_dir.clone(),
            project_name: self.create_project_name.clone(),
            author: self.create_author.clone(),
            license,
            init_git: self.create_init_git,
            force: false,
            extra_vars,
            quiet: true, // Prevents stdout pollution and text overlays
        };

        match create_project(&tpl, &opts) {
            Ok(summary) => {
                let cat_label = match self.create_category_idx {
                    1 => format!("Client Work ({})", self.create_client_name),
                    2 => "Resource / Asset Library".to_string(),
                    _ => "Personal Project".to_string(),
                };
                let created_name = self.create_project_name.clone();
                self.popup_message = Some(format!(
                    "[OK] Project '{}' Created Successfully!\n\nCategory: {}\nPath: {}\nCreated {} directories, {} files\nTemplate: {}",
                    created_name,
                    cat_label,
                    target_dir.display(),
                    summary.created_dirs,
                    summary.created_files,
                    tpl.name
                ));
                self.reload_projects();
                self.projects_filter_idx = 0; // Show All in projects view
                self.search_query.clear();    // Clear search so new project is visible
                self.is_searching = false;
                self.current_tab = 0;
                if let Some(pos) = self.filtered_projects().iter().position(|p| p.name == created_name) {
                    self.selected_project_idx = pos;
                }
                self.create_project_name = "my-project".to_string();
                self.create_pane = 0;
                self.create_field_idx = 0;
                self.update_destination_path();
            }
            Err(e) => {
                self.popup_message = Some(format!("[ERROR] Failed to create project:\n{}", e));
            }
        }
    }

    // ==========================================
    // TAB 1: VIEW PROJECTS HANDLERS
    // ==========================================
    fn handle_view_projects_keys(&mut self, key: KeyEvent) -> bool {
        // Search Mode Handling
        if self.is_searching {
            match key.code {
                KeyCode::Esc => {
                    self.is_searching = false;
                    self.search_query.clear();
                    self.selected_project_idx = 0;
                }
                KeyCode::Enter => {
                    self.is_searching = false;
                }
                KeyCode::Backspace => {
                    self.search_query.pop();
                    self.selected_project_idx = 0;
                }
                KeyCode::Up => {
                    if self.selected_project_idx > 0 {
                        self.selected_project_idx -= 1;
                    }
                }
                KeyCode::Down => {
                    let count = self.filtered_projects().len();
                    if count > 0 && self.selected_project_idx < count - 1 {
                        self.selected_project_idx += 1;
                    }
                }
                KeyCode::Char(c) => {
                    self.search_query.push(c);
                    self.selected_project_idx = 0;
                }
                _ => {}
            }
            return false;
        }

        let count = self.filtered_projects().len();

        match key.code {
            KeyCode::Char('q') => {
                return true;
            }
            KeyCode::Char('/') => {
                self.is_searching = true;
                self.status_message = Some("Search mode active. Type name/stack, Esc to exit.".into());
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_project_idx > 0 {
                    self.selected_project_idx -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if count > 0 && self.selected_project_idx < count - 1 {
                    self.selected_project_idx += 1;
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                self.current_tab = 1;
                self.create_pane = 0;
                return false;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.projects_filter_idx = if self.projects_filter_idx == 0 {
                    4
                } else {
                    self.projects_filter_idx - 1
                };
                self.selected_project_idx = 0;
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.projects_filter_idx = (self.projects_filter_idx + 1) % 5;
                self.selected_project_idx = 0;
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.reload_projects();
                self.status_message = Some("Projects list refreshed from workspace.".into());
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                // Interactive In-TUI Markdown Editor / Viewer for project README
                self.open_markdown_editor_for_project();
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                // Doctor check on selected project (opens visual Gauge modal)
                let target = self
                    .filtered_projects()
                    .get(self.selected_project_idx)
                    .map(|p| (p.name.clone(), p.path.clone()));

                if let Some((name, path)) = target {
                    match audit_project(&path, None) {
                        Ok(report) => {
                            self.doctor_popup = Some((name, report));
                        }
                        Err(e) => {
                            self.popup_message = Some(format!("[ERROR] Health check failed: {}", e));
                        }
                    }
                }
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                // Interactive Directory Tree viewer modal
                let target = self
                    .filtered_projects()
                    .get(self.selected_project_idx)
                    .map(|p| (p.name.clone(), p.path.clone()));

                if let Some((name, path)) = target {
                    let mut opts = TreeOptions::default();
                    opts.use_nerd_font = self.config.use_nerd_font();
                    let (lines, _) = get_tree_lines(&path, &opts);
                    self.tree_popup = Some((name, lines, 0));
                }
            }
            KeyCode::Char('o') | KeyCode::Char('O') | KeyCode::Enter => {
                // Open project in desktop file explorer / default viewer
                let target = self
                    .filtered_projects()
                    .get(self.selected_project_idx)
                    .map(|p| (p.name.clone(), p.path.clone(), p.category.clone()));

                if let Some((name, path, category)) = target {
                    match open_project_folder(&path) {
                        Ok(_) => {
                            self.status_message = Some(format!("Opened folder: {}", path.display()));
                            self.popup_message = Some(format!(
                                "[OK] Project Folder Opened\n\nProject: {}\nPath:    {}\nCategory: {:?}",
                                name,
                                path.display(),
                                category
                            ));
                        }
                        Err(e) => {
                            self.popup_message = Some(format!("[ERROR] Failed to open folder: {}", e));
                        }
                    }
                }
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                // Confirmation dialog before archiving (prevents accidental data relocation)
                let target = self
                    .filtered_projects()
                    .get(self.selected_project_idx)
                    .map(|p| (p.name.clone(), p.path.clone(), p.category.clone()));

                if let Some((name, path, category)) = target {
                    if matches!(category, ProjectCategory::Archive(_)) {
                        self.popup_message = Some(format!("Project '{}' is already in the archive.", name));
                    } else {
                        self.confirm_archive = Some((name, path));
                    }
                }
            }
            _ => {}
        }
        false
    }

    // ==========================================
    // TAB 2: TEMPLATES SECTION HANDLERS
    // ==========================================
    fn handle_templates_keys(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') => {
                return true;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.templates_filter_idx = if self.templates_filter_idx == 0 {
                    2
                } else {
                    self.templates_filter_idx - 1
                };
                self.templates_view_idx = 0;
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.templates_filter_idx = (self.templates_filter_idx + 1) % 3;
                self.templates_view_idx = 0;
            }
            KeyCode::Char('m') | KeyCode::Char('M') | KeyCode::Char('n') | KeyCode::Char('N') => {
                self.manual_template_modal = Some(ManualTemplateForm::default());
            }
            KeyCode::Char('f') | KeyCode::Char('F') => {
                let filtered = self.filtered_templates_for_view();
                if let Some(&(_, current_tpl)) = filtered.get(self.templates_view_idx) {
                    let name = current_tpl.name.clone();
                    let now_fav = self.config.toggle_favorite(&name);
                    let _ = save_config(&self.config);
                    self.reload_templates();
                    let new_filtered = self.filtered_templates_for_view();
                    if let Some(new_pos) = new_filtered.iter().position(|(_, t)| t.name == name) {
                        self.templates_view_idx = new_pos;
                    }
                    if now_fav {
                        self.status_message = Some(format!("Pinned '{}' to top favorites.", name));
                    } else {
                        self.status_message = Some(format!("Removed '{}' from favorites.", name));
                    }
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.templates_view_idx > 0 {
                    self.templates_view_idx -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let count = self.filtered_templates_for_view().len();
                if count > 0 && self.templates_view_idx < count - 1 {
                    self.templates_view_idx += 1;
                }
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                let out_dir = PathBuf::from(&self.config.custom_templates_dir);
                let _ = fs::create_dir_all(&out_dir);
                let out_file = out_dir.join("captured-template.yaml");

                let opts = CaptureOptions {
                    name: "captured-template".to_string(),
                    description: "Template captured from working directory".to_string(),
                    output_path: out_file.clone(),
                    replace_name_with_var: true,
                    max_file_size_kb: 128,
                };

                match capture_directory_as_template(&PathBuf::from("."), &opts) {
                    Ok(_) => {
                        self.reload_templates();
                        self.popup_message = Some(format!(
                            "[OK] Current directory captured as template!\nSaved to: {}",
                            out_file.display()
                        ));
                    }
                    Err(e) => {
                        self.popup_message = Some(format!("[ERROR] Capture failed:\n{}", e));
                    }
                }
            }
            _ => {}
        }
        false
    }

    fn handle_manual_template_keys(&mut self, key: KeyEvent) -> bool {
        let Some(ref mut form) = self.manual_template_modal else {
            return false;
        };

        if key.code == KeyCode::Esc {
            self.manual_template_modal = None;
            return false;
        }

        if key.modifiers.contains(KeyModifiers::CONTROL) && (key.code == KeyCode::Char('s') || key.code == KeyCode::Char('S')) {
            self.execute_save_manual_template();
            return false;
        }

        match key.code {
            KeyCode::Tab | KeyCode::Down => {
                form.active_field = (form.active_field + 1) % 8;
            }
            KeyCode::BackTab | KeyCode::Up => {
                form.active_field = if form.active_field == 0 { 7 } else { form.active_field - 1 };
            }
            KeyCode::Char(' ') if form.active_field == 5 => {
                form.save_to_workspace = !form.save_to_workspace;
            }
            KeyCode::Enter => {
                if form.active_field == 5 {
                    form.save_to_workspace = !form.save_to_workspace;
                } else if form.active_field == 6 {
                    self.execute_save_manual_template();
                } else if form.active_field == 7 {
                    self.manual_template_modal = None;
                } else {
                    form.active_field = (form.active_field + 1) % 8;
                }
            }
            KeyCode::Backspace => {
                match form.active_field {
                    0 => { form.name.pop(); }
                    1 => { form.description.pop(); }
                    2 => { form.directories.pop(); }
                    3 => { form.files.pop(); }
                    4 => { form.outputs.pop(); }
                    _ => {}
                }
            }
            KeyCode::Char(c) => {
                match form.active_field {
                    0 => { form.name.push(c); }
                    1 => { form.description.push(c); }
                    2 => { form.directories.push(c); }
                    3 => { form.files.push(c); }
                    4 => { form.outputs.push(c); }
                    _ => {}
                }
            }
            _ => {}
        }
        false
    }

    fn execute_save_manual_template(&mut self) {
        let Some(form) = self.manual_template_modal.clone() else {
            return;
        };

        let trimmed_name = form.name.trim();
        if trimmed_name.is_empty() {
            self.popup_message = Some("[ERROR] Template name cannot be empty!".to_string());
            return;
        }

        let desc = if form.description.trim().is_empty() {
            "Custom manual template".to_string()
        } else {
            form.description.trim().to_string()
        };

        let dirs: Vec<String> = form
            .directories
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let dirs = if dirs.is_empty() { vec!["src".to_string()] } else { dirs };

        let file_names: Vec<String> = form
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
                    format!("# {{{{project_name}}}}\n\n{}\n", desc)
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

        let outputs: Vec<String> = form
            .outputs
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let outputs = if outputs.is_empty() { vec!["Deliverable".to_string()] } else { outputs };

        let mut tpl = Template::new_manual(trimmed_name, desc);
        tpl.directories = dirs;
        tpl.files = files;
        tpl.outputs = outputs;

        let target_dir = if form.save_to_workspace {
            loader::get_local_templates_dir()
        } else if !self.config.custom_templates_dir.is_empty() {
            PathBuf::from(&self.config.custom_templates_dir)
        } else if let Some(user_dir) = loader::get_user_templates_dir() {
            user_dir
        } else {
            loader::get_local_templates_dir()
        };

        match loader::save_manual_template(&tpl, &target_dir) {
            Ok(saved_path) => {
                self.manual_template_modal = None;
                self.reload_templates();
                let saved_name = tpl.name.clone();
                let filtered = self.filtered_templates_for_view();
                if let Some(pos) = filtered.iter().position(|(_, t)| t.name == saved_name) {
                    self.templates_view_idx = pos;
                }
                self.popup_message = Some(format!(
                    "[OK] Manual Template '{}' Created Successfully!\n\nSaved to: {}\nType: Manual Template (Custom User Definition)",
                    saved_name,
                    saved_path.display()
                ));
            }
            Err(e) => {
                self.popup_message = Some(format!("[ERROR] Failed to save manual template:\n{}", e));
            }
        }
    }

    // ==========================================
    // IN-TUI MARKDOWN EDITOR & VIEWER
    // ==========================================
    pub fn open_markdown_editor_for_project(&mut self) {
        let target = self
            .filtered_projects()
            .get(self.selected_project_idx)
            .map(|p| (p.name.clone(), p.path.clone()));

        let Some((name, path)) = target else {
            return;
        };

        let mut readme_path = path.join("README.md");
        if !readme_path.exists() && path.join("readme.md").exists() {
            readme_path = path.join("readme.md");
        }

        if readme_path.exists() {
            let content = fs::read_to_string(&readme_path).unwrap_or_default();
            let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
            if lines.is_empty() {
                lines.push(String::new());
            }
            self.md_editor_modal = Some(MarkdownEditorModal {
                project_name: name,
                file_path: readme_path,
                lines,
                cursor_row: 0,
                cursor_col: 0,
                scroll_offset: 0,
                is_dirty: false,
                mode: MarkdownModalMode::Preview,
            });
            self.status_message = Some("Markdown Viewer opened. Press [Tab] to edit, [Esc] to close.".into());
        } else {
            let boilerplate = format!(
                "# {}\n\n> Project overview and documentation.\n\n## Getting Started\n- [ ] Install dependencies\n- [ ] Configure environment variables\n- [ ] Run application or tests\n\n## Architecture\nDescribe the system components and directory structure here.\n\n## Notes & Tasks\n- [x] Initialized with Seed\n- [ ] Complete implementation\n",
                name
            );
            let lines: Vec<String> = boilerplate.lines().map(|s| s.to_string()).collect();
            self.md_editor_modal = Some(MarkdownEditorModal {
                project_name: name,
                file_path: readme_path,
                lines,
                cursor_row: 0,
                cursor_col: 0,
                scroll_offset: 0,
                is_dirty: true,
                mode: MarkdownModalMode::Edit,
            });
            self.status_message = Some("New README.md initialized! Press [Ctrl+S] to save, [Tab] for preview.".into());
        }
    }

    fn execute_save_markdown(&mut self) {
        let Some(ref mut modal) = self.md_editor_modal else {
            return;
        };
        let content = modal.lines.join("\n");
        match fs::write(&modal.file_path, content) {
            Ok(_) => {
                modal.is_dirty = false;
                let fname = modal
                    .file_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                self.status_message = Some(format!("✓ Saved {} ({} lines)", fname, modal.lines.len()));
                self.reload_projects();
            }
            Err(e) => {
                self.popup_message = Some(format!("[ERROR] Failed to save markdown file:\n{}", e));
            }
        }
    }

    fn handle_md_editor_keys(&mut self, key: KeyEvent) -> bool {
        let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

        if is_ctrl && key.code == KeyCode::Char('c') {
            return true;
        }

        // Ctrl+S saves markdown buffer
        if is_ctrl && (key.code == KeyCode::Char('s') || key.code == KeyCode::Char('S')) {
            self.execute_save_markdown();
            return false;
        }

        let Some(ref mut modal) = self.md_editor_modal else {
            return false;
        };

        match modal.mode {
            MarkdownModalMode::Preview => {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                        self.md_editor_modal = None;
                    }
                    KeyCode::Tab | KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Char('i') | KeyCode::Char('I') | KeyCode::Enter => {
                        modal.mode = MarkdownModalMode::Edit;
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        modal.scroll_offset = modal.scroll_offset.saturating_sub(1);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if modal.scroll_offset + 1 < modal.lines.len() {
                            modal.scroll_offset += 1;
                        }
                    }
                    KeyCode::PageUp => {
                        modal.scroll_offset = modal.scroll_offset.saturating_sub(12);
                    }
                    KeyCode::PageDown => {
                        modal.scroll_offset = (modal.scroll_offset + 12).min(modal.lines.len().saturating_sub(1));
                    }
                    KeyCode::Home => {
                        modal.scroll_offset = 0;
                    }
                    KeyCode::End => {
                        modal.scroll_offset = modal.lines.len().saturating_sub(1);
                    }
                    _ => {}
                }
            }
            MarkdownModalMode::Edit => {
                if is_ctrl && (key.code == KeyCode::Char('t') || key.code == KeyCode::Char('T')) {
                    if modal.lines.is_empty() {
                        modal.lines.push(String::new());
                    }
                    insert_str_at(&mut modal.lines[modal.cursor_row], modal.cursor_col, "- [ ] ");
                    modal.cursor_col += 6;
                    modal.is_dirty = true;
                    return false;
                }

                if modal.lines.is_empty() {
                    modal.lines.push(String::new());
                }

                match key.code {
                    KeyCode::Tab => {
                        modal.mode = MarkdownModalMode::Preview;
                    }
                    KeyCode::Esc => {
                        modal.mode = MarkdownModalMode::Preview;
                    }
                    KeyCode::Up => {
                        if modal.cursor_row > 0 {
                            modal.cursor_row -= 1;
                            let line_len = modal.lines[modal.cursor_row].chars().count();
                            modal.cursor_col = modal.cursor_col.min(line_len);
                        }
                    }
                    KeyCode::Down => {
                        if modal.cursor_row + 1 < modal.lines.len() {
                            modal.cursor_row += 1;
                            let line_len = modal.lines[modal.cursor_row].chars().count();
                            modal.cursor_col = modal.cursor_col.min(line_len);
                        }
                    }
                    KeyCode::Left => {
                        if modal.cursor_col > 0 {
                            modal.cursor_col -= 1;
                        } else if modal.cursor_row > 0 {
                            modal.cursor_row -= 1;
                            modal.cursor_col = modal.lines[modal.cursor_row].chars().count();
                        }
                    }
                    KeyCode::Right => {
                        let cur_len = modal.lines[modal.cursor_row].chars().count();
                        if modal.cursor_col < cur_len {
                            modal.cursor_col += 1;
                        } else if modal.cursor_row + 1 < modal.lines.len() {
                            modal.cursor_row += 1;
                            modal.cursor_col = 0;
                        }
                    }
                    KeyCode::Home => {
                        modal.cursor_col = 0;
                    }
                    KeyCode::End => {
                        modal.cursor_col = modal.lines[modal.cursor_row].chars().count();
                    }
                    KeyCode::PageUp => {
                        modal.cursor_row = modal.cursor_row.saturating_sub(12);
                        let line_len = modal.lines[modal.cursor_row].chars().count();
                        modal.cursor_col = modal.cursor_col.min(line_len);
                    }
                    KeyCode::PageDown => {
                        modal.cursor_row = (modal.cursor_row + 12).min(modal.lines.len().saturating_sub(1));
                        let line_len = modal.lines[modal.cursor_row].chars().count();
                        modal.cursor_col = modal.cursor_col.min(line_len);
                    }
                    KeyCode::Backspace => {
                        if modal.cursor_col > 0 {
                            remove_char_at(&mut modal.lines[modal.cursor_row], modal.cursor_col - 1);
                            modal.cursor_col -= 1;
                            modal.is_dirty = true;
                        } else if modal.cursor_row > 0 {
                            let current_text = modal.lines.remove(modal.cursor_row);
                            modal.cursor_row -= 1;
                            modal.cursor_col = modal.lines[modal.cursor_row].chars().count();
                            modal.lines[modal.cursor_row].push_str(&current_text);
                            modal.is_dirty = true;
                        }
                    }
                    KeyCode::Delete => {
                        let cur_len = modal.lines[modal.cursor_row].chars().count();
                        if modal.cursor_col < cur_len {
                            remove_char_at(&mut modal.lines[modal.cursor_row], modal.cursor_col);
                            modal.is_dirty = true;
                        } else if modal.cursor_row + 1 < modal.lines.len() {
                            let next_text = modal.lines.remove(modal.cursor_row + 1);
                            modal.lines[modal.cursor_row].push_str(&next_text);
                            modal.is_dirty = true;
                        }
                    }
                    KeyCode::Enter => {
                        let cur_line = modal.lines[modal.cursor_row].clone();
                        let left: String = cur_line.chars().take(modal.cursor_col).collect();
                        let right: String = cur_line.chars().skip(modal.cursor_col).collect();

                        let mut prefix = String::new();
                        if left.starts_with("- [ ] ") {
                            if left.trim() == "- [ ]" {
                                modal.lines[modal.cursor_row] = right;
                                modal.cursor_col = 0;
                                modal.is_dirty = true;
                                return false;
                            }
                            prefix = "- [ ] ".to_string();
                        } else if left.starts_with("- [x] ") {
                            prefix = "- [ ] ".to_string();
                        } else if left.starts_with("- ") {
                            if left.trim() == "-" {
                                modal.lines[modal.cursor_row] = right;
                                modal.cursor_col = 0;
                                modal.is_dirty = true;
                                return false;
                            }
                            prefix = "- ".to_string();
                        } else if left.starts_with("* ") {
                            if left.trim() == "*" {
                                modal.lines[modal.cursor_row] = right;
                                modal.cursor_col = 0;
                                modal.is_dirty = true;
                                return false;
                            }
                            prefix = "* ".to_string();
                        } else if left.starts_with("  ") {
                            let spaces: String = left.chars().take_while(|c| *c == ' ').collect();
                            prefix = spaces;
                        }

                        let prefix_len = prefix.chars().count();
                        modal.lines[modal.cursor_row] = left;
                        modal.lines.insert(modal.cursor_row + 1, format!("{}{}", prefix, right));
                        modal.cursor_row += 1;
                        modal.cursor_col = prefix_len;
                        modal.is_dirty = true;
                    }
                    KeyCode::Char(c) => {
                        insert_char_at(&mut modal.lines[modal.cursor_row], modal.cursor_col, c);
                        modal.cursor_col += 1;
                        modal.is_dirty = true;
                    }
                    _ => {}
                }
            }
        }
        false
    }

    // ==========================================
    // TAB 3: CONFIG & PROJECT PATHS HANDLERS
    // ==========================================
    fn handle_config_keys(&mut self, key: KeyEvent) -> bool {
        // Ctrl+S saves anytime in Config tab
        if key.modifiers.contains(KeyModifiers::CONTROL) && (key.code == KeyCode::Char('s') || key.code == KeyCode::Char('S')) {
            self.execute_save_config();
            return false;
        }

        // 0: Workspace Root, 1: Default Author, 2: Default License, 3: Default Template, 4: Icon Mode, 5: Save Button, 6: Init Button
        let is_text_field = self.config_field_idx == 0 || self.config_field_idx == 1 || self.config_field_idx == 3;

        match key.code {
            KeyCode::Char('q') if !is_text_field => {
                return true;
            }
            KeyCode::Up => {
                if self.config_field_idx > 0 {
                    self.config_field_idx -= 1;
                }
            }
            KeyCode::Down => {
                if self.config_field_idx < 6 {
                    self.config_field_idx += 1;
                }
            }
            KeyCode::Char(' ') => {
                if self.config_field_idx == 2 {
                    let licenses = ["MIT", "Apache-2.0", "BSD-3-Clause", "Unlicense", "None"];
                    let cur_idx = licenses
                        .iter()
                        .position(|l| *l == self.config.default_license)
                        .unwrap_or(0);
                    self.config.default_license = licenses[(cur_idx + 1) % licenses.len()].to_string();
                } else if self.config_field_idx == 4 {
                    self.config.cycle_icon_mode();
                    let mode = if self.config.use_nerd_font() { "Nerd Font (enabled)" } else { "ASCII (standard)" };
                    self.status_message = Some(format!("Icon mode set to: {}", mode));
                } else if self.config_field_idx == 5 {
                    self.execute_save_config();
                } else if self.config_field_idx == 6 {
                    self.initialize_master_workspace();
                } else if is_text_field {
                    self.append_char_config(' ');
                }
            }
            KeyCode::Backspace => {
                if is_text_field {
                    match self.config_field_idx {
                        0 => { self.config.workspace_root.pop(); }
                        1 => { self.config.default_author.pop(); }
                        3 => { self.config.default_template.pop(); }
                        _ => {}
                    }
                }
            }
            KeyCode::Enter => {
                if self.config_field_idx == 2 {
                    let licenses = ["MIT", "Apache-2.0", "BSD-3-Clause", "Unlicense", "None"];
                    let cur_idx = licenses
                        .iter()
                        .position(|l| *l == self.config.default_license)
                        .unwrap_or(0);
                    self.config.default_license = licenses[(cur_idx + 1) % licenses.len()].to_string();
                } else if self.config_field_idx == 4 {
                    self.config.cycle_icon_mode();
                    let mode = if self.config.use_nerd_font() { "Nerd Font (enabled)" } else { "ASCII (standard)" };
                    self.status_message = Some(format!("Icon mode set to: {}", mode));
                } else if self.config_field_idx == 5 {
                    self.execute_save_config();
                } else if self.config_field_idx == 6 {
                    self.initialize_master_workspace();
                } else if self.config_field_idx < 6 {
                    self.config_field_idx += 1;
                }
            }
            KeyCode::Char(c)
                if is_text_field => {
                    self.append_char_config(c);
                }
            _ => {}
        }
        false
    }

    fn execute_save_config(&mut self) {
        if let Err(e) = save_config(&self.config) {
            self.popup_message = Some(format!("[ERROR] Failed to save config: {}", e));
        } else {
            self.update_destination_path();
            self.reload_projects();
            self.popup_message = Some(format!(
                "[OK] Configuration Saved!\n\nWorkspace Root: {}\n\nGuaranteed Workspace Structure:\n  ├── personal/  (Personal projects & prototypes)\n  ├── clients/   (Client work & deliverables)\n  ├── resources/ (Shared master asset library)\n  └── archive/   (Yearly cold storage)",
                self.config.workspace_root
            ));
        }
    }

    fn append_char_config(&mut self, c: char) {
        match self.config_field_idx {
            0 => self.config.workspace_root.push(c),
            1 => self.config.default_author.push(c),
            3 => self.config.default_template.push(c),
            _ => {}
        }
    }

    fn initialize_master_workspace(&mut self) {
        match self.config.init_master_workspace() {
            Ok(summary) => {
                self.update_destination_path();
                self.reload_projects();
                self.popup_message = Some(format!(
                    "[OK] Master Workspace Initialized Successfully!\n\nRoot: {}\nCreated {} directories\nCreated {} guide files\n\nStructure:\n  ├── {}/ (Personal projects)\n  ├── {}/ (Clients & work)\n  ├── {}/ (Resource library)\n  └── {}/ (Yearly archives)",
                    self.config.workspace_root,
                    summary.created_dirs,
                    summary.created_readme_files,
                    self.config.personal_dir,
                    self.config.clients_dir,
                    self.config.resources_dir,
                    self.config.archive_dir
                ));
            }
            Err(e) => {
                self.popup_message = Some(format!("[ERROR] Workspace initialization failed: {}", e));
            }
        }
    }
    pub fn handle_mouse(&mut self, mouse: MouseEvent) {
        // If markdown editor is open, mouse wheel scrolls preview or moves cursor
        if let Some(ref mut modal) = self.md_editor_modal {
            match mouse.kind {
                MouseEventKind::ScrollDown => {
                    match modal.mode {
                        MarkdownModalMode::Preview => {
                            if modal.scroll_offset + 1 < modal.lines.len() {
                                modal.scroll_offset += 1;
                            }
                        }
                        MarkdownModalMode::Edit => {
                            if modal.cursor_row + 1 < modal.lines.len() {
                                modal.cursor_row += 1;
                                let len = modal.lines[modal.cursor_row].chars().count();
                                modal.cursor_col = modal.cursor_col.min(len);
                            }
                        }
                    }
                }
                MouseEventKind::ScrollUp => {
                    match modal.mode {
                        MarkdownModalMode::Preview => {
                            modal.scroll_offset = modal.scroll_offset.saturating_sub(1);
                        }
                        MarkdownModalMode::Edit => {
                            if modal.cursor_row > 0 {
                                modal.cursor_row -= 1;
                                let len = modal.lines[modal.cursor_row].chars().count();
                                modal.cursor_col = modal.cursor_col.min(len);
                            }
                        }
                    }
                }
                _ => {}
            }
            return;
        }

        // If tree popup is open, mouse wheel scrolls tree lines
        if let Some((_, lines, scroll)) = self.tree_popup.as_mut() {
            match mouse.kind {
                MouseEventKind::ScrollDown => {
                    if *scroll + 1 < lines.len() {
                        *scroll += 1;
                    }
                }
                MouseEventKind::ScrollUp
                    if *scroll > 0 => {
                        *scroll -= 1;
                    }
                _ => {}
            }
            return;
        }

        match mouse.kind {
            MouseEventKind::ScrollDown => {
                match self.current_tab {
                    0 => {
                        let count = self.filtered_projects().len();
                        if count > 0 && self.selected_project_idx < count - 1 {
                            self.selected_project_idx += 1;
                        }
                    }
                    1 => {
                        if self.create_template_idx < self.templates.len().saturating_sub(1) {
                            self.create_template_idx += 1;
                        }
                    }
                    2
                        if self.templates_view_idx < self.filtered_templates_for_view().len().saturating_sub(1) => {
                            self.templates_view_idx += 1;
                        }
                    _ => {}
                }
            }
            MouseEventKind::ScrollUp => {
                match self.current_tab {
                    0 => {
                        if self.selected_project_idx > 0 {
                            self.selected_project_idx -= 1;
                        }
                    }
                    1 => {
                        if self.create_template_idx > 0 {
                            self.create_template_idx -= 1;
                        }
                    }
                    2
                        if self.templates_view_idx > 0 => {
                            self.templates_view_idx -= 1;
                        }
                    _ => {}
                }
            }
            MouseEventKind::Down(MouseButton::Left)
                // Click on header tabs or logo/help (row 0 to 2)
                if mouse.row <= 2 => {
                    let col = mouse.column;
                    if col < 24 {
                        self.show_help_modal = !self.show_help_modal;
                    } else if col < 42 {
                        self.current_tab = 0;
                        self.reload_projects();
                    } else if col < 62 {
                        self.current_tab = 1;
                    } else if col < 80 {
                        self.current_tab = 2;
                    } else {
                        self.current_tab = 3;
                    }
                }
            _ => {}
        }
    }
}

pub fn open_project_folder(path: &Path) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(path).spawn()?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer").arg(path).spawn()?;
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        let _ = std::process::Command::new("xdg-open").arg(path).spawn();
    }
    Ok(())
}

pub fn scan_workspace_projects(cfg: &AppConfig) -> Vec<ProjectItem> {
    let mut list = Vec::new();

    // 1. Personal projects (both direct in personal/ and nested sub-categories)
    let p_path = cfg.personal_path();
    scan_category_dir(&p_path, ProjectCategory::Personal, &mut list);

    // 2. Client projects (organized by clients/<client_name>/<project_name>)
    let c_path = cfg.clients_path();
    if let Ok(clients) = fs::read_dir(&c_path) {
        for client_entry in clients.flatten() {
            let cp = client_entry.path();
            if cp.is_dir() {
                let client_name = cp
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Client")
                    .to_string();
                if client_name != "_templates" && !client_name.starts_with('.') {
                    scan_category_dir(&cp, ProjectCategory::Client(client_name), &mut list);
                }
            }
        }
    }

    // 3. Shared Resources (only packages / folders with actual assets)
    let r_path = cfg.resources_path();
    scan_category_dir(&r_path, ProjectCategory::Resource, &mut list);

    // 4. Archive (organized by archive/<year>/<project_name> or archive/<year>/clients/<client>/<proj>)
    let a_path = cfg.archive_path();
    if let Ok(years) = fs::read_dir(&a_path) {
        for year_entry in years.flatten() {
            let yp = year_entry.path();
            if yp.is_dir() {
                let year_name = yp
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Archive")
                    .to_string();

                if !year_name.starts_with('.') {
                    scan_category_dir(&yp, ProjectCategory::Archive(year_name.clone()), &mut list);

                    // Check if archived clients folder exists
                    let archived_clients = yp.join("clients");
                    if archived_clients.is_dir()
                        && let Ok(clients) = fs::read_dir(&archived_clients) {
                            for client_entry in clients.flatten() {
                                let cpath = client_entry.path();
                                if cpath.is_dir() {
                                    let cname = cpath
                                        .file_name()
                                        .and_then(|n| n.to_str())
                                        .unwrap_or("Client")
                                        .to_string();
                                    scan_category_dir(
                                        &cpath,
                                        ProjectCategory::Archive(format!("{}/{}", year_name, cname)),
                                        &mut list,
                                    );
                                }
                            }
                        }
                }
            }
        }
    }

    list
}

fn scan_category_dir(base: &Path, cat: ProjectCategory, list: &mut Vec<ProjectItem>) {
    let Ok(entries) = fs::read_dir(base) else {
        return;
    };

    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.starts_with('.') || name == "target" || name == "node_modules" || name == "_templates" || name == "clients" {
                continue;
            }

            let mut file_count = 0;
            let mut dir_count = 0;
            let mut total_bytes = 0;
            let mut preview_files = Vec::new();

            for item in walkdir::WalkDir::new(&p).max_depth(3).into_iter().flatten() {
                let ipath = item.path();
                if ipath.is_file() {
                    let fname = ipath.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if !fname.starts_with('.') {
                        file_count += 1;
                        total_bytes += item.metadata().map(|m| m.len()).unwrap_or(0);
                        if preview_files.len() < 12
                            && let Ok(rel) = ipath.strip_prefix(&p) {
                                preview_files.push(rel.to_string_lossy().to_string());
                            }
                    }
                } else if ipath.is_dir() && ipath != p {
                    dir_count += 1;
                }
            }

            // A directory is NOT an existing project if it is empty!
            // Also if it is a container folder with sub-projects, scan its children.
            if file_count == 0 {
                // Check if it is a container of nested projects (e.g. personal/active/<project>)
                if dir_count > 0 {
                    scan_category_dir(&p, cat.clone(), list);
                }
                continue;
            }

            let detected = crate::maintainer::detect_project_type(&p)
                .unwrap_or("general")
                .to_string();

            list.push(ProjectItem {
                name: name.to_string(),
                category: cat.clone(),
                path: p,
                file_count,
                dir_count,
                total_bytes,
                detected_type: detected,
                preview_files,
            });
        }
    }
}

fn load_all_templates(config: &AppConfig) -> Vec<Template> {
    let mut list = get_builtin_templates();
    let mut loaded_paths = std::collections::HashSet::new();

    let custom_dir = PathBuf::from(&config.custom_templates_dir);
    if let Ok(entries) = fs::read_dir(&custom_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file()
                && let Ok(mut t) = load_template_from_file(&p) {
                    t.kind = crate::template::TemplateKind::Manual;
                    t.source_path = Some(p.display().to_string());
                    loaded_paths.insert(p.clone());
                    list.push(t);
                }
        }
    }

    if let Some(user_dir) = get_user_templates_dir()
        && user_dir != custom_dir
        && let Ok(entries) = fs::read_dir(user_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file()
                    && !loaded_paths.contains(&p)
                    && let Ok(mut t) = load_template_from_file(&p) {
                        t.kind = crate::template::TemplateKind::Manual;
                        t.source_path = Some(p.display().to_string());
                        loaded_paths.insert(p.clone());
                        list.push(t);
                    }
            }
        }

    let local_dir = get_local_templates_dir();
    if let Ok(entries) = fs::read_dir(local_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file()
                && !loaded_paths.contains(&p)
                && let Ok(mut t) = load_template_from_file(&p) {
                    t.kind = crate::template::TemplateKind::Manual;
                    t.source_path = Some(p.display().to_string());
                    list.push(t);
                }
        }
    }

    crate::template::sort_templates_with_favorites(list, &config.favorite_templates)
}

pub fn run_tui(root: PathBuf) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(root)?;
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("TUI Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if app.handle_key(key) {
                        return Ok(());
                    }
                }
                Event::Mouse(mouse) => {
                    app.handle_mouse(mouse);
                }
                _ => {}
            }
        }
    }
}

fn insert_str_at(s: &mut String, col: usize, to_insert: &str) {
    let mut chars: Vec<char> = s.chars().collect();
    if col >= chars.len() {
        chars.extend(to_insert.chars());
    } else {
        for (i, c) in to_insert.chars().enumerate() {
            chars.insert(col + i, c);
        }
    }
    *s = chars.into_iter().collect();
}

fn insert_char_at(s: &mut String, col: usize, c: char) {
    let mut chars: Vec<char> = s.chars().collect();
    if col >= chars.len() {
        chars.push(c);
    } else {
        chars.insert(col, c);
    }
    *s = chars.into_iter().collect();
}

fn remove_char_at(s: &mut String, col: usize) {
    let mut chars: Vec<char> = s.chars().collect();
    if col < chars.len() {
        chars.remove(col);
        *s = chars.into_iter().collect();
    }
}
