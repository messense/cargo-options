mod build;
mod check;
mod clippy;
mod common;
mod install;
mod metadata;
mod run;
mod rustc;
mod test;

// Specify crate to satisfy naming overlap w/ rustc clippy
pub use crate::clippy::Clippy;
pub use build::Build;
pub use check::Check;
pub use common::CommonOptions;
pub use install::Install;
pub use metadata::Metadata;
pub use run::Run;
pub use rustc::Rustc;
pub use test::Test;
