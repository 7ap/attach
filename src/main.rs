use clap::Parser;

/// Cross platform library injector written in Rust.
#[derive(Parser)]
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

    // Check if we are root/admin
    unimplemented!();

    // Check if library exists
    unimplemented!();

    // Print some information about the library
    unimplemented!();

    // Check if process exists
    unimplemented!();

    // Print some information about the process
    unimplemented!();

    // Inject library into process
    unimplemented!();
}
