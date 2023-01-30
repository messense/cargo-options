use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::process::Command;

use clap::{ArgAction, Parser};

use crate::common::CommonOptions;

/// `cargo check` options which are also a subset of `cargo clippy`
#[derive(Clone, Debug, Default, Parser)]
pub struct CheckOptions {
    /// Package to build (see `cargo help pkgid`)
    #[arg(
        short = 'p',
        long = "package",
        value_name = "SPEC",
        action = ArgAction::Append,
        num_args=0..=1,
    )]
    pub packages: Vec<String>,

    /// Check all packages in the workspace
    #[arg(long)]
    pub workspace: bool,

    /// Exclude packages from the build
    #[arg(long, value_name = "SPEC", action = ArgAction::Append)]
    pub exclude: Vec<String>,

    /// Alias for workspace (deprecated)
    #[arg(long)]
    pub all: bool,

    /// Check only this package's library
    #[arg(long)]
    pub lib: bool,

    /// Check only the specified binary
    #[arg(long, value_name = "NAME", action = ArgAction::Append, num_args=0..=1)]
    pub bin: Vec<String>,

    /// Check all binaries
    #[arg(long)]
    pub bins: bool,

    /// Check only the specified example
    #[arg(long, value_name = "NAME", action = ArgAction::Append, num_args=0..=1)]
    pub example: Vec<String>,

    /// Check all examples
    #[arg(long)]
    pub examples: bool,

    /// Check only the specified test target
    #[arg(long, value_name = "NAME", action = ArgAction::Append)]
    pub test: Vec<String>,

    /// Check all tests
    #[arg(long)]
    pub tests: bool,

    /// Check only the specified bench target
    #[arg(long, value_name = "NAME", action = ArgAction::Append)]
    pub bench: Vec<String>,

    /// Check all benches
    #[arg(long)]
    pub benches: bool,

    /// Check all targets
    #[arg(long)]
    pub all_targets: bool,

    /// Outputs a future incompatibility report at the end of the build (unstable)
    #[arg(long)]
    pub future_incompat_report: bool,
}

impl CheckOptions {
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
        for test in &self.test {
            cmd.arg("--test").arg(test);
        }
        if self.tests {
            cmd.arg("--tests");
        }
        for bench in &self.bench {
            cmd.arg("--bench").arg(bench);
        }
        if self.benches {
            cmd.arg("--benches");
        }
        if self.all_targets {
            cmd.arg("--all-targets");
        }
        if self.future_incompat_report {
            cmd.arg("--future-incompat-report");
        }
    }
}

/// Check a local package and all of its dependencies for errors
#[derive(Clone, Debug, Default, Parser)]
#[command(
    display_order = 1,
    after_help = "Run `cargo help check` for more detailed information."
)]
#[group(skip)]
pub struct Check {
    #[command(flatten)]
    pub common: CommonOptions,

    #[command(flatten)]
    pub check: CheckOptions,

    /// Path to Cargo.toml
    #[arg(long, value_name = "PATH")]
    pub manifest_path: Option<PathBuf>,

    /// Build artifacts in release mode, with optimizations
    #[arg(short = 'r', long)]
    pub release: bool,

    /// Ignore `rust-version` specification in packages
    #[arg(long)]
    pub ignore_rust_version: bool,

    /// Output build graph in JSON (unstable)
    #[arg(long)]
    pub unit_graph: bool,
}

impl Check {
    /// Build a `cargo check` command
    pub fn command(&self) -> Command {
        let mut cmd = CommonOptions::cargo_command();
        cmd.arg("check");

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

        cmd
    }
}

impl Deref for Check {
    type Target = CommonOptions;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for Check {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

#[cfg(test)]
mod test {
    use super::Check;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        <Check as CommandFactory>::command().debug_assert()
    }
}
