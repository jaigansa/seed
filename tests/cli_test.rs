use std::process::Command;
use tempfile::tempdir;

fn seed_bin() -> std::path::PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // drop test binary name
    if path.ends_with("deps") {
        path.pop();
    }
    path.join("seed")
}

#[test]
fn test_cli_version_and_help() {
    let bin = seed_bin();
    let output = Command::new(&bin).arg("--version").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("seed 0.2.0"));

    let help = Command::new(&bin).arg("--help").output().unwrap();
    assert!(help.status.success());
    let help_str = String::from_utf8_lossy(&help.stdout);
    assert!(help_str.contains("Project directory & file creator") || help_str.contains("project directory"));
}

#[test]
fn test_cli_list_templates() {
    let bin = seed_bin();
    let output = Command::new(&bin).arg("list").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("podcast"));
    assert!(stdout.contains("book"));
    assert!(stdout.contains("game"));
    assert!(stdout.contains("music"));
    assert!(stdout.contains("presentation"));
    assert!(stdout.contains("research-paper"));
    assert!(stdout.contains("newsletter"));
    assert!(stdout.contains("portfolio"));
    assert!(stdout.contains("yt-video"));
    assert!(stdout.contains("cad-maker"));
    assert!(stdout.contains("3d-blender"));
    assert!(stdout.contains("rust"));
    assert!(stdout.contains("Deliverables:"));
}

#[test]
fn test_cli_scaffold_output_related_templates() {
    let bin = seed_bin();
    let tmp = tempdir().unwrap();

    // 1. Test Podcast scaffold
    let podcast_target = tmp.path().join("my-daily-show");
    let status = Command::new(&bin)
        .args(["new", podcast_target.to_str().unwrap(), "-t", "podcast", "-a", "AudioHost"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(podcast_target.join("show_notes/template.md").exists());
    assert!(podcast_target.join("episodes/ep01/plan.md").exists());
    assert!(podcast_target.join("assets/artwork/cover/specs.md").exists());

    // 2. Test Book / Manuscript scaffold
    let book_target = tmp.path().join("my-great-novel");
    let status = Command::new(&bin)
        .args(["new", book_target.to_str().unwrap(), "-t", "book", "-a", "AuthorName"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(book_target.join("manuscript/chapters/ch01_the_beginning.md").exists());
    assert!(book_target.join("planning/outline/book_outline.md").exists());
    assert!(book_target.join("scripts/build_ebook.sh").exists());

    // 3. Test Indie Game scaffold
    let game_target = tmp.path().join("pixel-quest");
    let status = Command::new(&bin)
        .args(["new", game_target.to_str().unwrap(), "-t", "game", "-a", "GameDev"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(game_target.join("design/gdd/game_design_document.md").exists());
    assert!(game_target.join("design/level_design/level_01_concept.md").exists());

    // 4. Test Presentation scaffold
    let pres_target = tmp.path().join("investor-deck");
    let status = Command::new(&bin)
        .args(["new", pres_target.to_str().unwrap(), "-t", "presentation", "-a", "Speaker"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(pres_target.join("slides/presentation.md").exists());
    assert!(pres_target.join("handouts/summary.md").exists());
    assert!(pres_target.join("scripts/export_slides.sh").exists());
}

#[test]
fn test_cli_scaffold_regular_templates() {
    let bin = seed_bin();
    let tmp = tempdir().unwrap();

    // 1. Test C++ scaffold
    let cpp_target = tmp.path().join("my-cpp-service");
    let status = Command::new(&bin)
        .args(["new", cpp_target.to_str().unwrap(), "-t", "cpp", "-a", "Tester"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(cpp_target.join("CMakeLists.txt").exists());
    assert!(cpp_target.join("src/main.cpp").exists());
    assert!(cpp_target.join("include/my-cpp-service/core.hpp").exists());

    // 2. Test React scaffold
    let react_target = tmp.path().join("my-react-ui");
    let status = Command::new(&bin)
        .args(["new", react_target.to_str().unwrap(), "-t", "react", "-a", "Tester"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(react_target.join("package.json").exists());
    assert!(react_target.join("vite.config.ts").exists());
    assert!(react_target.join("src/App.tsx").exists());

    // 3. Test FastAPI scaffold
    let api_target = tmp.path().join("my-fastapi-service");
    let status = Command::new(&bin)
        .args(["new", api_target.to_str().unwrap(), "-t", "fastapi", "-a", "Tester"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(api_target.join("app/main.py").exists());
    assert!(api_target.join("app/api/routes/items.py").exists());
    assert!(api_target.join("requirements.txt").exists());

    // 4. Test Bash CLI scaffold
    let bash_target = tmp.path().join("my-bash-tool");
    let status = Command::new(&bin)
        .args(["new", bash_target.to_str().unwrap(), "-t", "bash", "-a", "Tester"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(bash_target.join("bin/my-bash-tool").exists());
    assert!(bash_target.join("lib/utils.sh").exists());
    assert!(bash_target.join("Makefile").exists());
}

#[test]
fn test_cli_scaffold_and_maintain_workflow() {
    let bin = seed_bin();
    let tmp = tempdir().unwrap();
    let target = tmp.path().join("test-go-app");

    // 1. Create Go project
    let status = Command::new(&bin)
        .args([
            "new",
            target.to_str().unwrap(),
            "--template",
            "go",
            "--author",
            "CI Test",
            "--license",
            "MIT",
        ])
        .status()
        .unwrap();
    assert!(status.success());

    assert!(target.join("go.mod").exists());
    assert!(target.join("cmd/test-go-app/main.go").exists());
    assert!(target.join("README.md").exists());
    assert!(target.join("LICENSE").exists());

    // 2. Doctor / Check
    let check = Command::new(&bin)
        .args(["check", target.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(check.status.success());
    let check_out = String::from_utf8_lossy(&check.stdout);
    assert!(check_out.contains("Project Profile : go"));

    // 3. Tree
    let tree = Command::new(&bin)
        .args(["tree", target.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(tree.status.success());
    let tree_out = String::from_utf8_lossy(&tree.stdout);
    assert!(tree_out.contains("go.mod"));
    assert!(tree_out.contains("main.go"));
}

#[test]
fn test_workspace_init_does_not_create_fake_projects() {
    let tmp = tempdir().unwrap();
    let ws = tmp.path().join("CleanWorkspace");

    let cfg = seed::config::AppConfig {
        workspace_root: ws.to_string_lossy().to_string(),
        personal_dir: "personal".into(),
        clients_dir: "clients".into(),
        resources_dir: "resources".into(),
        archive_dir: "archive".into(),
        default_author: "Artist".into(),
        default_license: "MIT".into(),
        default_template: "yt-video".into(),
        custom_templates_dir: tmp.path().join("templates").to_string_lossy().to_string(),
        favorite_templates: Vec::new(),
        icon_mode: "ascii".into(),
    };

    // 1. Initialize master workspace
    let summary = cfg.init_master_workspace().unwrap();
    assert!(summary.created_dirs >= 4);

    // 2. CRITICAL LOGIC CHECK:
    // Right after init structure, there must be ZERO existing projects detected!
    let projects = seed::tui::scan_workspace_projects(&cfg);
    assert_eq!(
        projects.len(),
        0,
        "Initialized workspace structure must not show fake projects! Found: {:?}",
        projects.iter().map(|p| &p.name).collect::<Vec<_>>()
    );

    // 3. Now create an actual personal project
    let personal_proj = cfg.personal_path().join("my-yt-vlog");
    std::fs::create_dir_all(&personal_proj).unwrap();
    std::fs::write(personal_proj.join("README.md"), "# Vlog").unwrap();
    std::fs::write(personal_proj.join("script.md"), "Intro...").unwrap();

    // 4. Create an actual client project
    let client_proj = cfg.clients_path().join("MegaCorp").join("branding");
    std::fs::create_dir_all(&client_proj).unwrap();
    std::fs::write(client_proj.join("README.md"), "# Brand").unwrap();
    std::fs::create_dir_all(client_proj.join("deliverables")).unwrap();
    std::fs::write(client_proj.join("deliverables/logo.svg"), "<svg></svg>").unwrap();

    let scanned = seed::tui::scan_workspace_projects(&cfg);
    assert_eq!(scanned.len(), 2);

    let personal = scanned.iter().find(|p| p.name == "my-yt-vlog").unwrap();
    assert_eq!(personal.category, seed::tui::ProjectCategory::Personal);
    assert_eq!(personal.file_count, 2);

    let client = scanned.iter().find(|p| p.name == "branding").unwrap();
    assert_eq!(client.category, seed::tui::ProjectCategory::Client("MegaCorp".into()));
    assert_eq!(client.file_count, 2);

    // 5. Test archive client project preserves client association
    let archived = cfg.archive_project(&client_proj).unwrap();
    assert!(archived.exists());
    assert!(archived.to_string_lossy().contains("MegaCorp"));
    assert!(!client_proj.exists());
}

#[test]
fn test_cli_fav_templates() {
    let bin = seed_bin();
    let tmp = tempdir().unwrap();
    let config_path = tmp.path().join("config.toml");

    // 1. Initial list with no favorites
    let output = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .arg("fav")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No favorite templates pinned yet"));

    // 2. Pin 'podcast' as favorite
    let output = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .args(["fav", "podcast"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pinned 'podcast' to favorites"));

    // 3. Check list shows 'podcast' with [FAV]
    let output = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .arg("list")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(seed::icons::fav_badge(true, true)) && stdout.contains("podcast"));

    // 4. Check fav listing
    let output = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .arg("fav")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("podcast"));
    assert!(stdout.contains("[FAV]"));

    // 5. Toggle off 'podcast'
    let output = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .args(["fav", "podcast"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Removed 'podcast' from favorites"));
}

#[test]
fn test_cli_icons_command() {
    let bin = seed_bin();
    let tmp = tempfile::tempdir().unwrap();
    let config_path = tmp.path().join("config.toml");

    // 1. Check current icons mode (defaults to nerdfont now)
    let output = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .arg("icons")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Current mode: nerdfont") && stdout.contains("ACTIVE"));

    // 2. Switch to ascii
    let output = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .args(["icons", "ascii"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Icon mode set to 'ascii'"));

    // 3. Check status is now DISABLED
    let output = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .arg("icons")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Current mode: ascii") && stdout.contains("DISABLED"));

    // 4. Test tree with icons
    let test_proj = tmp.path().join("demo");
    std::fs::create_dir_all(&test_proj).unwrap();
    std::fs::write(test_proj.join("main.rs"), "fn main() {}").unwrap();

    let output = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .args(["tree", test_proj.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("main.rs"));

    // 5. Toggle icons back to nerdfont
    let output = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .args(["icons", "toggle"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Icon mode toggled to 'nerdfont'"));
}

#[test]
fn test_preset_and_manual_templates_feature() {
    let bin = seed_bin();
    let tmp = tempdir().unwrap();
    let config_path = tmp.path().join("config.toml");
    let ws_path = tmp.path().join("Workspace");

    // Initialize workspace
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .args(["workspace", ws_path.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(out.status.success());

    // 1. Test 'seed list' shows [Preset] badges
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .args(["list"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Built-in Preset Templates:"));
    assert!(stdout.contains("[Preset]"));

    // 2. Test 'seed list --preset'
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .args(["list", "--preset"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Built-in Preset Templates:"));
    assert!(!stdout.contains("Manual User Templates:"));

    // 3. Create a new manual template with 'seed template-new'
    let out = Command::new(&bin)
        .current_dir(tmp.path())
        .env("SEED_CONFIG", &config_path)
        .args([
            "template-new",
            "studio-workflow",
            "--description",
            "Custom audio & podcast studio setup",
            "--dirs",
            "raw_audio,edits,exports,stems",
            "--files",
            "README.md,session.json,notes.txt",
            "--outputs",
            "Master WAV,Podcast MP3",
            "--local",
        ])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Created manual template 'studio-workflow' successfully!"));

    // 4. Test 'seed list --manual' shows our new manual template
    let out = Command::new(&bin)
        .current_dir(tmp.path())
        .env("SEED_CONFIG", &config_path)
        .args(["list", "--manual"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("studio-workflow"));
    assert!(stdout.contains("[Manual]"));
    assert!(!stdout.contains("Built-in Preset Templates:"));

    // 5. Scaffold a project using the manual template
    let out = Command::new(&bin)
        .current_dir(tmp.path())
        .env("SEED_CONFIG", &config_path)
        .args(["new", "daily-show", "-t", "studio-workflow", "-c", "personal"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Creating project 'daily-show' using template 'studio-workflow'"));

    let project_dir = ws_path.join("personal/daily-show");
    assert!(project_dir.exists());
    assert!(project_dir.join("raw_audio").is_dir());
    assert!(project_dir.join("edits").is_dir());
    assert!(project_dir.join("exports").is_dir());
    assert!(project_dir.join("stems").is_dir());
    assert!(project_dir.join("README.md").is_file());
    assert!(project_dir.join("session.json").is_file());
    assert!(project_dir.join("notes.txt").is_file());

    // 6. Scaffold a project in manual blank mode with '--manual'
    let out = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .args(["new", "scratch-app", "-m", "-c", "personal"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let scratch_dir = ws_path.join("personal/scratch-app");
    assert!(scratch_dir.exists());
    assert!(scratch_dir.join("src").is_dir());
    assert!(scratch_dir.join("README.md").is_file());
}
