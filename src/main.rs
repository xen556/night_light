use clap::Parser;
mod logic;

fn main() {
    let args = Args::parse();

    if args.level >= 1 && args.level <= 100 {
        logic::night_light(args.level);
    }
    else {
        eprintln!("The level must be between 1 and 100!");
    }
}

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = 50, help = "1-100")]
    level: i32,
}