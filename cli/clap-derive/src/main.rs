/// clap example using the 'derive' feature
/// Add the following to the dependancies:
///     [dependencies]
///     clap = { version = "4.5.16", features = ["derive"] }
///     clap_derive = { version = "4.0.0-rc.1" }
/// 
/// The general idea here is that the Args struct is used to
/// specify the arguments that are passed into the program.
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    /// --name or -n becomes the argument
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    /// there is a default value, so this 
    /// is optional
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}