use std::ops::{Deref, DerefMut};
use std::process::Command;

use clap::Parser;

use crate::check::CheckOptions;
use crate::common::CommonOptions;

/// Execute all unit and integration tests and build examples of a local package
#[derive(Clone, Debug, Default, Parser)]
#[command(
    display_order = 1,
    after_help = "Run `cargo help test` for more detailed information.\nRun `cargo test -- --help` for test binary options."
)]
#[group(skip)]
pub struct Test {
    #[command(flatten)]
    pub common: CommonOptions,

    #[command(flatten)]
    pub check: CheckOptions,

    /// Test only this library's documentation
    #[arg(long)]
    pub doc: bool,

    /// Compile, but don't run tests
    #[arg(long)]
    pub no_run: bool,

    /// Run all tests regardless of failure
    #[arg(long)]
    pub no_fail_fast: bool,

    /// Outputs a future incompatibility report at the end of the build (unstable)
    #[arg(long)]
    pub future_incompat_report: bool,

    /// If specified, only run tests containing this string in their names
    #[arg(value_name = "TESTNAME")]
    pub test_name: Option<String>,

    /// Arguments for the test binary
    #[arg(value_name = "args", trailing_var_arg = true, num_args = 0..)]
    pub args: Vec<String>,
}

impl Test {
    /// Build a `cargo test` command
    pub fn command(&self) -> Command {
        let mut cmd = CommonOptions::cargo_command();
        cmd.arg("test");

        self.common.apply(&mut cmd);
        self.check.apply(&mut cmd);

        if self.doc {
            cmd.arg("--doc");
        }
        if self.no_run {
            cmd.arg("--no-run");
        }
        if self.no_fail_fast {
            cmd.arg("--no-fail-fast");
        }
        if let Some(test_name) = self.test_name.as_ref() {
            cmd.arg(test_name);
        }
        if !self.args.is_empty() {
            cmd.arg("--");
            cmd.args(&self.args);
        }

        cmd
    }
}

impl Deref for Test {
    type Target = CommonOptions;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for Test {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

#[cfg(test)]
mod tests {
    use super::Test;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        <Test as CommandFactory>::command().debug_assert()
    }
}
