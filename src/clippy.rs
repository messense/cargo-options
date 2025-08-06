use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::process::Command;

use clap::Parser;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::check::CheckOptions;
use crate::common::CommonOptions;
use crate::heading;

/// Checks a package to catch common mistakes and improve your Rust code
#[derive(Clone, Debug, Default, Parser)]
#[command(
    display_order = 1,
    after_help = "Run `cargo help clippy` for more detailed information."
)]
#[group(skip)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Clippy {
    #[command(flatten)]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common: CommonOptions,

    #[command(flatten)]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub check: CheckOptions,

    /// Path to Cargo.toml
    #[arg(long, value_name = "PATH", help_heading = heading::MANIFEST_OPTIONS)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub manifest_path: Option<PathBuf>,

    /// Build artifacts in release mode, with optimizations
    #[arg(short = 'r', long, help_heading = heading::COMPILATION_OPTIONS)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub release: bool,

    /// Ignore `rust-version` specification in packages
    #[arg(long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub ignore_rust_version: bool,

    /// Output build graph in JSON (unstable)
    #[arg(long, help_heading = heading::COMPILATION_OPTIONS)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit_graph: bool,

    /// Ignore dependencies, run only on crate
    #[arg(long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub no_deps: bool,

    /// Automatically apply lint suggestions (see `cargo help clippy`)
    #[arg(long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub fix: bool,

    /// Allow applying fixes even if the working directory has changes
    #[arg(long, hide = true)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub allow_dirty: bool,

    /// Allow applying fixes even if the index has changes
    #[arg(long, hide = true)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub allow_staged: bool,

    /// Arguments passed to rustc.
    #[arg(value_name = "args", trailing_var_arg = true, num_args = 0..)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub args: Vec<String>,
}

impl Clippy {
    /// Build a `cargo clippy` command
    pub fn command(&self) -> Command {
        let mut cmd = CommonOptions::cargo_command();
        cmd.arg("clippy");

        self.common.apply(&mut cmd);
        self.check.apply(&mut cmd);

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
        if self.no_deps {
            cmd.arg("--no-deps");
        }
        if self.fix {
            cmd.arg("--fix");
        }
        if self.allow_dirty {
            cmd.arg("--allow-dirty");
        }
        if self.allow_staged {
            cmd.arg("--allow-staged");
        }
        if !self.args.is_empty() {
            cmd.arg("--");
            cmd.args(&self.args);
        }

        cmd
    }
}

impl Deref for Clippy {
    type Target = CommonOptions;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for Clippy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

#[cfg(test)]
mod test {
    use super::Clippy;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        <Clippy as CommandFactory>::command().debug_assert()
    }
}
