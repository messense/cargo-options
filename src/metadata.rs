use std::path::PathBuf;
use std::process::Command;

use clap::{ArgAction, Parser};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::heading;
use crate::CommonOptions;

/// Output the resolved dependencies of a package,
/// the concrete used versions including overrides,
/// in machine-readable format
#[derive(Clone, Debug, Default, Parser)]
#[command(
    display_order = 1,
    after_help = "Run `cargo help metadata` for more detailed information."
)]
#[group(skip)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Metadata {
    /// Do not print cargo log messages
    #[arg(short = 'q', long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub quiet: bool,

    /// Space or comma separated list of features to activate
    #[arg(
        short = 'F',
        long,
        action = ArgAction::Append,
        help_heading = heading::FEATURE_SELECTION,
        )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub features: Vec<String>,

    /// Activate all available features
    #[arg(long, help_heading = heading::FEATURE_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub all_features: bool,

    /// Do not activate the `default` feature
    #[arg(long, help_heading = heading::FEATURE_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub no_default_features: bool,

    /// Use verbose output (-vv very verbose/build.rs output)
    #[arg(short = 'v', long, action = ArgAction::Count)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub verbose: u8,

    /// Only include resolve dependencies matching the given target-triple
    #[arg(long, value_name = "TRIPLE", action = ArgAction::Append)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub filter_platform: Vec<String>,

    /// Output information only about the workspace members
    /// and don't fetch dependencies
    #[arg(long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub no_deps: bool,

    /// Path to Cargo.toml
    #[arg(long, value_name = "PATH")]
    #[cfg_attr(feature = "serde", serde(default))]
    pub manifest_path: Option<PathBuf>,

    /// Format version
    #[arg(long, value_name = "VERSION", value_parser = ["1"])]
    #[cfg_attr(feature = "serde", serde(default))]
    pub format_version: Option<String>,

    /// Coloring: auto, always, never
    #[arg(long, value_name = "WHEN")]
    #[cfg_attr(feature = "serde", serde(default))]
    pub color: Option<String>,

    /// Require Cargo.lock and cache are up to date
    #[arg(long, help_heading = heading::MANIFEST_OPTIONS)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub frozen: bool,

    /// Require Cargo.lock is up to date
    #[arg(long, help_heading = heading::MANIFEST_OPTIONS)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub locked: bool,

    /// Run without accessing the network
    #[arg(long, help_heading = heading::MANIFEST_OPTIONS)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub offline: bool,

    /// Override a configuration value (unstable)
    #[arg(long, value_name = "KEY=VALUE", action = ArgAction::Append)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub config: Vec<String>,

    /// Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    #[arg(short = 'Z', value_name = "FLAG", action = ArgAction::Append)]
    #[cfg_attr(feature = "serde", serde(default))]
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
            cmd.arg(format!("-{}", "v".repeat(self.verbose.into())));
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
