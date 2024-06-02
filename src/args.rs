use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    pub first: Option<String>,
}