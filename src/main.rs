use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {

    #[arg(short, long)]
    left_column_size: u32,

    #[arg(short, long)]
    right_column_size: u32,

    #[arg(short, long)]
    generate_n_graphs: u8
}

fn main() {
    let args = Args::parse();
    println!(
        "Left column size of {}, Right column size of {}, {} times", 
        &args.left_column_size,
        &args.right_column_size,
        &args.generate_n_graphs
    );
}
