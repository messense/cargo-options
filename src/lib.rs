mod build;
mod check;
mod clippy;
mod common;
mod install;
mod metadata;
mod run;
mod rustc;
mod test;

pub mod heading {
    pub const PACKAGE_SELECTION: &str = "Package Selection";
    pub const TARGET_SELECTION: &str = "Target Selection";
    pub const FEATURE_SELECTION: &str = "Feature Selection";
    pub const COMPILATION_OPTIONS: &str = "Compilation Options";
    pub const MANIFEST_OPTIONS: &str = "Manifest Options";
}

pub mod style {
    use anstyle::*;

    pub const NOP: Style = Style::new();
    pub const HEADER: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
    pub const USAGE: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
    pub const LITERAL: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
    pub const PLACEHOLDER: Style = AnsiColor::Cyan.on_default();
    pub const ERROR: Style = AnsiColor::Red.on_default().effects(Effects::BOLD);
    pub const WARN: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);
    pub const NOTE: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
    pub const GOOD: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
    pub const VALID: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
    pub const INVALID: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);

    pub const STYLES: clap::builder::Styles = {
        clap::builder::styling::Styles::styled()
            .header(HEADER)
            .usage(USAGE)
            .literal(LITERAL)
            .placeholder(PLACEHOLDER)
            .error(ERROR)
            .valid(VALID)
            .invalid(INVALID)
    };
}

// Specify crate to satisfy naming overlap w/ rustc clippy
pub use crate::clippy::Clippy;
pub use build::Build;
pub use check::Check;
pub use common::CommonOptions;
pub use install::Install;
pub use metadata::Metadata;
pub use run::Run;
pub use rustc::Rustc;
pub use test::Test;
