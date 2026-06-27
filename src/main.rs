use clap::Parser;
use std::fs;
use std::io::Read;

#[derive(Parser)]
#[command(name = "rarita", version, about = "RAR file extractor made with Rust")]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long, default_value = ".")]
    output: String,
}
fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("rarita v{}", env!("CARGO_PKG_VERSION"));
    }
    let Some(ref file) = cli.file else {
        println!("Usage: rarita --file <FILE> [--output <DIR>] [--verbose]");
        println!("Try --help for more information.");
        return;
    };
    if cli.verbose {
        println!("Input file: {file}");
        println!("Output directory: {}", cli.output);
    }
    if is_rar(file) {
        println!("{file} is a valid RAR archive.");
    } else {
        println!("{file} is not a RAR archive.");
    }
}
const RAR4_SIG: [u8; 7] = [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x00];
const RAR5_SIG: [u8; 8] = [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x01, 0x00];
fn is_rar(path: &str) -> bool {
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };
    let mut buf = Vec::new();
    if file.take(0x100000).read_to_end(&mut buf).is_err() {
        return false;
    }
    buf.windows(7).any(|w| w == RAR4_SIG) || buf.windows(8).any(|w| w == RAR5_SIG)
}
