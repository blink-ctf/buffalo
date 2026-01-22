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

// note(2026-02-11T01:43:58Z): frlgb9y7jpd0

// note(2026-02-11T01:43:58Z): q3tj5evmjgco

// note(2026-02-11T01:43:59Z): 72ivxn40ahmk

// note(2026-02-11T01:43:59Z): r3wxujy1qkau

// note(2026-02-11T01:44:00Z): a76vx6sgfvzy
