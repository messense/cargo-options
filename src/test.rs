use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::process::Command;

use clap::{ArgAction, Parser};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::common::CommonOptions;
use crate::heading;

/// Execute all unit and integration tests and build examples of a local package
#[derive(Clone, Debug, Default, Parser)]
#[command(
    display_order = 1,
    after_help = "Run `cargo help test` for more detailed information.\nRun `cargo test -- --help` for test binary options."
)]
#[group(skip)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Test {
    #[command(flatten)]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common: CommonOptions,

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

    /// Package to build (see `cargo help pkgid`)
    #[arg(
        short = 'p',
        long = "package",
        value_name = "SPEC",
        action = ArgAction::Append,
        num_args=0..=1,
        help_heading = heading::PACKAGE_SELECTION,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub packages: Vec<String>,

    /// Test all packages in the workspace
    #[arg(long, help_heading = heading::PACKAGE_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub workspace: bool,

    /// Exclude packages from the build
    #[arg(
        long,
        value_name = "SPEC",
        action = ArgAction::Append,
        help_heading = heading::PACKAGE_SELECTION,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub exclude: Vec<String>,

    /// Alias for workspace (deprecated)
    #[arg(long, help_heading = heading::PACKAGE_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub all: bool,

    /// Test only this package's library
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub lib: bool,

    /// Test only the specified binary
    #[arg(
        long,
        value_name = "NAME",
        action = ArgAction::Append,
        num_args=0..=1,
        help_heading = heading::TARGET_SELECTION,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub bin: Vec<String>,

    /// Test all binaries
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub bins: bool,

    /// Test only the specified example
    #[arg(
        long,
        value_name = "NAME",
        action = ArgAction::Append,
        num_args=0..=1,
        help_heading = heading::TARGET_SELECTION,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub example: Vec<String>,

    /// Test all examples
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: bool,

    /// Test only the specified test target
    #[arg(
        long,
        value_name = "NAME",
        action = ArgAction::Append,
        help_heading = heading::TARGET_SELECTION,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub test: Vec<String>,

    /// Test all tests
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub tests: bool,

    /// Test only the specified bench target
    #[arg(
        long,
        value_name = "NAME",
        action = ArgAction::Append,
        help_heading = heading::TARGET_SELECTION,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub bench: Vec<String>,

    /// Test all benches
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub benches: bool,

    /// Test all targets
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub all_targets: bool,

    /// Test only this library's documentation
    #[arg(long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub doc: bool,

    /// Compile, but don't run tests
    #[arg(long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub no_run: bool,

    /// Run all tests regardless of failure
    #[arg(long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub no_fail_fast: bool,

    /// Outputs a future incompatibility report at the end of the build (unstable)
    #[arg(long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub future_incompat_report: bool,

    /// If specified, only run tests containing this string in their names
    #[arg(value_name = "TESTNAME")]
    #[cfg_attr(feature = "serde", serde(default))]
    pub test_name: Option<String>,

    /// Arguments for the test binary
    #[arg(value_name = "args", trailing_var_arg = true, num_args = 0..)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub args: Vec<String>,
}

impl Test {
    /// Build a `cargo test` command
    pub fn command(&self) -> Command {
        let mut cmd = CommonOptions::cargo_command();
        cmd.arg("test");

        self.common.apply(&mut cmd);

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
        if self.doc {
            cmd.arg("--doc");
        }
        if self.no_run {
            cmd.arg("--no-run");
        }
        if self.no_fail_fast {
            cmd.arg("--no-fail-fast");
        }
        if self.future_incompat_report {
            cmd.arg("--future-incompat-report");
        }
        cmd.arg("--");
        if let Some(test_name) = self.test_name.as_ref() {
            cmd.arg(test_name);
        }
        cmd.args(&self.args);

        cmd
    }
}

impl Deref for Test {
    type Target = CommonOptions;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for Test {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

#[cfg(test)]
mod tests {
    use super::Test;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        <Test as CommandFactory>::command().debug_assert()
    }
}
