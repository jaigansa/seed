use anyhow::{Context, Result};
use chrono::Datelike;
use colored::Colorize;
use std::path::Path;
use std::process::Command;

pub fn current_year() -> i32 {
    chrono::Utc::now().year()
}

pub fn get_git_user() -> Option<String> {
    let output = Command::new("git")
        .args(["config", "--get", "user.name"])
        .output()
        .ok()?;

    if output.status.success() {
        let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !name.is_empty() {
            return Some(name);
        }
    }
    None
}

pub fn init_git_repo(path: &Path) -> Result<()> {
    let output = Command::new("git")
        .arg("init")
        .current_dir(path)
        .output()
        .context("Failed to run 'git init'")?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git init failed: {}", err);
    }
    Ok(())
}

pub fn get_license_text(license: &str, author: &str, year: i32) -> String {
    match license.to_uppercase().as_str() {
        "MIT" => format!(
            r#"MIT License

Copyright (c) {year} {author}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"#
        ),
        "APACHE-2.0" | "APACHE" => format!(
            r#"Apache License
Version 2.0, January 2004
http://www.apache.org/licenses/

Copyright {year} {author}

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
"#
        ),
        "BSD-3-CLAUSE" | "BSD-3" | "BSD" => format!(
            r#"BSD 3-Clause License

Copyright (c) {year}, {author}
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
"#
        ),
        "UNLICENSE" => r#"This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>
"#.to_string(),
        _ => format!(
            r#"Copyright (c) {year} {author}. All rights reserved.
License: {license}
"#
        ),
    }
}

pub fn print_success(msg: &str) {
    println!("{} {}", "[OK]".green().bold(), msg);
}

pub fn print_info(msg: &str) {
    println!("{} {}", "[INFO]".cyan().bold(), msg);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitRepoStatus {
    NotRepo,
    Clean,
    Modified,
}

pub fn check_git_status(path: &Path) -> GitRepoStatus {
    if !path.join(".git").exists() {
        return GitRepoStatus::NotRepo;
    }
    match Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(path)
        .output()
    {
        Ok(output) if output.status.success() => {
            let s = String::from_utf8_lossy(&output.stdout);
            if s.trim().is_empty() {
                GitRepoStatus::Clean
            } else {
                GitRepoStatus::Modified
            }
        }
        _ => GitRepoStatus::Clean,
    }
}

pub fn find_in_path(cmd: &str) -> bool {
    if let Some(paths) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&paths) {
            let full = dir.join(cmd);
            if full.is_file() {
                return true;
            }
        }
    }
    false
}

pub fn resolve_editor() -> Option<String> {
    if let Ok(editor) = std::env::var("VISUAL") {
        let trimmed = editor.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }
    if let Ok(editor) = std::env::var("EDITOR") {
        let trimmed = editor.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }
    for candidate in ["code", "cursor", "zed", "nvim", "vim", "nano"] {
        if find_in_path(candidate) {
            return Some(candidate.to_string());
        }
    }
    None
}

pub fn is_gui_editor(cmd: &str) -> bool {
    let lower = cmd.to_lowercase();
    let bin_name = Path::new(&lower)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&lower);
    bin_name.starts_with("code")
        || bin_name.starts_with("cursor")
        || bin_name.starts_with("zed")
        || bin_name.starts_with("subl")
        || bin_name.starts_with("atom")
        || bin_name.starts_with("gedit")
        || bin_name.starts_with("kate")
}

pub fn launch_gui_editor(editor: &str, path: &Path) -> Result<()> {
    let parts: Vec<&str> = editor.split_whitespace().collect();
    if parts.is_empty() {
        anyhow::bail!("Editor command is empty");
    }
    let mut cmd = Command::new(parts[0]);
    if parts.len() > 1 {
        cmd.args(&parts[1..]);
    }
    cmd.arg(path);
    cmd.spawn()
        .with_context(|| format!("Failed to spawn GUI editor '{}'", editor))?;
    Ok(())
}

pub fn open_in_terminal_shell(path: &Path) -> Result<()> {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    print!("\x1B[2J\x1B[1;1H");
    println!("{}", "======================================================================".cyan());
    println!("  SEED Shell: Dropping into project directory");
    println!("  Path: {}", path.display().to_string().bold());
    println!("  Type 'exit' or press Ctrl+D to return to Seed");
    println!("{}", "======================================================================".cyan());

    let _ = Command::new(&shell)
        .current_dir(path)
        .status();
    Ok(())
}

pub fn get_creation_time(path: &Path) -> Option<std::time::SystemTime> {
    if let Ok(meta) = path.metadata() {
        if let Ok(created) = meta.created() {
            return Some(created);
        }
        if let Ok(modified) = meta.modified() {
            return Some(modified);
        }
    }
    None
}

pub fn format_date_time(time: std::time::SystemTime) -> String {
    let dt: chrono::DateTime<chrono::Local> = time.into();
    dt.format("%b %d, %Y (%H:%M)").to_string()
}

pub fn format_relative_time(time: std::time::SystemTime) -> String {
    let now = std::time::SystemTime::now();
    match now.duration_since(time) {
        Ok(dur) => {
            let secs = dur.as_secs();
            if secs < 60 {
                "Just now".to_string()
            } else if secs < 3600 {
                let mins = secs / 60;
                format!("{}m ago", mins)
            } else if secs < 86400 {
                let hours = secs / 3600;
                format!("{}h ago", hours)
            } else {
                let days = secs / 86400;
                if days < 30 {
                    format!("{}d ago", days)
                } else {
                    let months = days / 30;
                    format!("{}mo ago", months)
                }
            }
        }
        Err(_) => "Recent".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_relative_time() {
        let now = std::time::SystemTime::now();
        assert_eq!(format_relative_time(now), "Just now");

        let one_hour_ago = now - std::time::Duration::from_secs(3660);
        assert_eq!(format_relative_time(one_hour_ago), "1h ago");
    }

    #[test]
    fn test_format_date_time() {
        let now = std::time::SystemTime::now();
        let formatted = format_date_time(now);
        assert!(!formatted.is_empty());
        assert!(formatted.contains('(') && formatted.contains(')'));
    }

    #[test]
    fn test_is_gui_editor() {
        assert!(is_gui_editor("code"));
        assert!(is_gui_editor("/usr/bin/code"));
        assert!(is_gui_editor("cursor"));
        assert!(!is_gui_editor("nvim"));
        assert!(!is_gui_editor("vim"));
        assert!(!is_gui_editor("nano"));
    }
}
