use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::process::Command;

use clap::{ArgAction, Parser};

use crate::common::CommonOptions;

/// Compile a local package and all of its dependencies
#[derive(Clone, Debug, Default, Parser)]
#[command(
    display_order = 1,
    after_help = "Run `cargo help build` for more detailed information."
)]
#[group(skip)]
pub struct Build {
    #[command(flatten)]
    pub common: CommonOptions,

    /// Package to build (see `cargo help pkgid`)
    #[arg(
        short = 'p',
        long = "package",
        value_name = "SPEC",
        action = ArgAction::Append,
    )]
    pub packages: Vec<String>,

    /// Build all packages in the workspace
    #[arg(long)]
    pub workspace: bool,

    /// Exclude packages from the build
    #[arg(long, value_name = "SPEC", action = ArgAction::Append)]
    pub exclude: Vec<String>,

    /// Alias for workspace (deprecated)
    #[arg(long)]
    pub all: bool,

    /// Build only this package's library
    #[arg(long)]
    pub lib: bool,

    /// Build only the specified binary
    #[arg(long, value_name = "NAME", action = ArgAction::Append)]
    pub bin: Vec<String>,

    /// Build all binaries
    #[arg(long)]
    pub bins: bool,

    /// Build only the specified example
    #[arg(long, value_name = "NAME", action = ArgAction::Append)]
    pub example: Vec<String>,

    /// Build all examples
    #[arg(long)]
    pub examples: bool,

    /// Build only the specified test target
    #[arg(long, value_name = "NAME", action = ArgAction::Append)]
    pub test: Vec<String>,

    /// Build all tests
    #[arg(long)]
    pub tests: bool,

    /// Build only the specified bench target
    #[arg(long, value_name = "NAME", action = ArgAction::Append)]
    pub bench: Vec<String>,

    /// Build all benches
    #[arg(long)]
    pub benches: bool,

    /// Build all targets
    #[arg(long)]
    pub all_targets: bool,

    /// Copy final artifacts to this directory (unstable)
    #[arg(long, value_name = "PATH")]
    pub out_dir: Option<PathBuf>,

    /// Output the build plan in JSON (unstable)
    #[arg(long)]
    pub build_plan: bool,

    /// Outputs a future incompatibility report at the end of the build (unstable)
    #[arg(long)]
    pub future_incompat_report: bool,
}

impl Build {
    /// Build a `cargo build` command
    pub fn command(&self) -> Command {
        let mut cmd = CommonOptions::cargo_command();
        cmd.arg("build");

        self.common.apply(&mut cmd);

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
        if let Some(dir) = self.out_dir.as_ref() {
            cmd.arg("--out-dir").arg(dir);
        }
        if self.build_plan {
            cmd.arg("--build-plan");
        }
        if self.future_incompat_report {
            cmd.arg("--future-incompat-report");
        }

        cmd
    }
}

impl Deref for Build {
    type Target = CommonOptions;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for Build {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

#[cfg(test)]
mod test {
    use super::Build;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        <Build as CommandFactory>::command().debug_assert()
    }
}
