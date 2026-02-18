use clap::Parser;

fn pi() -> f64 {
  3.1415926
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long)]
  name: String,

  /// Number of times to greet
  #[arg(short, long, default_value_t = 1)]
  count: u8,
}

fn main() {
  let args = Args::parse();

  for _ in 0..args.count {
    println!("Hello {}!", args.name)
  }
}

#[cfg(test)]
mod tests {
  use super::pi;
  use approx::assert_abs_diff_eq;
  use std::f64::consts::PI;

  #[test]
  fn pi_value() {
    assert_abs_diff_eq!(pi(), PI, epsilon = 0.001);
  }
}
