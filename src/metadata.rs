use std::path::PathBuf;
use std::process::Command;

use clap::Parser;

use crate::CommonOptions;

/// Output the resolved dependencies of a package,
/// the concrete used versions including overrides,
/// in machine-readable format
#[derive(Clone, Debug, Default, Parser)]
#[clap(setting = clap::AppSettings::DeriveDisplayOrder, after_help = "Run `cargo help metadata` for more detailed information.")]
pub struct Metadata {
    /// Do not print cargo log messages
    #[clap(short = 'q', long)]
    pub quiet: bool,

    /// Space or comma separated list of features to activate
    #[clap(short = 'F', long, multiple_occurrences = true)]
    pub features: Vec<String>,

    /// Activate all available features
    #[clap(long)]
    pub all_features: bool,

    /// Do not activate the `default` feature
    #[clap(long)]
    pub no_default_features: bool,

    /// Use verbose output (-vv very verbose/build.rs output)
    #[clap(short = 'v', long, parse(from_occurrences), max_occurrences = 2)]
    pub verbose: usize,

    /// Only include resolve dependencies matching the given target-triple
    #[clap(long, value_name = "TRIPLE", multiple_occurrences = true)]
    pub filter_platform: Vec<String>,

    /// Output information only about the workspace members
    /// and don't fetch dependencies
    #[clap(long)]
    pub no_deps: bool,

    /// Path to Cargo.toml
    #[clap(long, value_name = "PATH", parse(from_os_str))]
    pub manifest_path: Option<PathBuf>,

    /// Format version
    #[clap(long, value_name = "VERSION", possible_values = &["1"])]
    pub format_version: Option<String>,

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
    #[clap(long, value_name = "KEY=VALUE", multiple_occurrences = true)]
    pub config: Vec<String>,

    /// Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    #[clap(short = 'Z', value_name = "FLAG", multiple_occurrences = true)]
    pub unstable_flags: Vec<String>,
}

impl Metadata {
    /// Build a `cargo metadata` command
    pub fn command(&self) -> Command {
        let mut cmd = CommonOptions::cargo_command();
        cmd.arg("metadata");
        if self.quiet {
            cmd.arg("--quiet");
        }
        if self.verbose > 0 {
            cmd.arg(format!("-{}", "v".repeat(self.verbose)));
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
        for platform in &self.filter_platform {
            cmd.arg("--filter-platform").arg(platform);
        }
        if self.no_deps {
            cmd.arg("--no-deps");
        }
        if let Some(path) = self.manifest_path.as_ref() {
            cmd.arg("--manifest-path").arg(path);
        }
        if let Some(format_version) = self.format_version.as_ref() {
            cmd.arg("--format-version").arg(format_version);
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
        cmd
    }
}

#[cfg(test)]
mod test {
    use super::Metadata;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        <Metadata as CommandFactory>::command().debug_assert()
    }
}
