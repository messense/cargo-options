use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::process::Command;

use clap::{ArgAction, Parser};

use crate::common::CommonOptions;
use crate::heading;

/// Compile a package, and pass extra options to the compiler
#[derive(Clone, Debug, Default, Parser)]
#[command(
    display_order = 1,
    after_help = "Run `cargo help rustc` for more detailed information."
)]
#[group(skip)]
pub struct Rustc {
    #[command(flatten)]
    pub common: CommonOptions,

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

    /// Package to build (see `cargo help pkgid`)
    #[arg(
        short = 'p',
        long = "package",
        value_name = "SPEC",
        action = ArgAction::Append,
        num_args=0..=1,
        help_heading = heading::PACKAGE_SELECTION,
    )]
    pub packages: Vec<String>,

    /// Build only this package's library
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    pub lib: bool,

    /// Build only the specified binary
    #[arg(
        long,
        value_name = "NAME",
        action = ArgAction::Append,
        num_args=0..=1,
        help_heading = heading::TARGET_SELECTION,
    )]
    pub bin: Vec<String>,

    /// Build all binaries
    #[arg(long)]
    pub bins: bool,

    /// Build only the specified example
    #[arg(
        long,
        value_name = "NAME",
        action = ArgAction::Append,
        num_args=0..=1,
        help_heading = heading::TARGET_SELECTION,
    )]
    pub example: Vec<String>,

    /// Build all examples
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    pub examples: bool,

    /// Build only the specified test target
    #[arg(
        long,
        value_name = "NAME",
        action = ArgAction::Append,
        help_heading = heading::TARGET_SELECTION,
    )]
    pub test: Vec<String>,

    /// Build all tests
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    pub tests: bool,

    /// Build only the specified bench target
    #[arg(
        long,
        value_name = "NAME",
        action = ArgAction::Append,
        help_heading = heading::TARGET_SELECTION,
    )]
    pub bench: Vec<String>,

    /// Build all benches
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    pub benches: bool,

    /// Build all targets
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    pub all_targets: bool,

    /// Output compiler information without compiling
    #[arg(long, value_name = "INFO")]
    pub print: Option<String>,

    /// Comma separated list of types of crates for the compiler to emit
    #[arg(long, value_name = "CRATE-TYPE", action = ArgAction::Append)]
    pub crate_type: Vec<String>,

    /// Outputs a future incompatibility report at the end of the build (unstable)
    #[arg(long)]
    pub future_incompat_report: bool,

    /// Rustc flags
    #[arg(value_name = "args", trailing_var_arg = true, num_args = 0..)]
    pub args: Vec<String>,
}

impl Rustc {
    /// Build a `cargo rustc` command
    pub fn command(&self) -> Command {
        let mut cmd = CommonOptions::cargo_command();
        cmd.arg("rustc");

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
        if let Some(print) = self.print.as_ref() {
            cmd.arg("--print").arg(print);
        }
        if !self.crate_type.is_empty() {
            cmd.arg("--crate-type").arg(self.crate_type.join(","));
        }
        if self.future_incompat_report {
            cmd.arg("--future-incompat-report");
        }
        if !self.args.is_empty() {
            cmd.arg("--").args(&self.args);
        }

        cmd
    }
}

impl Deref for Rustc {
    type Target = CommonOptions;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for Rustc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

#[cfg(test)]
mod test {
    use super::Rustc;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        <Rustc as CommandFactory>::command().debug_assert()
    }
}
