use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::process::Command;

use clap::{ArgAction, Parser};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::common::CommonOptions;
use crate::heading;

/// `cargo bench` options
#[derive(Clone, Debug, Default, Parser)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct BenchOptions {
    /// Package to run benchmarks for
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

    /// Benchmark all packages in the workspace
    #[arg(long, help_heading = heading::PACKAGE_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub workspace: bool,

    /// Exclude packages from the benchmark
    #[arg(
        long,
        value_name = "SPEC",
        action = ArgAction::Append,
        help_heading = heading::PACKAGE_SELECTION,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub exclude: Vec<String>,

    /// Alias for --workspace (deprecated)
    #[arg(long, help_heading = heading::PACKAGE_SELECTION,)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub all: bool,

    /// Benchmark only this package's library
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub lib: bool,

    /// Benchmark only the specified binary
    #[arg(
        long,
        value_name = "NAME",
        action = ArgAction::Append,
        num_args=0..=1,
        help_heading = heading::TARGET_SELECTION,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub bin: Vec<String>,

    /// Benchmark all binaries
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub bins: bool,

    /// Benchmark only the specified example
    #[arg(
        long,
        value_name = "NAME",
        action = ArgAction::Append,
        num_args=0..=1,
        help_heading = heading::TARGET_SELECTION,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub example: Vec<String>,

    /// Benchmark all examples
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: bool,

    /// Benchmark only the specified test target
    #[arg(
        long,
        value_name = "NAME",
        action = ArgAction::Append,
        help_heading = heading::TARGET_SELECTION,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub test: Vec<String>,

    /// Benchmark all targets that have `test = true` set
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub tests: bool,

    /// Benchmark only the specified bench target
    #[arg(
        long,
        value_name = "NAME",
        action = ArgAction::Append,
        help_heading = heading::TARGET_SELECTION,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub bench: Vec<String>,

    /// Benchmark all targets that have `bench = true` set
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub benches: bool,

    /// Benchmark all targets
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub all_targets: bool,

    /// Compile, but don't run benchmarks
    #[arg(long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub no_run: bool,

    /// Run all benchmarks regardless of failure
    #[arg(long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub no_fail_fast: bool,

    /// If specified, only run benches containing this string in their names
    #[arg(value_name = "BENCHNAME")]
    #[cfg_attr(feature = "serde", serde(default))]
    pub bench_name: Option<String>,

    /// Arguments for the bench binary
    #[arg(value_name = "ARGS", last = true, num_args = 0..)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub args: Vec<String>,
}

impl BenchOptions {
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
        if self.no_run {
            cmd.arg("--no-run");
        }
        if self.no_fail_fast {
            cmd.arg("--no-fail-fast");
        }
        if self.bench_name.is_some() || !self.args.is_empty() {
            cmd.arg("--");
            if let Some(bench_name) = self.bench_name.as_ref() {
                cmd.arg(bench_name);
            }
        }
        cmd.args(&self.args);
    }
}

/// Execute all benchmarks of a local package
#[derive(Clone, Debug, Default, Parser)]
#[command(
    display_order = 1,
    after_help = "Run `cargo help bench` for more detailed information."
)]
#[group(skip)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Bench {
    #[command(flatten)]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common: CommonOptions,

    #[command(flatten)]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub bench: BenchOptions,

    /// Path to Cargo.toml
    #[arg(long, value_name = "PATH", help_heading = heading::MANIFEST_OPTIONS)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub manifest_path: Option<PathBuf>,

    /// Ignore `rust-version` specification in packages
    #[arg(long, help_heading = heading::MANIFEST_OPTIONS)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub ignore_rust_version: bool,

    /// Output build graph in JSON (unstable)
    #[arg(long, help_heading = heading::COMPILATION_OPTIONS)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit_graph: bool,
}

impl Bench {
    /// Build a `cargo bench` command
    pub fn command(&self) -> Command {
        let mut cmd = CommonOptions::cargo_command();
        cmd.arg("bench");

        self.common.apply(&mut cmd);
        self.bench.apply(&mut cmd);

        if let Some(path) = self.manifest_path.as_ref() {
            cmd.arg("--manifest-path").arg(path);
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

impl Deref for Bench {
    type Target = CommonOptions;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for Bench {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

#[cfg(test)]
mod test {
    use super::Bench;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        <Bench as CommandFactory>::command().debug_assert()
    }
}
