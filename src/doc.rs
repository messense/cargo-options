use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::process::Command;

use clap::{ArgAction, Parser};

use crate::common::CommonOptions;
use crate::heading;

/// `cargo doc` options
#[derive(Clone, Debug, Default, Parser)]
pub struct DocOptions {
    /// Package to document
    #[arg(
        short = 'p',
        long = "package",
        value_name = "SPEC",
        action = ArgAction::Append,
        num_args=0..=1,
        help_heading = heading::PACKAGE_SELECTION,
    )]
    pub packages: Vec<String>,

    /// Document all packages in the workspace
    #[arg(long, help_heading = heading::PACKAGE_SELECTION)]
    pub workspace: bool,

    /// Exclude packages from the build
    #[arg(
        long,
        value_name = "SPEC",
        action = ArgAction::Append,
        help_heading = heading::PACKAGE_SELECTION,
    )]
    pub exclude: Vec<String>,

    /// Alias for --workspace (deprecated)
    #[arg(long, help_heading = heading::PACKAGE_SELECTION,)]
    pub all: bool,

    /// Document only this package's library
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    pub lib: bool,

    /// Document only the specified binary
    #[arg(
        long,
        value_name = "NAME",
        action = ArgAction::Append,
        num_args=0..=1,
        help_heading = heading::TARGET_SELECTION,
    )]
    pub bin: Vec<String>,

    /// Document all binaries
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    pub bins: bool,

    /// Document only the specified example
    #[arg(
        long,
        value_name = "NAME",
        action = ArgAction::Append,
        num_args=0..=1,
        help_heading = heading::TARGET_SELECTION,
    )]
    pub example: Vec<String>,

    /// Document all examples
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    pub examples: bool,

    /// Don't build documentation for dependencies
    #[arg(long)]
    pub no_deps: bool,

    /// Document private items
    #[arg(long)]
    pub document_private_items: bool,

    /// Opens the docs in a browser after the operation
    #[arg(long)]
    pub open: bool,
}

impl DocOptions {
    pub fn apply(&self, cmd: &mut Command) {
        for pkg in &self.packages {
            cmd.arg("--package").arg(pkg);
        }
        if self.workspace {
            cmd.arg("--workspace");
        }
        for item in &self.exclude {
            cmd.arg("--exclude").arg(item);
        }
        if self.all {
            cmd.arg("--all");
        }
        if self.lib {
            cmd.arg("--lib");
        }
        for bin in &self.bin {
            cmd.arg("--bin").arg(bin);
        }
        if self.bins {
            cmd.arg("--bins");
        }
        for example in &self.example {
            cmd.arg("--example").arg(example);
        }
        if self.examples {
            cmd.arg("--examples");
        }
        if self.no_deps {
            cmd.arg("--no-deps");
        }
        if self.document_private_items {
            cmd.arg("--document-private-items");
        }
        if self.open {
            cmd.arg("--open");
        }
    }
}

/// Build a package's documentation
#[derive(Clone, Debug, Default, Parser)]
#[command(
    display_order = 1,
    after_help = "Run `cargo help doc` for more detailed information."
)]
#[group(skip)]
pub struct Doc {
    #[command(flatten)]
    pub common: CommonOptions,

    #[command(flatten)]
    pub doc: DocOptions,

    /// Path to Cargo.toml
    #[arg(long, value_name = "PATH", help_heading = heading::MANIFEST_OPTIONS)]
    pub manifest_path: Option<PathBuf>,

    /// Build artifacts in release mode, with optimizations
    #[arg(short = 'r', long, help_heading = heading::COMPILATION_OPTIONS)]
    pub release: bool,

    /// Ignore `rust-version` specification in packages
    #[arg(long)]
    pub ignore_rust_version: bool,

    /// Output build graph in JSON (unstable)
    #[arg(long, help_heading = heading::COMPILATION_OPTIONS)]
    pub unit_graph: bool,
}

impl Doc {
    /// Build a `cargo doc` command
    pub fn command(&self) -> Command {
        let mut cmd = CommonOptions::cargo_command();
        cmd.arg("doc");

        self.common.apply(&mut cmd);
        self.doc.apply(&mut cmd);

        if let Some(path) = self.manifest_path.as_ref() {
            cmd.arg("--manifest-path").arg(path);
        }
        if self.release {
            cmd.arg("--release");
        }
        if self.ignore_rust_version {
            cmd.arg("--ignore-rust-version");
        }
        if self.unit_graph {
            cmd.arg("--unit-graph");
        }

        cmd
    }
}

impl Deref for Doc {
    type Target = CommonOptions;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for Doc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

#[cfg(test)]
mod test {
    use super::Doc;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        <Doc as CommandFactory>::command().debug_assert()
    }
}
