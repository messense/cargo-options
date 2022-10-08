use cargo_options::{Build, Metadata, Run, Rustc, Test};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "cargo-mimic", display_order = 1)]
enum Opt {
    #[command(name = "build", aliases = &["b"] )]
    Build(Build),
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
