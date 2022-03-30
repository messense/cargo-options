use std::ops::{Deref, DerefMut};

use clap::Parser;

use crate::common::CommonOptions;

/// Run a binary or example of the local package
#[derive(Clone, Debug, Default, Parser)]
#[clap(
    setting = clap::AppSettings::DeriveDisplayOrder,
    trailing_var_arg = true,
    after_help = "Run `cargo help run` for more detailed information.")
]
pub struct Run {
    #[clap(flatten)]
    pub common: CommonOptions,

    /// Package to run (see `cargo help pkgid`)
    #[clap(short = 'p', long = "package", value_name = "SPEC")]
    pub packages: Option<String>,

    /// Run the specified binary
    #[clap(long, value_name = "NAME", multiple_values = true)]
    pub bin: Vec<String>,

    /// Run the specified example
    #[clap(long, value_name = "NAME", multiple_values = true)]
    pub example: Vec<String>,

    /// Arguments for the binary to run
    #[clap(takes_value = true, multiple_values = true)]
    pub args: Vec<String>,
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
