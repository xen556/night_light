use clap::Parser;
mod logic;
fn main() {
    let args = Args::parse();
    if args.daemon {
        logic::daemon();
    }
    else {
        if args.level >= 0 && args.level <= 100 {
            logic::level_save(args.level);
        } else {
            eprintln!("Level must be between 0 and 100!");
        }
    }
}
#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = 50, help = "1-100")]
    level: i32,
    #[arg(long)]
    daemon: bool,
}