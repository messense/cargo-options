use cargo_options::{Build, Metadata, Run, Rustc, Test};
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "cargo-mimic",
    global_setting(clap::AppSettings::DeriveDisplayOrder)
)]
enum Opt {
    #[clap(name = "build", aliases = &["b"] )]
    Build(Build),
    #[clap(name = "metadata")]
    Metadata(Metadata),
    #[clap(name = "rustc")]
    Rustc(Rustc),
    #[clap(name = "run", alias = "r")]
    Run(Run),
    #[clap(name = "test", alias = "t")]
    Test(Test),
}

fn main() {
    let _opt = Opt::parse();
}
