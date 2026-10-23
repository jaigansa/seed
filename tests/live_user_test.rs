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

    // 2. User scaffolds a personal podcast project
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &cfg_file)
        .args(["new", "weekly-podcast", "-c", "personal", "-t", "podcast", "-a", "HostJane", "--git"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let podcast_dir = ws_root.join("personal/weekly-podcast");
    assert!(podcast_dir.join("show_notes/template.md").exists());
    assert!(podcast_dir.join("episodes/ep01/plan.md").exists());

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
            "client-project",
            "-a",
            "Freelancer",
        ])
        .output()
        .unwrap();
    assert!(out.status.success());
    let client_dir = ws_root.join("clients/AcmeCorp/summer-campaign");
    assert!(client_dir.join("brief_and_contracts").exists());
    assert!(client_dir.join("deliverables_final").exists());
    assert!(client_dir.join("invoices").exists());

    // 4. User inspects directory tree with 'seed tree'
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &cfg_file)
        .args(["tree", podcast_dir.to_str().unwrap(), "-L", "2"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("episodes"));
    assert!(stdout.contains("show_notes"));

    // 5. User checks project health with 'seed check' (should be 100/100 initially)
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &cfg_file)
        .args(["check", podcast_dir.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Health Score") && stdout.contains("100/100"));

    // 6. User accidentally pollutes project with junk files and removes README
    fs::write(podcast_dir.join(".DS_Store"), "junk").unwrap();
    fs::write(podcast_dir.join("notes.tmp"), "temp").unwrap();
    fs::remove_file(podcast_dir.join("README.md")).unwrap();

    // Check again -> health score must be degraded and issues reported
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &cfg_file)
        .args(["check", podcast_dir.to_str().unwrap()])
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
        .args(["sync", podcast_dir.to_str().unwrap(), "--clean"])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(!podcast_dir.join(".DS_Store").exists());
    assert!(!podcast_dir.join("notes.tmp").exists());
    assert!(podcast_dir.join("README.md").exists());

    // Verify health is restored to 100
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &cfg_file)
        .args(["check", podcast_dir.to_str().unwrap()])
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
            podcast_dir.to_str().unwrap(),
            "--name",
            "my-podcast-v2",
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

    // CRITICAL USER BUG FIX VERIFICATION:
    // When user types numbers '1', '2', '3', '4' in project name,
    // it MUST type into project name, NOT jump to Tab 0, 1, 2, 3!
    app.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('1'), KeyModifiers::NONE));

    assert_eq!(app.current_tab, 1, "Tab must NOT jump when typing digits in project name!");
    assert_eq!(app.create_project_name, "v2a1");

    // 3. User hits Tab while in text field -> must advance to next field!
    app.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert_eq!(app.current_tab, 1, "Tab must advance field, not jump sections when editing!");
    assert_eq!(app.create_field_idx, 2, "Field must advance to Author");

    // 4. User hits Enter while on a non-button field -> advances field
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(app.create_field_idx, 3, "Enter must advance to License field");

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
    app.current_tab = 3;
    app.config_field_idx = 4; // Icon Mode field
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

    // Press Left arrow from non-text field -> smoothly returns to Template list container
    app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert_eq!(app.create_pane, 0, "Left arrow on non-text field must return to Template list");

    // 10. Test Preset / Manual Templates feature in Tab 2 (Templates)
    app.current_tab = 2; // Templates section
    assert_eq!(app.templates_filter_idx, 0, "Initial filter is All");
    let all_count = app.filtered_templates_for_view().len();
    assert!(all_count >= 30);

    // Right arrow -> cycles to Preset filter
    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(app.templates_filter_idx, 1, "Filter changed to Preset");
    for (_, tpl) in app.filtered_templates_for_view() {
        assert!(tpl.is_preset());
    }

    // Right arrow -> cycles to Manual filter
    app.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert_eq!(app.templates_filter_idx, 2, "Filter changed to Manual");

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
    assert_eq!(modal.is_dirty, false);
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
