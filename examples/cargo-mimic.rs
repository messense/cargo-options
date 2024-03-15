use cargo_options::{Build, Check, Clippy, Doc, Install, Metadata, Run, Rustc, Test};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "cargo-mimic",
    display_order = 1,
    styles = cargo_options::styles(),
)]
enum Opt {
    #[command(name = "build", aliases = &["b"] )]
    Build(Build),
    #[command(name = "clippy")]
    Clippy(Clippy),
    #[command(name = "check", aliases = &["c"])]
    Check(Check),
    #[command(name = "doc")]
    Doc(Doc),
    #[command(name = "install")]
    Install(Install),
    #[command(name = "metadata")]
    Metadata(Metadata),
    #[command(name = "rustc")]
    Rustc(Rustc),
    #[command(name = "run", alias = "r")]
    Run(Run),
    #[command(name = "test", alias = "t")]
    Test(Test),
}

fn main() {
    let _opt = Opt::parse();
}
