use std::path::PathBuf;
use std::process::Command;

use clap::Parser;

/// common cargo options
#[derive(Clone, Debug, Default, Parser)]
pub struct CommonOptions {
    /// Do not print cargo log messages
    #[clap(short = 'q', long)]
    pub quiet: bool,

    /// Number of parallel jobs, defaults to # of CPUs
    #[clap(short = 'j', long, value_name = "N")]
    pub jobs: Option<usize>,

    /// Build artifacts in release mode, with optimizations
    #[clap(short = 'r', long)]
    pub release: bool,

    /// Build artifacts with the specified Cargo profile
    #[clap(long, value_name = "PROFILE-NAME")]
    pub profile: Option<String>,

    /// Space or comma separated list of features to activate
    #[clap(long, multiple_values = true)]
    pub features: Vec<String>,

    /// Activate all available features
    #[clap(long)]
    pub all_features: bool,

    /// Do not activate the `default` feature
    #[clap(long)]
    pub no_default_features: bool,

    /// Build for the target triple
    #[clap(
        long,
        value_name = "TRIPLE",
        env = "CARGO_BUILD_TARGET",
        multiple_occurrences = true
    )]
    pub target: Vec<String>,

    /// Directory for all generated artifacts
    #[clap(long, value_name = "DIRECTORY", parse(from_os_str))]
    pub target_dir: Option<PathBuf>,

    /// Path to Cargo.toml
    #[clap(long, value_name = "PATH", parse(from_os_str))]
    pub manifest_path: Option<PathBuf>,

    /// Ignore `rust-version` specification in packages
    #[clap(long)]
    pub ignore_rust_version: bool,

    /// Error format
    #[clap(long, value_name = "FMT", multiple_values = true)]
    pub message_format: Vec<String>,

    /// Output build graph in JSON (unstable)
    #[clap(long)]
    pub unit_graph: bool,

    /// Use verbose output (-vv very verbose/build.rs output)
    #[clap(short = 'v', long, parse(from_occurrences), max_occurrences = 2)]
    pub verbose: usize,

    /// Coloring: auto, always, never
    #[clap(long, value_name = "WHEN")]
    pub color: Option<String>,

    /// Require Cargo.lock and cache are up to date
    #[clap(long)]
    pub frozen: bool,

    /// Require Cargo.lock is up to date
    #[clap(long)]
    pub locked: bool,

    /// Run without accessing the network
    #[clap(long)]
    pub offline: bool,

    /// Override a configuration value (unstable)
    #[clap(long, value_name = "KEY=VALUE", multiple_values = true)]
    pub config: Vec<String>,

    /// Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    #[clap(short = 'Z', value_name = "FLAG", multiple_values = true)]
    pub unstable_flags: Vec<String>,

    /// Timing output formats (unstable) (comma separated): html, json
    #[clap(
        long,
        value_name = "FMTS",
        min_values = 0,
        value_delimiter = ',',
        require_equals = true
    )]
    pub timings: Option<Vec<String>>,
}

impl CommonOptions {
    /// Apply options to `Command`
    pub fn apply(&self, cmd: &mut Command) {
        if self.quiet {
            cmd.arg("--quiet");
        }
        if let Some(jobs) = self.jobs {
            cmd.arg("--jobs").arg(jobs.to_string());
        }
        if self.release {
            cmd.arg("--release");
        }
        if let Some(profile) = self.profile.as_ref() {
            cmd.arg("--profile").arg(profile);
        }
        for feature in &self.features {
            cmd.arg("--features").arg(feature);
        }
        if self.all_features {
            cmd.arg("--all-features");
        }
        if self.no_default_features {
            cmd.arg("--no-default-features");
        }

        // Support <target_triple>.<glibc_version> syntax
        // For example: x86_64-unknown-linux-gnu.2.17
        let rust_targets = self
            .target
            .iter()
            .map(|target| target.split_once('.').map(|(t, _)| t).unwrap_or(target))
            .collect::<Vec<&str>>();
        rust_targets.iter().for_each(|target| {
            cmd.arg("--target").arg(&target);
        });

        if let Some(dir) = self.target_dir.as_ref() {
            cmd.arg("--target-dir").arg(dir);
        }
        if let Some(path) = self.manifest_path.as_ref() {
            cmd.arg("--manifest-path").arg(path);
        }
        if self.ignore_rust_version {
            cmd.arg("--ignore-rust-version");
        }
        for fmt in &self.message_format {
            cmd.arg("--message-format").arg(fmt);
        }
        if self.unit_graph {
            cmd.arg("--unit-graph");
        }
        if self.verbose > 0 {
            cmd.arg(format!("-{}", "v".repeat(self.verbose)));
        }
        if let Some(color) = self.color.as_ref() {
            cmd.arg("--color").arg(color);
        }
        if self.frozen {
            cmd.arg("--frozen");
        }
        if self.locked {
            cmd.arg("--locked");
        }
        if self.offline {
            cmd.arg("--offline");
        }
        for config in &self.config {
            cmd.arg("--config").arg(config);
        }
        for flag in &self.unstable_flags {
            cmd.arg("-Z").arg(flag);
        }
        if let Some(timings) = &self.timings {
            if timings.is_empty() {
                cmd.arg("--timings");
            } else {
                let timings: Vec<_> = timings.iter().map(|x| x.as_str()).collect();
                cmd.arg(format!("--timings={}", timings.join(",")));
            }
        }
    }

    pub(crate) fn cargo_path() -> PathBuf {
        match std::env::var_os("CARGO") {
            Some(cargo) => cargo.into(),
            None => PathBuf::from("cargo"),
        }
    }
}
