use std::path::PathBuf;

use clap::Parser;

/// common cargo options
#[derive(Clone, Debug, Default, Parser)]
pub struct CommonOptions {
    /// Do not print cargo log messages
    #[clap(short = 'q', long)]
    pub quiet: bool,

    /// Number of parallel jobs, defaults to # of CPUs
    #[clap(short = 'j', long, value_name = "N")]
    pub jobs: Option<usize>,

    /// Build artifacts in release mode, with optimizations
    #[clap(short = 'r', long)]
    pub release: bool,

    /// Build artifacts with the specified Cargo profile
    #[clap(long, value_name = "PROFILE-NAME")]
    pub profile: Option<String>,

    /// Space or comma separated list of features to activate
    #[clap(long, multiple_values = true)]
    pub features: Vec<String>,

    /// Activate all available features
    #[clap(long)]
    pub all_features: bool,

    /// Do not activate the `default` feature
    #[clap(long)]
    pub no_default_features: bool,

    /// Build for the target triple
    #[clap(
        long,
        value_name = "TRIPLE",
        env = "CARGO_BUILD_TARGET",
        multiple_occurrences = true
    )]
    pub target: Vec<String>,

    /// Directory for all generated artifacts
    #[clap(long, value_name = "DIRECTORY", parse(from_os_str))]
    pub target_dir: Option<PathBuf>,

    /// Path to Cargo.toml
    #[clap(long, value_name = "PATH", parse(from_os_str))]
    pub manifest_path: Option<PathBuf>,

    /// Ignore `rust-version` specification in packages
    #[clap(long)]
    pub ignore_rust_version: bool,

    /// Error format
    #[clap(long, value_name = "FMT", multiple_values = true)]
    pub message_format: Vec<String>,

    /// Output build graph in JSON (unstable)
    #[clap(long)]
    pub unit_graph: bool,

    /// Use verbose output (-vv very verbose/build.rs output)
    #[clap(short = 'v', long, parse(from_occurrences), max_occurrences = 2)]
    pub verbose: usize,

    /// Coloring: auto, always, never
    #[clap(long, value_name = "WHEN")]
    pub color: Option<String>,

    /// Require Cargo.lock and cache are up to date
    #[clap(long)]
    pub frozen: bool,

    /// Require Cargo.lock is up to date
    #[clap(long)]
    pub locked: bool,

    /// Run without accessing the network
    #[clap(long)]
    pub offline: bool,

    /// Override a configuration value (unstable)
    #[clap(long, value_name = "KEY=VALUE", multiple_values = true)]
    pub config: Vec<String>,

    /// Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    #[clap(short = 'Z', value_name = "FLAG", multiple_values = true)]
    pub unstable_flags: Vec<String>,

    /// Timing output formats (unstable) (comma separated): html, json
    #[clap(
        long,
        value_name = "FMTS",
        min_values = 0,
        value_delimiter = ',',
        require_equals = true
    )]
    pub timings: Option<Vec<String>>,
}
