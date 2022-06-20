use std::ops::{Deref, DerefMut};
use std::process::Command;

use clap::Parser;

use crate::common::CommonOptions;

/// Compile a package, and pass extra options to the compiler
#[derive(Clone, Debug, Default, Parser)]
#[clap(
    setting = clap::AppSettings::DeriveDisplayOrder,
    trailing_var_arg = true,
    after_help = "Run `cargo help rustc` for more detailed information."
)]
pub struct Rustc {
    #[clap(flatten)]
    pub common: CommonOptions,

    /// Package to build (see `cargo help pkgid`)
    #[clap(short = 'p', long = "package", value_name = "SPEC")]
    pub packages: Vec<String>,

    /// Build only this package's library
    #[clap(long)]
    pub lib: bool,

    /// Build only the specified binary
    #[clap(long, value_name = "NAME", multiple_values = true)]
    pub bin: Vec<String>,

    /// Build all binaries
    #[clap(long)]
    pub bins: bool,

    /// Build only the specified example
    #[clap(long, value_name = "NAME", multiple_values = true)]
    pub example: Vec<String>,

    /// Build all examples
    #[clap(long)]
    pub examples: bool,

    /// Build only the specified test target
    #[clap(long, value_name = "NAME", multiple_values = true)]
    pub test: Vec<String>,

    /// Build all tests
    #[clap(long)]
    pub tests: bool,

    /// Build only the specified bench target
    #[clap(long, value_name = "NAME", multiple_values = true)]
    pub bench: Vec<String>,

    /// Build all benches
    #[clap(long)]
    pub benches: bool,

    /// Build all targets
    #[clap(long)]
    pub all_targets: bool,

    /// Output compiler information without compiling
    #[clap(long, value_name = "INFO")]
    pub print: Option<String>,

    /// Comma separated list of types of crates for the compiler to emit (unstable)
    #[clap(
        long,
        value_name = "CRATE-TYPE",
        use_value_delimiter = true,
        multiple_values = true
    )]
    pub crate_type: Vec<String>,

    /// Outputs a future incompatibility report at the end of the build (unstable)
    #[clap(long)]
    pub future_incompat_report: bool,

    /// Rustc flags
    #[clap(takes_value = true, multiple_values = true)]
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
