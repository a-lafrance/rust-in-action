// use libactionkv::ActionKV;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, about, version)]
#[clap(propagate_version = true)]
struct Args {
    #[clap(subcommand)]
    op: Operation
}

#[derive(Subcommand)]
enum Operation {
    Get { key: String },
    Insert { key: String, value: String },
    Update { key: String, value: String },
    Delete { key: String },
}

fn main() {
    let op = Args::parse().op;

    match op {
        Operation::Get { .. } => todo!(),
        Operation::Insert { .. } => todo!(),
        Operation::Update { .. } => todo!(),
        Operation::Delete { .. } => todo!(),
    }
}
