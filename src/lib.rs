//! Seed: Project directory & file creator and maintainer engine

pub mod capture;
pub mod cli;
pub mod completions;
pub mod config;
pub mod creator;
pub mod icons;
pub mod maintainer;
pub mod template;
pub mod tree;
pub mod tui;
pub mod utils;
pub mod wizard;

pub use creator::{create_project, CreateOptions};
pub use maintainer::{audit_project, sync_project, AuditReport, SyncOptions};
pub use template::Template;
