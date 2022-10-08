use std::ops::{Deref, DerefMut};
use std::process::Command;

use clap::{ArgAction, Parser};

use crate::common::CommonOptions;

/// Compile a package, and pass extra options to the compiler
#[derive(Clone, Debug, Default, Parser)]
#[command(
    display_order = 1,
    after_help = "Run `cargo help rustc` for more detailed information."
)]
pub struct Rustc {
    #[command(flatten)]
    pub common: CommonOptions,

    /// Package to build (see `cargo help pkgid`)
    #[arg(
        short = 'p',
        long = "package",
        value_name = "SPEC",
        action = ArgAction::Append
    )]
    pub packages: Vec<String>,

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

    /// Output compiler information without compiling
    #[arg(long, value_name = "INFO")]
    pub print: Option<String>,

    /// Comma separated list of types of crates for the compiler to emit
    #[arg(long, value_name = "CRATE-TYPE", action = ArgAction::Append)]
    pub crate_type: Vec<String>,

    /// Outputs a future incompatibility report at the end of the build (unstable)
    #[arg(long)]
    pub future_incompat_report: bool,

    /// Rustc flags
    #[arg(value_name = "args", trailing_var_arg = true, num_args = 0..)]
    pub args: Vec<String>,
}

impl Rustc {
    /// Build a `cargo rustc` command
    pub fn command(&self) -> Command {
        let mut cmd = CommonOptions::cargo_command();
        cmd.arg("rustc");

        self.common.apply(&mut cmd);

        for pkg in &self.packages {
            cmd.arg("--package").arg(pkg);
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
        if let Some(print) = self.print.as_ref() {
            cmd.arg("--print").arg(print);
        }
        if !self.crate_type.is_empty() {
            cmd.arg("--crate-type").arg(self.crate_type.join(","));
        }
        if self.future_incompat_report {
            cmd.arg("--future-incompat-report");
        }
        if !self.args.is_empty() {
            cmd.arg("--").args(&self.args);
        }

        cmd
    }
}

impl Deref for Rustc {
    type Target = CommonOptions;

    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for Rustc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

#[cfg(test)]
mod test {
    use super::Rustc;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        <Rustc as CommandFactory>::command().debug_assert()
    }
}
