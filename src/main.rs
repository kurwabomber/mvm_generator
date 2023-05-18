use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    map: String,
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    start_money: i32,
    #[arg(short, long)]
    waves: i32,
    #[arg(short, long)]
    respawnwavetime: i32,
    #[arg(short, long)]
    config: String
}

fn main() {
    let args = Args::parse();
    println!("Hello, world!");
}
