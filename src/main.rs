use clap::Parser;

/// Cross platform library injector written in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Path to the target library
   #[arg(short, long)]
   library: String,

   /// Name of the target process
   #[arg(short, long)]
   process: String,
}

fn main() {
    let args = Args::parse();
}
