use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Set the name
    #[arg(short, long)]
    name: String,
}

fn main() {
    let opts: Args = Args::parse();

    let name = opts.name;

    let mut heart = String::new();

    for y in (-15..15).rev() {
        for x in -30..30 {
            let formula = (x as f64 * 0.05).powi(2) + (y as f64 * 0.1).powi(2) - 1.0;
            if formula.powi(3) - (x as f64 * 0.05).powi(2) * (y as f64 * 0.1).powi(3) <= 0.0 {
                let idx = ((x - y) % name.len() as i32).abs() as usize;
                heart.push(name.chars().nth(idx).unwrap());
            } else {
                heart.push(' ');
            }
        }
        heart.push('\n');
    }
    print!("{}", heart);
}