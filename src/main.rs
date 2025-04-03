mod graphs;
mod fs;
use clap::Parser;
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

    #[arg(short, long)]
    edge_chance: u8
}

fn main() {
    let args = Args::parse();

    let options = GraphOptions {
        left_size: args.left_column_size,
        right_size: args.right_column_size,
        edge_chance: args.edge_chance,
    };

    for _ in 0..args.generate_n_graphs {
        let graph = Graph::generate(&options);
        graph.save();
    }
}
