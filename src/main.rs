mod cmd;
mod nodes;
mod partitions;

use clap::{self, Parser};

#[derive(Parser)]
#[command(name = "rsqueue")]
#[command(bin_name = "rsqueue")]
#[command(author = "Yangyang Li <yangyang.li@northwestern.edu>")]
#[command(about = "A wrapper of slurm command that make that looks nicer")]
#[command(version = "1.0")]
enum Rsqueue {
    #[command(about = "Show Nodes Information")]
    Nodes(Nodes),
    #[command(about = "Show Partitions Information")]
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
            let mut partitions = partitions::Partitions::default();
            partitions.update_job_status();
            partitions.print_job_status();
        }
    }
}
