use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::process::Command;

use clap::{ArgAction, Parser};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::common::CommonOptions;
use crate::heading;

/// Install a Rust binary. Default location is $HOME/.cargo/bin
#[derive(Clone, Debug, Default, Parser)]
#[command(
    display_order = 1,
    after_help = "Run `cargo help install` for more detailed information."
)]
#[group(skip)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Install {
    #[command(flatten)]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common: CommonOptions,

    /// Specify a version to install
    #[arg(long, value_name = "VERSION", alias = "vers", requires = "crates")]
    #[cfg_attr(feature = "serde", serde(default))]
    pub version: Option<String>,

    /// Git URL to install the specified crate from
    #[arg(long, value_name = "URL", conflicts_with_all = ["path", "index", "registry"])]
    #[cfg_attr(feature = "serde", serde(default))]
    pub git: Option<String>,

    /// Branch to use when installing from git
    #[arg(long, value_name = "BRANCH", requires = "git")]
    #[cfg_attr(feature = "serde", serde(default))]
    pub branch: Option<String>,

    /// Tag to use when installing from git
    #[arg(long, value_name = "TAG", requires = "git")]
    #[cfg_attr(feature = "serde", serde(default))]
    pub tag: Option<String>,

    /// Specific commit to use when installing from git
    #[arg(long, value_name = "SHA", requires = "git")]
    #[cfg_attr(feature = "serde", serde(default))]
    pub rev: Option<String>,

    /// Filesystem path to local crate to install
    #[arg(long, value_name = "PATH", conflicts_with_all = ["git", "index", "registry"])]
    #[cfg_attr(feature = "serde", serde(default))]
    pub path: Option<PathBuf>,

    /// list all installed packages and their versions
    #[arg(long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub list: bool,

    /// Force overwriting existing crates or binaries
    #[arg(short, long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub force: bool,

    /// Do not save tracking information
    #[arg(long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub no_track: bool,

    /// Build in debug mode (with the 'dev' profile) instead of release mode
    #[arg(long)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub debug: bool,

    /// Directory to install packages into
    #[arg(long, value_name = "DIR")]
    #[cfg_attr(feature = "serde", serde(default))]
    pub root: Option<PathBuf>,

    /// Registry index to install from
    #[arg(
        long,
        value_name = "INDEX",
        conflicts_with_all = ["git", "path", "registry"],
        requires = "crates",
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub index: Option<String>,

    /// Registry to use
    #[arg(
        long,
        value_name = "REGISTRY",
        conflicts_with_all = ["git", "path", "index"],
        requires = "crates",
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub registry: Option<String>,

    /// Install only the specified binary
    #[arg(
        long,
        value_name = "NAME",
        action = ArgAction::Append,
        num_args=0..=1,
        help_heading = heading::TARGET_SELECTION,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub bin: Vec<String>,

    /// Install all binaries
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub bins: bool,

    /// Install only the specified example
    #[arg(
        long,
        value_name = "NAME",
        action = ArgAction::Append,
        num_args=0..=1,
        help_heading = heading::TARGET_SELECTION,
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    pub example: Vec<String>,

    /// Install all examples
    #[arg(long, help_heading = heading::TARGET_SELECTION)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub examples: bool,

    #[arg(value_name = "crate", action = ArgAction::Append, num_args = 0..)]
    #[cfg_attr(feature = "serde", serde(default))]
    pub crates: Vec<String>,
}

impl Install {
    /// Build a `cargo install` command
    pub fn command(&self) -> Command {
        let mut cmd = CommonOptions::cargo_command();
        cmd.arg("install");

        self.common.apply(&mut cmd);

        if let Some(version) = self.version.as_ref() {
            cmd.arg("--version").arg(version);
        }
        if let Some(git) = self.git.as_ref() {
            cmd.arg("--git").arg(git);
        }
        if let Some(branch) = self.branch.as_ref() {
            cmd.arg("--branch").arg(branch);
        }
        if let Some(tag) = self.tag.as_ref() {
            cmd.arg("--tag").arg(tag);
        }
        if let Some(rev) = self.rev.as_ref() {
            cmd.arg("--rev").arg(rev);
        }
        if let Some(path) = self.path.as_ref() {
            cmd.arg("--path").arg(path);
        }
        if self.list {
            cmd.arg("--list");
        }
        if self.force {
            cmd.arg("--force");
        }
        if self.no_track {
            cmd.arg("--no-track");
        }
        if self.debug {
            cmd.arg("--debug");
        }
        if let Some(root) = self.root.as_ref() {
            cmd.arg("--root").arg(root);
        }
        if let Some(index) = self.index.as_ref() {
            cmd.arg("--index").arg(index);
        }
        if let Some(registry) = self.registry.as_ref() {
            cmd.arg("--registry").arg(registry);
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
        cmd.args(&self.crates);

        cmd
    }
}

impl Deref for Install {
    type Target = CommonOptions;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for Install {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

#[cfg(test)]
mod test {
    use super::Install;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        <Install as CommandFactory>::command().debug_assert()
    }
}
