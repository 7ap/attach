use clap::Parser;
use std::path::Path;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod platform;

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
    assert!(
        platform::has_permissions(),
        "You must run this program as root/administrator."
    );

    // Check if library exists
    assert!(
        Path::new(&args.library).exists(),
        "The library '{}' does not exist.",
        &args.library
    );

    // Check if process exists
    unimplemented!();

    // Print some information about the library & process, maybe prompt the user if they *really* want to inject?
    unimplemented!();

    // Inject library into process
    unimplemented!();
}
