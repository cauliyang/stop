#![allow(unused)]
mod cmd;
mod nodes;
mod partitions;
mod table;


use clap::Parser;
use cmd::cmds;
use nodes::node;

#[derive(Parser)]
#[command(name = "rsqueue")]
#[command(bin_name = "rsqueue")]
#[command(author = "Yangyang Li <yangyang.li@northwestern.edu>")]
#[command(about = "A wrapper of slurm command that make that looks nicer")]
#[command(version = "1.0")]
enum Rsqueue {
    #[command(about="Print Nodes INFO")]
    Nodes(Nodes),
    #[command(about="Print Partitions INFO")]
    Partitions(Partitions),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct Nodes {}

#[derive(clap::Args)]
#[command(author, version, about)]
struct Partitions {
    #[arg(long)]
    me: Option<bool>,
}

fn main() {
    match Rsqueue::parse() {
        Rsqueue::Nodes(Nodes {}) => println! {"nodes"},
        Rsqueue::Partitions(Partitions { me }) => {
            cmds::squeue(Some(&["--me"]));
        },
    }
}
