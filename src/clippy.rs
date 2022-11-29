use std::ops::{Deref, DerefMut};
use std::process::Command;

use clap::Parser;

use crate::check::CheckOptions;
use crate::common::CommonOptions;

/// Compile a local package and all of its dependencies
#[derive(Clone, Debug, Default, Parser)]
#[command(
    display_order = 1,
    after_help = "Run `cargo help clippy` for more detailed information."
)]
#[group(skip)]
pub struct Clippy {
    #[command(flatten)]
    pub common: CommonOptions,

    #[command(flatten)]
    pub check: CheckOptions,

    /// Ignore dependencies, run only on crate
    #[arg(long)]
    pub no_deps: bool,

    /// Automatically apply lint suggestions (see `cargo help clippy`)
    #[arg(long)]
    pub fix: bool,

    /// Arguments passed to rustc.
    #[arg(value_name = "args", trailing_var_arg = true, num_args = 0..)]
    pub args: Vec<String>,
}

impl Clippy {
    /// Build a `cargo clippy` command
    pub fn command(&self) -> Command {
        let mut cmd = CommonOptions::cargo_command();
        cmd.arg("clippy");

        self.common.apply(&mut cmd);
        self.check.apply(&mut cmd);

        if self.no_deps {
            cmd.arg("--no-deps");
        }
        if self.fix {
            cmd.arg("--fix");
        }
        if !self.args.is_empty() {
            cmd.arg("--");
            cmd.args(&self.args);
        }

        cmd
    }
}

impl Deref for Clippy {
    type Target = CommonOptions;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for Clippy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

#[cfg(test)]
mod test {
    use super::Clippy;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        <Clippy as CommandFactory>::command().debug_assert()
    }
}
