use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value = "world")]
    name: String,
}

fn main() {
    let args = Args::parse();
    println!("hello, {}", args.name);
}
