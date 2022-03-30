use std::ops::{Deref, DerefMut};

use clap::Parser;

use crate::common::CommonOptions;

/// Execute all unit and integration tests and build examples of a local package
#[derive(Clone, Debug, Default, Parser)]
#[clap(
    setting = clap::AppSettings::DeriveDisplayOrder,
    trailing_var_arg = true,
    after_help = "Run `cargo help test` for more detailed information.\nRun `cargo test -- --help` for test binary options.")
]
pub struct Test {
    #[clap(flatten)]
    pub common: CommonOptions,

    /// Package to build (see `cargo help pkgid`)
    #[clap(
        short = 'p',
        long = "package",
        value_name = "SPEC",
        multiple_values = true
    )]
    pub packages: Vec<String>,

    /// Test all packages in the workspace
    #[clap(long)]
    pub workspace: bool,

    /// Exclude packages from the build
    #[clap(long, value_name = "SPEC", multiple_values = true)]
    pub exclude: Vec<String>,

    /// Alias for workspace (deprecated)
    #[clap(long)]
    pub all: bool,

    /// Test only this package's library
    #[clap(long)]
    pub lib: bool,

    /// Test only the specified binary
    #[clap(long, value_name = "NAME", multiple_values = true)]
    pub bin: Vec<String>,

    /// Test all binaries
    #[clap(long)]
    pub bins: bool,

    /// Test only the specified example
    #[clap(long, value_name = "NAME", multiple_values = true)]
    pub example: Vec<String>,

    /// Test all examples
    #[clap(long)]
    pub examples: bool,

    /// Test only the specified test target
    #[clap(long, value_name = "NAME", multiple_values = true)]
    pub test: Vec<String>,

    /// Test all tests
    #[clap(long)]
    pub tests: bool,

    /// Test only the specified bench target
    #[clap(long, value_name = "NAME", multiple_values = true)]
    pub bench: Vec<String>,

    /// Test all benches
    #[clap(long)]
    pub benches: bool,

    /// Test all targets
    #[clap(long)]
    pub all_targets: bool,

    /// Test only this library's documentation
    #[clap(long)]
    pub doc: bool,

    /// Compile, but don't run tests
    #[clap(long)]
    pub no_run: bool,

    /// Run all tests regardless of failure
    #[clap(long)]
    pub no_fail_fast: bool,

    /// Outputs a future incompatibility report at the end of the build (unstable)
    #[clap(long)]
    pub future_incompat_report: bool,

    /// If specified, only run tests containing this string in their names
    #[clap(value_name = "TESTNAME", takes_value = true)]
    pub test_name: Option<String>,

    /// Arguments for the test binary
    #[clap(takes_value = true, multiple_values = true)]
    pub args: Vec<String>,
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
