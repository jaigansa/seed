use std::fs;
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
    assert!(stdout.contains("3dmodel"));
    assert!(stdout.contains("cad"));
    assert!(stdout.contains("logo"));
    assert!(stdout.contains("video-edit"));
    assert!(stdout.contains("photo-edit"));
    assert!(stdout.contains("vector-artwork"));
    assert!(stdout.contains("webdesign"));
    assert!(stdout.contains("coding"));
    assert!(stdout.contains("Deliverables:"));
}

#[test]
fn test_cli_scaffold_output_related_templates() {
    let bin = seed_bin();
    let tmp = tempdir().unwrap();

    // 1. Test 3D Model scaffold
    let model_target = tmp.path().join("my-hero-character");
    let status = Command::new(&bin)
        .args(["new", model_target.to_str().unwrap(), "-t", "3dmodel", "-a", "Artist", "--client", "GameStudio"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(model_target.join("models/specs.md").exists());
    assert!(model_target.join("README.md").exists());
    let readme = fs::read_to_string(model_target.join("README.md")).unwrap();
    assert!(readme.contains("Task List") && readme.contains("GameStudio"));

    // 2. Test CAD scaffold
    let cad_target = tmp.path().join("robot-arm-joint");
    let status = Command::new(&bin)
        .args(["new", cad_target.to_str().unwrap(), "-t", "cad", "-a", "Engineer", "--client", "RoboticsCorp"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(cad_target.join("bom/bill_of_materials.md").exists());
    assert!(cad_target.join("specs/tolerances.md").exists());
    let readme = fs::read_to_string(cad_target.join("README.md")).unwrap();
    assert!(readme.contains("Task List") && readme.contains("RoboticsCorp"));

    // 3. Test Logo scaffold
    let logo_target = tmp.path().join("brand-identity-suite");
    let status = Command::new(&bin)
        .args(["new", logo_target.to_str().unwrap(), "-t", "logo", "-a", "Designer", "--client", "ApexMedia"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(logo_target.join("guidelines/brand_guidelines.md").exists());
    assert!(logo_target.join("source/color_palettes/palette.md").exists());
    let readme = fs::read_to_string(logo_target.join("README.md")).unwrap();
    assert!(readme.contains("ApexMedia") && readme.contains("Task List"));

    // 4. Test Video Edit scaffold
    let video_target = tmp.path().join("product-commercial");
    let status = Command::new(&bin)
        .args(["new", video_target.to_str().unwrap(), "-t", "video-edit", "-a", "Editor", "--client", "TechCo"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(video_target.join("projects/timeline/project_notes.md").exists());
    assert!(video_target.join("transcripts/script_and_cue.md").exists());
    let readme = fs::read_to_string(video_target.join("README.md")).unwrap();
    assert!(readme.contains("TechCo") && readme.contains("Task List"));
}

#[test]
fn test_cli_scaffold_regular_templates() {
    let bin = seed_bin();
    let tmp = tempdir().unwrap();

    // 1. Test Photo Edit scaffold
    let photo_target = tmp.path().join("studio-portrait");
    let status = Command::new(&bin)
        .args(["new", photo_target.to_str().unwrap(), "-t", "photo-edit", "-a", "Photographer", "--client", "FashionMag"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(photo_target.join("metadata/shot_list.md").exists());
    let readme = fs::read_to_string(photo_target.join("README.md")).unwrap();
    assert!(readme.contains("FashionMag") && readme.contains("Task List"));

    // 2. Test Vector Artwork scaffold
    let vector_target = tmp.path().join("icon-system");
    let status = Command::new(&bin)
        .args(["new", vector_target.to_str().unwrap(), "-t", "vector-artwork", "-a", "Illustrator", "--client", "AppCompany"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(vector_target.join("source/components/design_tokens.md").exists());
    let readme = fs::read_to_string(vector_target.join("README.md")).unwrap();
    assert!(readme.contains("AppCompany") && readme.contains("Task List"));

    // 3. Test Web Design scaffold
    let web_target = tmp.path().join("saas-landing-page");
    let status = Command::new(&bin)
        .args(["new", web_target.to_str().unwrap(), "-t", "webdesign", "-a", "UIUX", "--client", "SaaSStartup"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(web_target.join("prototype/src/index.html").exists());
    assert!(web_target.join("prototype/src/styles.css").exists());
    let readme = fs::read_to_string(web_target.join("README.md")).unwrap();
    assert!(readme.contains("SaaSStartup") && readme.contains("Task List"));

    // 4. Test Coding scaffold
    let code_target = tmp.path().join("core-backend-api");
    let status = Command::new(&bin)
        .args(["new", code_target.to_str().unwrap(), "-t", "coding", "-a", "Developer", "--client", "CloudCorp"])
        .status()
        .unwrap();
    assert!(status.success());
    assert!(code_target.join("docs/architecture/overview.md").exists());
    assert!(code_target.join("scripts/build.sh").exists());
    let readme = fs::read_to_string(code_target.join("README.md")).unwrap();
    assert!(readme.contains("CloudCorp") && readme.contains("Task List"));
}

#[test]
fn test_cli_scaffold_and_maintain_workflow() {
    let bin = seed_bin();
    let tmp = tempdir().unwrap();
    let target = tmp.path().join("test-coding-app");

    // 1. Create coding project
    let status = Command::new(&bin)
        .args([
            "new",
            target.to_str().unwrap(),
            "--template",
            "coding",
            "--author",
            "CI Test",
            "--license",
            "MIT",
        ])
        .status()
        .unwrap();
    assert!(status.success());

    assert!(target.join("src/main.rs").exists());
    assert!(target.join("docs/architecture/overview.md").exists());
    assert!(target.join("scripts/build.sh").exists());
    assert!(target.join("README.md").exists());
    assert!(target.join("LICENSE").exists());

    // 2. Doctor / Check
    let check = Command::new(&bin)
        .args(["check", target.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(check.status.success());
    let check_out = String::from_utf8_lossy(&check.stdout);
    assert!(check_out.contains("Project Profile : coding"));

    // 3. Tree
    let tree = Command::new(&bin)
        .args(["tree", target.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(tree.status.success());
    let tree_out = String::from_utf8_lossy(&tree.stdout);
    assert!(tree_out.contains("main.rs"));
    assert!(tree_out.contains("build.sh"));
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
        default_template: "video-edit".into(),
        custom_templates_dir: tmp.path().join("templates").to_string_lossy().to_string(),
        favorite_templates: Vec::new(),
        pinned_projects: Vec::new(),
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

    // 2. Pin '3dmodel' as favorite
    let output = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .args(["fav", "3dmodel"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pinned '3dmodel' to favorites"));

    // 3. Check list shows '3dmodel' with [FAV]
    let output = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .arg("list")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(seed::icons::fav_badge(true, true)) && stdout.contains("3dmodel"));

    // 4. Check fav listing
    let output = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .arg("fav")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("3dmodel"));
    assert!(stdout.contains("[FAV]"));

    // 5. Toggle off '3dmodel'
    let output = Command::new(&bin)
        .env("SEED_CONFIG", &config_path)
        .args(["fav", "3dmodel"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Removed '3dmodel' from favorites"));
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
