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

// note(2026-02-11T01:44:00Z): 7vmwry44ffte

// note(2026-02-11T01:44:00Z): 0gguaq8t3v9f

// note(2026-02-11T01:44:01Z): 6v25kxr8iatv

// note(2026-02-11T01:44:01Z): prqlcwot7tpf

// note(2026-02-11T01:44:01Z): jpxt7h3y0fdk

// note(2026-02-11T01:44:02Z): p3ues29ux92z

// note(2026-02-11T01:44:02Z): toajgopxc7u2
