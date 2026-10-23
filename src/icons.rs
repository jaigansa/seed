//! Pure Rust icon mapping system supporting clean ASCII and Nerd Font glyphs.
//! Strictly zero Unicode emojis and zero C/FFI dependencies.

use std::path::Path;

#[allow(dead_code)]
pub mod nerdfont {
    // Folders
    pub const FOLDER: &str = "\u{f07b} ";      // 
    pub const FOLDER_OPEN: &str = "\u{f07c} "; // 

    // System & Meta
    pub const GIT: &str = "\u{f1d3} ";         // 
    pub const FAV: &str = "\u{f005} ";         //  (Star)
    pub const CHECK: &str = "\u{f00c} ";       // 
    pub const WARN: &str = "\u{f071} ";        // 
    pub const INFO: &str = "\u{f05a} ";        // 
    pub const ARCHIVE: &str = "\u{f187} ";     // 

    // Languages & Frameworks
    pub const RUST: &str = "\u{e7a8} ";        // 
    pub const PYTHON: &str = "\u{e73c} ";      // 
    pub const TYPESCRIPT: &str = "\u{e628} ";  // 
    pub const JAVASCRIPT: &str = "\u{e74e} ";  // 
    pub const REACT: &str = "\u{e7ba} ";       // 
    pub const GO: &str = "\u{e627} ";          // 
    pub const CPP: &str = "\u{e61d} ";         // 
    pub const C: &str = "\u{e61e} ";           // 
    pub const JAVA: &str = "\u{e738} ";        // 
    pub const SHELL: &str = "\u{f489} ";       // 
    pub const DOCKER: &str = "\u{f308} ";      // 
    pub const HTML: &str = "\u{e736} ";        // 
    pub const CSS: &str = "\u{e749} ";         // 

    // Content, Media & Creative
    pub const VIDEO: &str = "\u{f03d} ";       // 
    pub const AUDIO: &str = "\u{f001} ";       // 
    pub const PODCAST: &str = "\u{f130} ";     // 
    pub const BOOK: &str = "\u{f02d} ";        // 
    pub const GAME: &str = "\u{f11b} ";        // 
    pub const CUBE: &str = "\u{f1b2} ";        // 
    pub const DESIGN: &str = "\u{f040} ";      // 
    pub const PRESENTATION: &str = "\u{f108} ";// 
    pub const DOCUMENT: &str = "\u{f15c} ";    // 
    pub const CONFIG: &str = "\u{e60b} ";      // 
    pub const FILE: &str = "\u{f15b} ";        // 
}

#[allow(dead_code)]
pub mod ascii {
    pub const FOLDER: &str = "+ ";
    pub const FOLDER_OPEN: &str = "+ ";
    pub const GIT: &str = "[git] ";
    pub const FAV: &str = "[FAV] ";
    pub const CHECK: &str = "[OK] ";
    pub const WARN: &str = "[!] ";
    pub const INFO: &str = "[i] ";
    pub const ARCHIVE: &str = "[ARC] ";
    pub const FILE: &str = "- ";
}

/// Returns an icon for a template based on its name (disabled: template category icons are unnecessary)
#[allow(dead_code)]
pub fn template_icon(_name: &str, _nerd_font: bool) -> &'static str {
    ""
}

/// Returns an icon for a project stack based on its detected type
pub fn project_icon(detected_type: &str, nerd_font: bool) -> &'static str {
    if !nerd_font {
        return "";
    }

    match detected_type.to_lowercase().as_str() {
        "rust" => nerdfont::RUST,
        "python" | "fastapi" | "data-science" => nerdfont::PYTHON,
        "react" => nerdfont::REACT,
        "typescript" | "node" => nerdfont::TYPESCRIPT,
        "javascript" | "web" => nerdfont::JAVASCRIPT,
        "go" => nerdfont::GO,
        "c++" | "cpp" => nerdfont::CPP,
        "c" => nerdfont::C,
        "java" => nerdfont::JAVA,
        "bash" | "shell" => nerdfont::SHELL,
        "docker" => nerdfont::DOCKER,
        "podcast" => nerdfont::PODCAST,
        "music" => nerdfont::AUDIO,
        "video" | "yt-video" => nerdfont::VIDEO,
        "book" => nerdfont::BOOK,
        "game" => nerdfont::GAME,
        "cad" | "3d" => nerdfont::CUBE,
        _ => nerdfont::FOLDER,
    }
}

/// Returns an icon for a file based on its extension
pub fn file_icon(path: &Path, nerd_font: bool) -> &'static str {
    if !nerd_font {
        return ascii::FILE;
    }

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "rs" => nerdfont::RUST,
        "py" => nerdfont::PYTHON,
        "ts" | "tsx" => nerdfont::TYPESCRIPT,
        "js" | "jsx" => nerdfont::JAVASCRIPT,
        "go" => nerdfont::GO,
        "cpp" | "hpp" | "cc" | "cxx" => nerdfont::CPP,
        "c" | "h" => nerdfont::C,
        "java" => nerdfont::JAVA,
        "sh" | "bash" | "zsh" => nerdfont::SHELL,
        "html" | "htm" => nerdfont::HTML,
        "css" | "scss" | "sass" => nerdfont::CSS,
        "md" | "txt" | "tex" | "bib" => nerdfont::DOCUMENT,
        "toml" | "yaml" | "yml" | "json" | "xml" | "ini" => nerdfont::CONFIG,
        "svg" | "png" | "jpg" | "jpeg" | "webp" | "gif" => nerdfont::DESIGN,
        "mp3" | "wav" | "flac" | "ogg" | "mid" | "midi" => nerdfont::AUDIO,
        "mp4" | "mkv" | "mov" | "avi" => nerdfont::VIDEO,
        "blend" | "stl" | "step" | "3mf" | "dxf" => nerdfont::CUBE,
        "dockerfile" => nerdfont::DOCKER,
        _ => nerdfont::FILE,
    }
}

/// Returns a folder icon
pub fn folder_icon(nerd_font: bool) -> &'static str {
    if nerd_font {
        nerdfont::FOLDER
    } else {
        ascii::FOLDER
    }
}

/// Returns a favorite badge indicator
pub fn fav_badge(is_fav: bool, nerd_font: bool) -> &'static str {
    if !is_fav {
        ""
    } else if nerd_font {
        nerdfont::FAV
    } else {
        ascii::FAV
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_resolutions() {
        // Template category icons are disabled (unnecessary)
        assert_eq!(template_icon("rust", false), "");
        assert_eq!(template_icon("rust", true), "");
        assert_eq!(template_icon("podcast", true), "");
        assert_eq!(file_icon(Path::new("main.rs"), false), "- ");
        assert_eq!(file_icon(Path::new("main.rs"), true), nerdfont::RUST);
        assert_eq!(file_icon(Path::new("app.py"), true), nerdfont::PYTHON);
        assert_eq!(file_icon(Path::new("config.toml"), true), nerdfont::CONFIG);
        assert_eq!(folder_icon(true), nerdfont::FOLDER);
        assert_eq!(fav_badge(true, true), nerdfont::FAV);
    }
}
