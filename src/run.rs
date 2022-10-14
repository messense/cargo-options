use std::ops::{Deref, DerefMut};
use std::process::Command;

use clap::{ArgAction, Parser};

use crate::common::CommonOptions;

/// Run a binary or example of the local package
#[derive(Clone, Debug, Default, Parser)]
#[command(
    display_order = 1,
    after_help = "Run `cargo help run` for more detailed information."
)]
#[group(skip)]
pub struct Run {
    #[command(flatten)]
    pub common: CommonOptions,

    /// Package to run (see `cargo help pkgid`)
    #[arg(
        short = 'p',
        long = "package",
        value_name = "SPEC",
        action = ArgAction::Append
    )]
    pub packages: Vec<String>,

    /// Run the specified binary
    #[arg(long, value_name = "NAME", action = ArgAction::Append)]
    pub bin: Vec<String>,

    /// Run the specified example
    #[arg(long, value_name = "NAME", action = ArgAction::Append)]
    pub example: Vec<String>,

    /// Arguments for the binary to run
    #[arg(value_name = "args", trailing_var_arg = true, num_args = 0..)]
    pub args: Vec<String>,
}

impl Run {
    /// Build a `cargo run` command
    pub fn command(&self) -> Command {
        let mut cmd = CommonOptions::cargo_command();
        cmd.arg("run");

        self.common.apply(&mut cmd);

        for pkg in &self.packages {
            cmd.arg("--package").arg(pkg);
        }
        for bin in &self.bin {
            cmd.arg("--bin").arg(bin);
        }
        for example in &self.example {
            cmd.arg("--example").arg(example);
        }
        if !self.args.is_empty() {
            cmd.arg("--");
            cmd.args(&self.args);
        }

        cmd
    }
}

impl Deref for Run {
    type Target = CommonOptions;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for Run {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

#[cfg(test)]
mod test {
    use super::Run;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        <Run as CommandFactory>::command().debug_assert()
    }
}
