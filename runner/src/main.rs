use clap::Parser;

/// Advent of Code 2025 runner
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Day number to run (e.g. 1 or 2)
    day: u32,

    /// Part number to run (1 or 2)
    #[arg(value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// Which input to use: example or real
    #[arg(value_parser = ["example", "real"], value_name = "example|real")]
    mode: String,

    /// Optional expected answer; if provided, result will be validated against it
    expected: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let use_example = matches!(cli.mode.as_str(), "example");

    let result = match solvers::run_day(cli.day, cli.part, use_example) {
        Some(r) => r,
        None => {
            eprintln!("Day {} is not implemented yet", cli.day);
            std::process::exit(0);
        }
    };

    if let Some(exp) = cli.expected {
        if result == exp {
            println!("✅ Matches expected: {}", exp);
        } else {
            eprintln!("❌ Mismatch: expected {}, got {}", exp, result);
        }
    } else {
        println!("Result: {}", result);
    }
}
