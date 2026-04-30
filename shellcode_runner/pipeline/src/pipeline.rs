use std::process::Command;
use lib::crypt::Algo;
use clap::Parser;

#[derive(Parser)]

struct Args {
    /// Binary to crypt
    file: String,

    /// Choose algorithm used to crypt, by default: xor
    #[arg(short, long, value_name = "ALGORITHM")]
    algo: Option<Algo>,

    /// Choose key used to crypt
    #[arg(short, long, value_name = "KEY")]
    key: Option<String>,

    ///verbose mode
    #[arg(short, long)]
    verbose: bool,
}

fn run_encryptor(args: &Args, algo_str: &str) {
    let mut cmd = Command::new("./target/release/encryptor");
    cmd.arg(&args.file).arg("-a").arg(algo_str);
    if let Some(key) = &args.key {
        cmd.arg("-k").arg(key);
    }

    if args.verbose {
        cmd.arg("-v");
    }

    cmd.status().expect("Failed to run encryptor");
}

fn run_runner(args: &Args, algo_str: &str, enc_file: &str) {
    let mut cmd = Command::new("./target/release/runner");
    cmd.arg(&enc_file).arg("-a").arg(algo_str);
    if let Some(key) = &args.key {
        cmd.arg("-k").arg(key);
    }
    if args.verbose {
        cmd.arg("-v");
    }
    cmd.status().expect("Failed to run runner");
}

fn main() {
    let args = Args::parse();

    let algo_str = match &args.algo {
        Some(Algo::Xor) => "xor",
        Some(Algo::Aes) => "aes",
        None => "xor", //default
    };

    run_encryptor(&args, &algo_str);
    let encrypted_file = format!("{}.{}", &args.file, &algo_str);
    run_runner(&args, &algo_str, &encrypted_file);
}