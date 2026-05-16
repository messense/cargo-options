use std::path::PathBuf;
use std::process::Command;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::heading;
use clap::{ArgAction, Parser};

/// common cargo options
#[derive(Clone, Debug, Default, Parser)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CommonOptions {
    /// Do not print cargo log messages
    #[arg(short = 'q', long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub quiet: bool,

    /// Number of parallel jobs, defaults to # of CPUs.
    #[arg(
        short = 'j',
        long,
        value_name = "N",
        allow_hyphen_values = true,
        help_heading = heading::COMPILATION_OPTIONS,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub jobs: Option<usize>,

    /// Do not abort the build as soon as there is an error
    #[arg(long, help_heading = heading::COMPILATION_OPTIONS)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub keep_going: bool,

    /// Build artifacts with the specified profile
    #[arg(
        long,
        value_name = "PROFILE-NAME",
        help_heading = heading::COMPILATION_OPTIONS,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub profile: Option<String>,

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

    /// Build for the target triple
    #[arg(
        long,
        value_name = "TRIPLE",
        num_args = 0..=1,
        default_missing_value = "",
        action = ArgAction::Append,
        help_heading = heading::COMPILATION_OPTIONS,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub target: Vec<String>,

    /// Directory for all generated artifacts
    #[arg(
        long,
        value_name = "DIRECTORY",
        help_heading = heading::COMPILATION_OPTIONS,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub target_dir: Option<PathBuf>,

    /// Error format
    #[arg(
        long,
        value_name = "FMT",
        action = ArgAction::Append,
        value_delimiter = ',',
        ignore_case = true,
        value_parser = [
            "human",
            "short",
            "json",
            "json-diagnostic-short",
            "json-diagnostic-rendered-ansi",
            "json-render-diagnostics",
        ],
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub message_format: Vec<String>,

    /// Use verbose output (-vv very verbose/build.rs output)
    #[arg(short = 'v', long, action = ArgAction::Count)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub verbose: u8,

    /// Coloring
    #[arg(long, value_name = "WHEN", value_parser = ["auto", "always", "never"])]
    #[cfg_attr(feature = "serde", serde(default))]
    pub color: Option<String>,

    /// Equivalent to specifying both --locked and --offline
    #[arg(long, help_heading = heading::MANIFEST_OPTIONS)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub frozen: bool,

    /// Assert that `Cargo.lock` will remain unchanged
    #[arg(long, help_heading = heading::MANIFEST_OPTIONS)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub locked: bool,

    /// Run without accessing the network
    #[arg(long, help_heading = heading::MANIFEST_OPTIONS)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub offline: bool,

    /// Override a configuration value
    #[arg(long, value_name = "KEY=VALUE|PATH", action = ArgAction::Append)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub config: Vec<String>,

    /// Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    #[arg(short = 'Z', value_name = "FLAG", action = ArgAction::Append)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub unstable_flags: Vec<String>,

    /// Output a build timing report at the end of the build
    #[arg(long, help_heading = heading::COMPILATION_OPTIONS)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub timings: bool,
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
        if self.keep_going {
            cmd.arg("--keep-going");
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
            cmd.arg("--target");
            if !target.is_empty() {
                cmd.arg(target);
            }
        });

        if let Some(dir) = self.target_dir.as_ref() {
            cmd.arg("--target-dir").arg(dir);
        }
        for fmt in &self.message_format {
            cmd.arg("--message-format").arg(fmt);
        }
        if self.verbose > 0 {
            cmd.arg(format!("-{}", "v".repeat(self.verbose.into())));
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
        if self.timings {
            cmd.arg("--timings");
        }
    }

    pub(crate) fn cargo_command() -> Command {
        let cargo = match std::env::var_os("CARGO") {
            Some(cargo) => cargo.into(),
            None => PathBuf::from("cargo"),
        };
        let mut cmd = Command::new(cargo);
        cmd.env_remove("CARGO");
        cmd
    }
}
