use clap::Parser;

use crate::common::CommonOptions;

/// Compile a package, and pass extra options to the compiler
/// with zig as the linker
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

    /// Do not activate the `default` feature
    #[clap(long)]
    pub no_default_features: bool,

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
