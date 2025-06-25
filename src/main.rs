use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {}

fn main() {
    let args = Args::parse();
    println!("Hello rustimizer");
}
