mod graphs;
mod fs;
use clap::{Parser, Subcommand};
use graphs::{Graph, GraphOptions};
use fs::FileHandler;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {

    #[arg(short, long)]
    left_column_size: u32,

    #[arg(short, long)]
    right_column_size: u32,

    #[arg(short, long)]
    generate_n_graphs: u8,

    #[command(subcommand)]
    cmd: Commands

}

#[derive(Subcommand, Debug)]
enum Commands {
    Balanced {

    },
    Random {
        #[clap(short, long)]
        edge_chance: u8,
    }
}

fn main() {
    let args = Args::parse();

    let mut edge_chances: u8 = 0;
    let mut balanced_arcs: bool = false;

    match args.cmd {
        Commands::Balanced {  } => {
            balanced_arcs = true;
        },
        Commands::Random { edge_chance } => {
            edge_chances = edge_chance
        }
    }

    let options = GraphOptions {
        left_size: args.left_column_size,
        right_size: args.right_column_size,
        edge_chance: edge_chances,
        balanced_arcs: balanced_arcs
    };

    for _ in 0..args.generate_n_graphs {
        let graph = Graph::generate(&options);
        graph.save();
    }
}
