use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use seed::tui::{App, MarkdownModalMode};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::tempdir;

fn seed_bin() -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    path.join("seed")
}

#[test]
fn test_live_user_workflow_cli() {
    let bin = seed_bin();
    let tmp = tempdir().unwrap();
    let ws_root = tmp.path().join("UserWorkspace");
    let cfg_file = tmp.path().join("config.toml");

    // 1. Live user initializes their master creative workspace
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &cfg_file)
        .args(["workspace", ws_root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Master Workspace initialized"));
    assert!(ws_root.join("personal").exists());
    assert!(ws_root.join("clients").exists());
    assert!(ws_root.join("resources").exists());
    assert!(ws_root.join("archive").exists());

    // 2. User scaffolds a personal 3D model project
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &cfg_file)
        .args(["new", "character-asset", "-c", "personal", "-t", "3dmodel", "-a", "HostJane", "--git"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let project_dir = ws_root.join("personal/character-asset");
    assert!(project_dir.join("models/specs.md").exists());
    assert!(project_dir.join("README.md").exists());

    // 3. User scaffolds a client commission under 'AcmeCorp'
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &cfg_file)
        .args([
            "new",
            "summer-campaign",
            "-c",
            "client",
            "--client",
            "AcmeCorp",
            "-t",
            "logo",
            "-a",
            "Freelancer",
        ])
        .output()
        .unwrap();
    assert!(out.status.success());
    let client_dir = ws_root.join("clients/AcmeCorp/summer-campaign");
    assert!(client_dir.join("guidelines/brand_guidelines.md").exists());
    assert!(client_dir.join("source/color_palettes/palette.md").exists());

    // 4. User inspects directory tree with 'seed tree'
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &cfg_file)
        .args(["tree", project_dir.to_str().unwrap(), "-L", "2"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("models"));
    assert!(stdout.contains("textures"));

    // 5. User checks project health with 'seed check' (should be 100/100 initially)
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &cfg_file)
        .args(["check", project_dir.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Health Score") && stdout.contains("100/100"));

    // 6. User accidentally pollutes project with junk files and removes README
    fs::write(project_dir.join(".DS_Store"), "junk").unwrap();
    fs::write(project_dir.join("notes.tmp"), "temp").unwrap();
    fs::remove_file(project_dir.join("README.md")).unwrap();

    // Check again -> health score must be degraded and issues reported
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &cfg_file)
        .args(["check", project_dir.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[CLEAN]") && stdout.contains("junk / artifact file"));
    assert!(stdout.contains(".DS_Store"));
    assert!(stdout.contains("README.md is missing"));

    // 7. User runs 'seed sync --clean' to repair the project
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &cfg_file)
        .args(["sync", project_dir.to_str().unwrap(), "--clean"])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(!project_dir.join(".DS_Store").exists());
    assert!(!project_dir.join("notes.tmp").exists());
    assert!(project_dir.join("README.md").exists());

    // Verify health is restored to 100
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &cfg_file)
        .args(["check", project_dir.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Health Score") && stdout.contains("100/100"));

    // 8. User captures a project structure into a custom template
    let template_file = tmp.path().join("my-custom-tpl.yaml");
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &cfg_file)
        .args([
            "capture",
            project_dir.to_str().unwrap(),
            "--name",
            "my-asset-v2",
            "--output",
            template_file.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(template_file.exists());

    // 9. User archives completed client project
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &cfg_file)
        .args(["archive", client_dir.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(!client_dir.exists());
    let archive_dir = ws_root.join("archive");
    let year = chrono::Utc::now().format("%Y").to_string();
    assert!(archive_dir.join(&year).join("clients/AcmeCorp/summer-campaign").exists());
}

#[test]
fn test_live_user_tui_state_and_input_fixes() {
    let tmp = tempdir().unwrap();
    let ws_root = tmp.path().join("Projects");
    fs::create_dir_all(&ws_root).unwrap();

    let mut app = App::new(ws_root.clone()).unwrap();
    app.config.workspace_root = ws_root.to_string_lossy().to_string();
    app.config.custom_templates_dir = tmp.path().join("templates").to_string_lossy().to_string();
    app.config.favorite_templates.clear();
    app.config.icon_mode = "ascii".to_string();
    app.reload_templates();

    // 1. Verify user is initially on Tab 0 (View Projects)
    assert_eq!(app.current_tab, 0);

    // 2. Switch to Tab 1 (New Project) and focus project name field (create_field_idx = 1)
    app.current_tab = 1;
    app.create_pane = 1;
    app.create_field_idx = 1;
    app.create_project_name.clear();

    // In Navigation Mode, pressing '1'..='4' would switch tabs.
    // Entering Edit Mode via Enter activates text editing!
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(app.is_input_editing, "Enter activates edit mode on text field");

    // CRITICAL USER BUG FIX VERIFICATION:
    // When user types numbers '1', '2', '3', '4' in project name while editing,
    // it MUST type into project name, NOT jump to Tab 0, 1, 2, 3!
    app.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('1'), KeyModifiers::NONE));

    assert_eq!(app.current_tab, 1, "Tab must NOT jump when typing digits in project name!");
    assert_eq!(app.create_project_name, "v2a1");

    // 3. User hits Tab while in text field -> must advance to next field!
    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(app.current_tab, 1, "Tab must stay on Tab 1");
    assert_eq!(app.create_pane, 1, "Tab must advance form fields, NOT collapse container!");
    assert_eq!(app.create_field_idx, 2, "Field must advance to Author");

    // BackTab goes back to previous field
    app.handle_key(KeyEvent::new(KeyCode::BackTab, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 1, "BackTab must stay in form");
    assert_eq!(app.create_field_idx, 1, "BackTab must return to Title");

    // Esc cleanly returns focus to Template List (Pane 0)
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 0, "Esc cleanly returns to Template List");

    // Tab from Template List enters Form container (Pane 1)
    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 1, "Tab from templates enters Form container");

    // User enters edit mode and advances field using Enter
    app.create_field_idx = 1;
    app.is_input_editing = true;
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(app.create_field_idx, 2, "Enter while editing commits and advances to Author");

    // 4. User hits Enter (enters edit mode) and Enter again (commits & advances to License)
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(app.is_input_editing, "First Enter enters edit mode on Author");
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(app.create_field_idx, 3, "Second Enter commits and advances to License field");

    // 5. Test empty project name validation
    app.create_project_name = "   ".to_string();
    app.create_field_idx = 5; // create button field
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(app.popup_message.is_some(), "Must show error popup for empty project name!");
    let msg = app.popup_message.unwrap();
    assert!(msg.contains("cannot be empty"));

    // Dismiss popup
    app.popup_message = None;

    // 6. Test favorite toggle hotkey [F] on Tab 1
    let initial_fav_count = app.config.favorite_templates.len();
    let current_template_name = app.templates[app.create_template_idx].name.clone();

    // Not in text field -> press 'f'
    app.create_field_idx = 0;
    app.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    assert!(app.config.is_favorite(&current_template_name));
    assert_eq!(app.config.favorite_templates.len(), initial_fav_count + 1);

    // Toggle again -> unpins
    app.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    assert!(!app.config.is_favorite(&current_template_name));
    assert_eq!(app.config.favorite_templates.len(), initial_fav_count);

    // 7. Test Icon Mode cycling in Tab 3 (Config)
    app.current_tab = 2;
    app.config_field_idx = 5; // Icon Mode field
    assert_eq!(app.config.icon_mode, "ascii");
    app.handle_key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
    assert_eq!(app.config.icon_mode, "nerdfont");
    assert!(app.config.use_nerd_font());
    app.handle_key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
    assert_eq!(app.config.icon_mode, "ascii");
    assert!(!app.config.use_nerd_font());

    // 8. Test Target Path Space cycle method on Tab 1 (New Project)
    app.current_tab = 1;
    app.create_pane = 1; // Form container focused
    app.create_field_idx = 0; // Target Path field
    app.create_project_name = "awesome-tool".to_string();
    app.create_client_name = "GlobexCorp".to_string();
    app.create_category_idx = 0; // Personal
    app.update_destination_path();
    assert!(app.create_custom_path.contains("personal/awesome-tool"));

    // Cycle to Client via Space
    app.handle_key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
    assert_eq!(app.create_category_idx, 1);
    assert!(app.create_custom_path.contains("clients/GlobexCorp/awesome-tool"));

    // Cycle to Resource via Space
    app.handle_key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
    assert_eq!(app.create_category_idx, 2);
    assert!(app.create_custom_path.contains("resources/awesome-tool"));

    // Cycle back to Personal via Space
    app.handle_key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
    assert_eq!(app.create_category_idx, 0);
    assert!(app.create_custom_path.contains("personal/awesome-tool"));

    // Enter on field 0 advances to Project Name (field 1)
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(app.create_field_idx, 1);

    // 9. Test Container Switching and Border Focus
    // Focus template list container (create_pane = 0)
    app.create_pane = 0;
    let initial_tpl_idx = app.create_template_idx;
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(app.create_template_idx, initial_tpl_idx + 1, "Down arrow browses templates list");
    app.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(app.create_template_idx, initial_tpl_idx, "Up arrow browses templates list");

    // Press Tab from Template list -> smoothly focuses Form container
    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 1, "Tab must focus Form container");
    assert_eq!(app.create_field_idx, 0);

    // Left and Right arrows change Target value without collapsing pane
    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 1, "Right arrow must NOT collapse Form container");
    assert_eq!(app.create_category_idx, 1, "Right arrow cycles category to Client");

    app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 1, "Left arrow must NOT collapse Form container");
    assert_eq!(app.create_category_idx, 0, "Left arrow cycles category back to Personal");

    // Esc returns to Template list container
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 0, "Esc cleanly returns to Template list");

    // 10. Test Preset / Manual Templates feature in New Project modal (Tab 1)
    app.current_tab = 1;
    app.create_pane = 0; // Template list container
    assert_eq!(app.create_template_filter_idx, 0, "Initial filter is All");
    let all_count = app.filtered_templates_for_create().len();
    assert!(all_count >= 8);

    // Right arrow -> cycles to Preset filter
    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(app.create_template_filter_idx, 1, "Filter changed to Preset");
    for (_, tpl) in app.filtered_templates_for_create() {
        assert!(tpl.is_preset());
    }

    // Right arrow -> cycles to Manual filter
    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(app.create_template_filter_idx, 2, "Filter changed to Manual");

    // Press 'm' to open New Manual Template creator modal
    app.handle_key(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE));
    assert!(app.manual_template_modal.is_some(), "Pressing 'm' opens manual template modal");

    // Type template name: "custom-suite"
    for c in "custom-suite".chars() {
        app.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    assert_eq!(app.manual_template_modal.as_ref().unwrap().name, "custom-suite");

    // Advance to Save Button
    let form = app.manual_template_modal.as_mut().unwrap();
    form.active_field = 6; // Save button

    // Enter to save manual template
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(app.manual_template_modal.is_none(), "Modal closes after saving");
    assert!(app.popup_message.is_some(), "Success popup shown after creating manual template");
    assert!(app.popup_message.as_ref().unwrap().contains("custom-suite"));

    // Check that custom-suite is now in templates and is recognized as Manual!
    let created = app.templates.iter().find(|t| t.name == "custom-suite");
    assert!(created.is_some(), "New manual template must exist in app.templates");
    assert!(created.unwrap().is_manual(), "Must be categorized as a Manual template");
    assert!(!created.unwrap().is_preset());
}

#[test]
fn test_easy_to_use_ui_ux_enhancements() {
    let tmp = tempdir().unwrap();
    let ws_root = tmp.path().join("UserWorkspace");
    fs::create_dir_all(&ws_root).unwrap();

    let mut app = App::new(ws_root.clone()).unwrap();
    app.config.workspace_root = ws_root.to_string_lossy().to_string();
    app.config.custom_templates_dir = tmp.path().join("templates").to_string_lossy().to_string();
    app.reload_templates();
    app.reload_projects();

    // 1. Verify Help Cheat Sheet modal toggle (? / F1)
    assert!(!app.show_help_modal);
    app.handle_key(KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE));
    assert!(app.show_help_modal, "? must open the help modal");

    // Close help modal via Esc
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert!(!app.show_help_modal, "Esc must close the help modal");

    // Open via F1
    app.handle_key(KeyEvent::new(KeyCode::F(1), KeyModifiers::NONE));
    assert!(app.show_help_modal, "F1 must open the help modal");

    // Close via Space
    app.handle_key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
    assert!(!app.show_help_modal, "Space must close the help modal");

    // 2. Verify Tab 0 Category Filter wrapping (Left & Right)
    assert_eq!(app.current_tab, 0);
    assert_eq!(app.projects_filter_idx, 0); // All

    // Left wraps to index 4 (Archive)
    app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(app.projects_filter_idx, 4, "Left arrow on All filter must wrap to Archive");

    // Right wraps to index 0 (All)
    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(app.projects_filter_idx, 0, "Right arrow on Archive must wrap to All");

    // 3. Verify Direct Navigation to New Project via [N]
    app.handle_key(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));
    assert_eq!(app.current_tab, 1, "[N] key must jump directly to Tab 1 (New Project)");

    // 4. Verify Template Filter cycling in Tab 1 (Create Project, Pane 0)
    app.create_pane = 0;
    assert_eq!(app.create_template_filter_idx, 0); // All

    // Right cycles to Preset (1)
    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(app.create_template_filter_idx, 1, "Right cycles to Preset filter");

    // Right cycles to Manual (2)
    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(app.create_template_filter_idx, 2, "Right cycles to Manual filter");

    // Right wraps back to All (0)
    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(app.create_template_filter_idx, 0, "Right wraps back to All filter");

    // Left wraps to Manual (2)
    app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(app.create_template_filter_idx, 2, "Left wraps back to Manual filter");

    // Reset filter to All
    app.create_template_filter_idx = 0;

    // 5. Verify Smooth Project Creation Auto-Selection & Tab Jump
    app.create_project_name = "brand-new-engine".to_string();
    app.update_destination_path();
    app.create_pane = 1;
    app.create_field_idx = 5; // Create button for Personal category

    // Hit Enter to execute creation
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    // Verify success popup, automatic switch to Tab 0, and auto-selection of the new project
    assert!(app.popup_message.is_some(), "Must display success popup");
    let popup = app.popup_message.as_ref().unwrap();
    assert!(popup.contains("[OK]") && popup.contains("brand-new-engine"));
    assert_eq!(app.current_tab, 0, "Must automatically jump to Tab 0 (View Projects) upon creation");

    let filtered = app.filtered_projects();
    assert!(!filtered.is_empty(), "Filtered projects must include the created project");
    let selected = filtered[app.selected_project_idx];
    assert_eq!(selected.name, "brand-new-engine", "Newly created project must be auto-selected in Tab 0");
    assert_eq!(app.create_project_name, "my-project", "Project name must be reset to 'my-project'");
}

#[test]
fn test_create_project_no_crash_and_no_text_overlays() {
    let tmp = tempdir().unwrap();
    let ws_root = tmp.path().join("UserWorkspace");
    fs::create_dir_all(&ws_root).unwrap();

    let mut app = App::new(ws_root.clone()).unwrap();
    app.config.workspace_root = ws_root.to_string_lossy().to_string();
    app.config.custom_templates_dir = tmp.path().join("templates").to_string_lossy().to_string();
    app.reload_templates();
    app.reload_projects();

    let backend = ratatui::backend::TestBackend::new(80, 24);
    let mut terminal = ratatui::Terminal::new(backend).unwrap();

    // 1. Render all tabs in standard 80x24 terminal - verify NO panic or crash
    for tab in 0..=3 {
        app.current_tab = tab;
        terminal.draw(|f| seed::tui::ui::render(f, &mut app)).unwrap();
    }

    // 2. Tab 1 (Create Project) with Manual Filter (0 manual templates)
    app.current_tab = 1;
    app.create_pane = 0;
    app.create_template_filter_idx = 2; // Manual filter
    assert_eq!(app.filtered_templates_for_create().len(), 0);

    // Rendering with empty filtered templates must not crash
    terminal.draw(|f| seed::tui::ui::render(f, &mut app)).unwrap();

    // Arrow keys on empty template list must not crash
    app.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(app.create_template_filter_idx, 2);

    // Attempting to create project when no template matches filter must show error popup, NOT crash
    app.create_pane = 1;
    app.create_field_idx = 5;
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(app.popup_message.is_some());
    assert!(app.popup_message.as_ref().unwrap().contains("No templates available"));
    app.popup_message = None;

    // 3. Test field index clamping when switching from Client (6 fields) to Personal (5 fields)
    app.create_template_filter_idx = 0; // All
    app.create_category_idx = 1; // Client
    app.create_field_idx = 6; // Create button
    app.create_client_name = "MegaCorp".to_string();
    app.update_destination_path();

    // Switch category from Client to Resource (Space on field 0)
    app.create_field_idx = 0;
    app.handle_key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
    assert_eq!(app.create_category_idx, 2); // Resource
    assert!(app.create_field_idx <= 5, "Field index must be clamped to max_fields");

    // 4. Test long path truncation in UI - verify render doesn't crash or overflow
    app.create_project_name = "a-very-long-project-name-that-would-normally-wrap-in-terminal".to_string();
    app.update_destination_path();
    terminal.draw(|f| seed::tui::ui::render(f, &mut app)).unwrap();

    // 5. Create project successfully and verify quiet mode (no stdout output) and clean state reset
    app.create_project_name = "clean-ui-test".to_string();
    app.update_destination_path();
    app.create_pane = 1;
    app.create_field_idx = 5; // Create button

    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(app.popup_message.is_some());
    assert!(app.popup_message.as_ref().unwrap().contains("[OK]"));

    // State reset checks
    assert_eq!(app.current_tab, 0, "Must be back on Tab 0");
    assert_eq!(app.create_pane, 0, "create_pane must be reset to 0");
    assert_eq!(app.create_field_idx, 0, "create_field_idx must be reset to 0");
    assert_eq!(app.projects_filter_idx, 0, "projects_filter_idx must be reset to 0");
    assert_eq!(app.search_query, "", "search_query must be cleared");

    // Rendering popup and help modal must not panic
    terminal.draw(|f| seed::tui::ui::render(f, &mut app)).unwrap();
    app.popup_message = None;

    app.show_help_modal = true;
    terminal.draw(|f| seed::tui::ui::render(f, &mut app)).unwrap();
}

#[test]
fn test_tui_markdown_editor_modal_preview_and_edit() {
    let tmp = tempdir().unwrap();
    let ws_root = tmp.path().join("UserWorkspace");
    fs::create_dir_all(ws_root.join("personal/sample-app")).unwrap();
    fs::write(
        ws_root.join("personal/sample-app/README.md"),
        "# Sample App\n\n> Welcome to sample app.\n\n- [ ] Task 1\n",
    )
    .unwrap();
    fs::write(
        ws_root.join("personal/sample-app/main.rs"),
        "fn main() {}\n",
    )
    .unwrap();

    let mut app = App::new(ws_root.clone()).unwrap();
    app.config.workspace_root = ws_root.to_string_lossy().to_string();
    app.reload_projects();

    let backend = ratatui::backend::TestBackend::new(90, 30);
    let mut terminal = ratatui::Terminal::new(backend).unwrap();

    // 1. Verify project exists on Tab 0
    assert_eq!(app.current_tab, 0);
    assert_eq!(app.filtered_projects().len(), 1);
    assert_eq!(app.filtered_projects()[0].name, "sample-app");

    // 2. Open Markdown editor via 'e'
    assert!(app.md_editor_modal.is_none());
    app.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    assert!(app.md_editor_modal.is_some(), "Markdown modal must open upon pressing 'e'");

    let modal = app.md_editor_modal.as_ref().unwrap();
    assert_eq!(modal.mode, MarkdownModalMode::Preview);
    assert!(!modal.is_dirty);
    assert_eq!(modal.lines.len(), 5);
    assert!(!app.is_editing_text(), "Preview mode should not block tab shortcuts");

    // Render in Preview mode without panic
    terminal.draw(|f| seed::tui::ui::render(f, &mut app)).unwrap();

    // 3. Toggle to Edit mode with Tab
    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(
        app.md_editor_modal.as_ref().unwrap().mode,
        MarkdownModalMode::Edit
    );
    assert!(app.is_editing_text(), "Edit mode must flag is_editing_text as true");

    // Render in Edit mode without panic
    terminal.draw(|f| seed::tui::ui::render(f, &mut app)).unwrap();

    // 4. Type text in Edit mode
    // Move to end of first line: cursor_col is 0 initially, line is "# Sample App"
    app.handle_key(KeyEvent::new(KeyCode::End, KeyModifiers::NONE));
    assert_eq!(app.md_editor_modal.as_ref().unwrap().cursor_col, 12);

    // Type " 2.0"
    for c in " 2.0".chars() {
        app.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    assert_eq!(
        app.md_editor_modal.as_ref().unwrap().lines[0],
        "# Sample App 2.0"
    );
    assert!(app.md_editor_modal.as_ref().unwrap().is_dirty);

    // 5. Test Enter and smart auto-indent / bullet handling
    // Move to the task line: line index 4 ("- [ ] Task 1")
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::End, KeyModifiers::NONE));
    
    // Hit Enter - smart auto-indent should insert "- [ ] "
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(
        app.md_editor_modal.as_ref().unwrap().cursor_row,
        5
    );
    assert_eq!(
        app.md_editor_modal.as_ref().unwrap().lines[5],
        "- [ ] "
    );

    // 6. Test Ctrl+S save
    app.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL));
    assert!(!app.md_editor_modal.as_ref().unwrap().is_dirty, "File should be marked saved");
    let saved_content = fs::read_to_string(ws_root.join("personal/sample-app/README.md")).unwrap();
    assert!(saved_content.contains("# Sample App 2.0"));
    assert!(saved_content.contains("- [ ] Task 1"));

    // 7. Toggle back to Preview mode with Tab and Esc to close
    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(
        app.md_editor_modal.as_ref().unwrap().mode,
        MarkdownModalMode::Preview
    );
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert!(app.md_editor_modal.is_none(), "Esc must close markdown modal in Preview mode");

    // 8. Test project with NO README: opens in Edit mode with starter template
    fs::remove_file(ws_root.join("personal/sample-app/README.md")).unwrap();
    app.reload_projects();
    app.handle_key(KeyEvent::new(KeyCode::Char('E'), KeyModifiers::NONE));
    assert!(app.md_editor_modal.is_some());
    assert_eq!(
        app.md_editor_modal.as_ref().unwrap().mode,
        MarkdownModalMode::Edit,
        "Project with no README should open directly in Edit mode"
    );
    assert!(app.md_editor_modal.as_ref().unwrap().is_dirty);
    assert!(app.md_editor_modal.as_ref().unwrap().lines[0].contains("sample-app"));

    // Save newly generated README
    app.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL));
    assert!(ws_root.join("personal/sample-app/README.md").exists(), "README.md must be written to disk");
    assert!(!app.md_editor_modal.as_ref().unwrap().is_dirty);

    // Close modal
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)); // to preview
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)); // close
    assert!(app.md_editor_modal.is_none());
}

#[test]
fn test_key_navigation_and_value_type_no_collapse() {
    let ws = tempdir().unwrap();
    let mut app = App::new(ws.path().to_path_buf()).unwrap();
    app.current_tab = 1; // Tab 1: New Project
    app.create_pane = 1; // Inside Form container

    // 1. Verify Tab advances across all fields and wraps around without collapsing container
    assert_eq!(app.create_field_idx, 0);
    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 1);
    assert_eq!(app.create_field_idx, 1); // Title

    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 1);
    assert_eq!(app.create_field_idx, 2); // Author

    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 1);
    assert_eq!(app.create_field_idx, 3); // License

    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 1);
    assert_eq!(app.create_field_idx, 4); // Git Init

    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 1);
    assert_eq!(app.create_field_idx, 5); // Seed button

    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 1);
    assert_eq!(app.create_field_idx, 0); // Wraps back to Target

    // 2. Verify Left & Right on Target category change value type without collapsing
    assert_eq!(app.create_category_idx, 0); // Personal
    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 1, "Right arrow must NOT collapse Form container");
    assert_eq!(app.create_category_idx, 1, "Right arrow changes category to Client");

    app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 1, "Left arrow must NOT collapse Form container");
    assert_eq!(app.create_category_idx, 0, "Left arrow changes category back to Personal");

    // 3. Verify Left & Right on License change value type without collapsing
    app.create_field_idx = 3; // License field for non-client
    assert_eq!(app.create_license_idx, 0); // MIT
    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 1, "Right arrow must NOT collapse Form container on License");
    assert_eq!(app.create_license_idx, 1); // Apache-2.0

    app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 1, "Left arrow must NOT collapse Form container on License");
    assert_eq!(app.create_license_idx, 0); // MIT

    // 4. Verify Esc cleanly returns to Template List
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 0, "Esc cleanly returns to Template List");

    // 5. Test Config tab Left & Right navigation for License and Icon mode
    app.current_tab = 2;
    app.config_field_idx = 3; // Default License
    let init_lic = app.config.default_license.clone();
    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_ne!(app.config.default_license, init_lic);
    app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(app.config.default_license, init_lic);

    app.config_field_idx = 5; // Icon Mode
    let init_mode = app.config.icon_mode.clone();
    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_ne!(app.config.icon_mode, init_mode);
    app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(app.config.icon_mode, init_mode);

    // 6. Test Config user bug fix:
    // Press '3' to go to Config, then press '1' to go to Tab 0 (View Projects)
    // It must jump to Tab 0, NOT type '1' into workspace_root!
    app.current_tab = 0;
    app.config_field_idx = 0;
    app.handle_key(KeyEvent::new(KeyCode::Char('3'), KeyModifiers::NONE));
    assert_eq!(app.current_tab, 2, "Jump to Config via '3'");
    assert_eq!(app.config_field_idx, 0, "Focused on workspace_root");
    let original_ws = app.config.workspace_root.clone();

    // In Navigation Mode, press '1' -> Must jump to Tab 0 (View Projects) without modifying path!
    app.handle_key(KeyEvent::new(KeyCode::Char('1'), KeyModifiers::NONE));
    assert_eq!(app.current_tab, 0, "Must jump to Tab 1 when '1' is pressed in Navigation Mode!");
    assert_eq!(app.config.workspace_root, original_ws, "Path must NOT be corrupted with '1'!");

    // Jump back to Config, press Enter to enter Edit Mode -> now '1' types into path
    app.handle_key(KeyEvent::new(KeyCode::Char('3'), KeyModifiers::NONE));
    assert_eq!(app.current_tab, 2);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(app.is_input_editing, "Enter activates Edit Mode on text field");
    app.handle_key(KeyEvent::new(KeyCode::Char('1'), KeyModifiers::NONE));
    assert_eq!(app.config.workspace_root, format!("{}1", original_ws), "Typing while in Edit Mode modifies field");

    // Press Esc to exit Edit Mode
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert!(!app.is_input_editing, "Esc exits Edit Mode");

    // Now press '1' again -> jumps back to Tab 0!
    app.handle_key(KeyEvent::new(KeyCode::Char('1'), KeyModifiers::NONE));
    assert_eq!(app.current_tab, 0, "Jump to Tab 1 works again after exiting Edit Mode!");

    // Test Alt+1 quick tab switch even while in Edit Mode
    app.handle_key(KeyEvent::new(KeyCode::Char('4'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(app.is_input_editing);
    app.handle_key(KeyEvent::new(KeyCode::Char('1'), KeyModifiers::ALT));
    assert_eq!(app.current_tab, 0, "Alt+1 immediately jumps to Tab 0 and exits Edit Mode");
    assert!(!app.is_input_editing);
}

#[test]
fn test_unified_dashboard_modal_flow() {
    let tmp = tempdir().unwrap();
    let ws_root = tmp.path().join("UserWorkspace");
    fs::create_dir_all(&ws_root).unwrap();

    let mut app = App::new(ws_root.clone()).unwrap();
    app.config.workspace_root = ws_root.to_string_lossy().to_string();
    app.config.custom_templates_dir = tmp.path().join("templates").to_string_lossy().to_string();
    app.reload_templates();
    app.reload_projects();

    let backend = ratatui::backend::TestBackend::new(100, 30);
    let mut terminal = ratatui::Terminal::new(backend).unwrap();

    // 1. Initial State: Always on Unified Dashboard (current_tab == 0)
    assert_eq!(app.current_tab, 0);
    terminal.draw(|f| seed::tui::ui::render(f, &mut app)).unwrap();

    // 2. Press 'N' -> Opens New Project modal (current_tab == 1)
    app.handle_key(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));
    assert_eq!(app.current_tab, 1, "Pressing 'N' must open New Project modal");
    terminal.draw(|f| seed::tui::ui::render(f, &mut app)).unwrap();

    // Press Esc in template list -> Closes modal back to Dashboard
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(app.current_tab, 0, "Pressing Esc must close New Project modal and return to Dashboard");

    // 3. Press 'C' -> Opens Config Settings modal (current_tab == 2)
    app.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    assert_eq!(app.current_tab, 2, "Pressing 'C' must open Config Settings modal");
    terminal.draw(|f| seed::tui::ui::render(f, &mut app)).unwrap();

    // Verify Custom Templates Folder Path field can be edited in Config
    app.config_field_idx = 1; // Custom Templates Folder Path
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(app.is_input_editing);
    app.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    assert!(app.config.custom_templates_dir.ends_with('x'));
    app.handle_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert!(!app.is_input_editing);

    // Press Esc in Config modal -> Closes modal back to Dashboard
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert_eq!(app.current_tab, 0, "Pressing Esc must close Config modal and return to Dashboard");

    // 5. Create a project via 'N' modal and verify automatic return to Dashboard
    app.handle_key(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));
    assert_eq!(app.current_tab, 1);
    app.create_project_name = "unified-app".to_string();
    app.update_destination_path();
    // Verify modal renders with path in Directory & File Structure Preview
    terminal.draw(|f| seed::tui::ui::render(f, &mut app)).unwrap();

    app.create_field_idx = 5; // Create button
    app.create_pane = 1;
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    // Must be back on dashboard (current_tab == 0) with project selected
    assert_eq!(app.current_tab, 0, "Must return to Dashboard upon project creation");
    assert!(app.filtered_projects().iter().any(|p| p.name == "unified-app"));

    // Dismiss creation success popup with Enter
    assert!(app.popup_message.is_some());
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(app.popup_message.is_none());

    // 6. Test 'P' opens Directory Tree modal on the created project
    assert!(app.tree_popup.is_none());
    app.handle_key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));
    assert!(app.tree_popup.is_some(), "Pressing 'P' opens Directory Tree popup modal");
    terminal.draw(|f| seed::tui::ui::render(f, &mut app)).unwrap();

    // Close Tree modal with Esc
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert!(app.tree_popup.is_none(), "Esc closes Tree modal");
}

#[test]
fn test_no_duplicate_action_key_hints() {
    let tmp = tempdir().unwrap();
    let ws_root = tmp.path().join("UserWorkspace");
    fs::create_dir_all(ws_root.join("personal/demo-app")).unwrap();
    fs::write(ws_root.join("personal/demo-app/README.md"), "# Demo App\nWelcome!").unwrap();

    let mut app = App::new(ws_root.clone()).unwrap();
    app.config.workspace_root = ws_root.to_string_lossy().to_string();
    app.reload_templates();
    app.reload_projects();

    let backend = ratatui::backend::TestBackend::new(120, 35);
    let mut terminal = ratatui::Terminal::new(backend).unwrap();

    // 1. Dashboard (Tab 0)
    terminal.draw(|f| seed::tui::ui::render(f, &mut app)).unwrap();
    let screen = format!("{:?}", terminal.backend());
    assert!(screen.contains("Created:"), "Dashboard card must show project creation date detail");
    assert!(!screen.contains("Actions:"), "Dashboard card must not contain redundant 'Actions:' block");
    assert!(!screen.contains("Press [E] to edit in TUI"), "Redundant editor helper prompt removed");
    assert!(!screen.contains("Selected Project Details & Actions"), "Card title cleaned of '& Actions'");
    assert!(!screen.contains("Filter [←/→] • [/] Search • [N]"), "Filter header cleaned of duplicate shortcuts");

    // 2. New Project Modal (Tab 1)
    app.handle_key(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));
    assert_eq!(app.current_tab, 1);
    terminal.draw(|f| seed::tui::ui::render(f, &mut app)).unwrap();
    let screen = format!("{:?}", terminal.backend());
    assert!(!screen.contains("(Enter to edit)"), "Create form rows must not have repeated '(Enter to edit)'");
    assert!(!screen.contains("(Space)"), "Create form rows must not have repeated '(Space)'");
    assert!(!screen.contains("CREATE & SEED PROJECT NOW ] (Enter)"), "Button must not repeat '(Enter)'");

    // 3. Config Settings Modal (Tab 2)
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    assert_eq!(app.current_tab, 2);
    terminal.draw(|f| seed::tui::ui::render(f, &mut app)).unwrap();
    let screen = format!("{:?}", terminal.backend());
    assert!(screen.contains("Custom Templates Folder Path"), "Config displays template folder path assignment");
    assert!(!screen.contains("(Press Enter to edit)"), "Config rows must not have repeated '(Press Enter to edit)'");
    assert!(!screen.contains("SAVE CONFIGURATION (or Ctrl+S)"), "Save button must not duplicate '(or Ctrl+S)'");
    assert!(!screen.contains("[↑/↓ to navigate, Space to cycle"), "Config box title cleaned of duplicate hotkeys");
}

#[test]
fn test_project_sorting_modes_and_cycle() {
    let tmp = tempdir().unwrap();
    let ws_root = tmp.path().join("UserWorkspace");
    fs::create_dir_all(ws_root.join("personal/alpha-proj")).unwrap();
    fs::write(ws_root.join("personal/alpha-proj/README.md"), "# Alpha").unwrap();

    fs::create_dir_all(ws_root.join("personal/zeta-proj")).unwrap();
    fs::write(ws_root.join("personal/zeta-proj/README.md"), "# Zeta\nExtra large content file").unwrap();

    let mut app = App::new(ws_root.clone()).unwrap();
    app.config.workspace_root = ws_root.to_string_lossy().to_string();
    app.reload_projects();

    assert_eq!(app.sort_mode, seed::tui::ProjectSortMode::Recent);

    // Press 's' to cycle to Alphabetical
    app.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
    assert_eq!(app.sort_mode, seed::tui::ProjectSortMode::Alphabetical);
    assert_eq!(app.projects_list[0].name, "alpha-proj");
    assert_eq!(app.projects_list[1].name, "zeta-proj");

    // Press 's' to cycle to Size
    app.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
    assert_eq!(app.sort_mode, seed::tui::ProjectSortMode::Size);

    // Press 's' to cycle to Health
    app.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
    assert_eq!(app.sort_mode, seed::tui::ProjectSortMode::Health);

    // Press 's' to cycle back to Recent
    app.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
    assert_eq!(app.sort_mode, seed::tui::ProjectSortMode::Recent);
}

#[test]
fn test_pin_project_to_top_of_list() {
    let tmp = tempdir().unwrap();
    let ws_root = tmp.path().join("UserWorkspace");
    fs::create_dir_all(ws_root.join("personal/aaa-proj")).unwrap();
    fs::write(ws_root.join("personal/aaa-proj/README.md"), "# AAA").unwrap();

    fs::create_dir_all(ws_root.join("personal/zzz-proj")).unwrap();
    fs::write(ws_root.join("personal/zzz-proj/README.md"), "# ZZZ").unwrap();

    let mut app = App::new(ws_root.clone()).unwrap();
    app.config.workspace_root = ws_root.to_string_lossy().to_string();
    app.sort_mode = seed::tui::ProjectSortMode::Alphabetical;
    app.reload_projects();

    // Alphabetical order: aaa-proj is 0, zzz-proj is 1
    assert_eq!(app.projects_list[0].name, "aaa-proj");
    assert_eq!(app.projects_list[1].name, "zzz-proj");

    // Select zzz-proj and press 'f' to pin it
    app.selected_project_idx = 1;
    app.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));

    // Now zzz-proj must be pinned and moved to index 0 (top of list)
    assert!(app.config.is_pinned_project("zzz-proj"));
    assert_eq!(app.projects_list[0].name, "zzz-proj");
    assert_eq!(app.projects_list[1].name, "aaa-proj");

    // Press 'f' again on zzz-proj to unpin it
    app.selected_project_idx = 0;
    app.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    assert!(!app.config.is_pinned_project("zzz-proj"));
    assert_eq!(app.projects_list[0].name, "aaa-proj");
    assert_eq!(app.projects_list[1].name, "zzz-proj");
}

#[test]
fn test_markdown_dirty_confirm_discard_dialog() {
    let tmp = tempdir().unwrap();
    let file = tmp.path().join("README.md");
    fs::write(&file, "# Initial Title\n").unwrap();

    let mut app = App::new(tmp.path().to_path_buf()).unwrap();
    app.md_editor_modal = Some(seed::tui::MarkdownEditorModal {
        project_name: "test-proj".to_string(),
        file_path: file.clone(),
        lines: vec!["# Initial Title".to_string(), "Unsaved draft line".to_string()],
        cursor_row: 1,
        cursor_col: 18,
        scroll_offset: 0,
        is_dirty: true,
        mode: MarkdownModalMode::Preview,
    });

    // Press 'q' or 'Esc' in preview mode when dirty -> triggers confirm discard modal
    app.handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
    assert!(app.confirm_discard_md, "Must trigger confirm discard when buffer is dirty");
    assert!(app.md_editor_modal.is_some(), "Buffer must not be discarded immediately");

    // User chooses 'n' (Cancel / keep editing)
    app.handle_key(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));
    assert!(!app.confirm_discard_md, "confirm_discard_md must be cleared");
    assert!(app.md_editor_modal.is_some(), "Buffer remains open");

    // Press 'Esc' again to re-open confirm dialog
    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert!(app.confirm_discard_md);

    // User chooses 'y' (Confirm discard)
    app.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    assert!(!app.confirm_discard_md);
    assert!(app.md_editor_modal.is_none(), "Buffer is now closed after confirmation");
}

#[test]
fn test_shell_completions_cli_command() {
    let bin = seed_bin();

    // 1. Bash completions
    let out = Command::new(&bin).args(["completions", "bash"]).output().unwrap();
    assert!(out.status.success());
    let bash_str = String::from_utf8_lossy(&out.stdout);
    assert!(bash_str.contains("complete -F _seed_completions seed"));
    assert!(bash_str.contains("workspace"));

    // 2. Zsh completions
    let out = Command::new(&bin).args(["completions", "zsh"]).output().unwrap();
    assert!(out.status.success());
    let zsh_str = String::from_utf8_lossy(&out.stdout);
    assert!(zsh_str.contains("#compdef seed"));

    // 3. Fish completions
    let out = Command::new(&bin).args(["completions", "fish"]).output().unwrap();
    assert!(out.status.success());
    let fish_str = String::from_utf8_lossy(&out.stdout);
    assert!(fish_str.contains("complete -c seed"));
}

#[test]
fn test_developer_actions_and_git_status() {
    let tmp = tempdir().unwrap();
    let ws_root = tmp.path().join("UserWorkspace");
    let proj_dir = ws_root.join("personal/git-proj");
    fs::create_dir_all(&proj_dir).unwrap();
    fs::write(proj_dir.join("README.md"), "# Git Proj").unwrap();

    // Initialize git repo
    let _ = Command::new("git").args(["init"]).current_dir(&proj_dir).output();
    let _ = Command::new("git").args(["config", "user.name", "Tester"]).current_dir(&proj_dir).output();
    let _ = Command::new("git").args(["config", "user.email", "test@example.com"]).current_dir(&proj_dir).output();

    // Initially dirty (untracked README)
    let status = seed::utils::check_git_status(&proj_dir);
    assert_eq!(status, seed::utils::GitRepoStatus::Modified);

    // Commit changes -> should be clean
    let _ = Command::new("git").args(["add", "."]).current_dir(&proj_dir).output();
    let _ = Command::new("git").args(["commit", "-m", "init"]).current_dir(&proj_dir).output();

    let status = seed::utils::check_git_status(&proj_dir);
    assert_eq!(status, seed::utils::GitRepoStatus::Clean);

    // Test TUI trigger for shell [$] and IDE [I]
    let mut app = App::new(ws_root.clone()).unwrap();
    app.config.workspace_root = ws_root.to_string_lossy().to_string();
    app.reload_projects();

    // Trigger shell
    app.handle_key(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::NONE));
    assert!(matches!(app.pending_action, Some(seed::tui::PendingAction::OpenTerminal(_))));
    app.pending_action = None;

    // Trigger IDE / Editor
    app.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    // Either pending_action or launched
    assert!(app.pending_action.is_some() || app.status_message.is_some() || app.popup_message.is_some());
}
