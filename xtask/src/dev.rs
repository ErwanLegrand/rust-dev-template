// Copyright (c) 2025 Erwan Patrick Legrand

//! Development server and utilities

use clap::Args;

#[derive(Args)]
pub struct DevArgs {
    /// Port to run the development server on
    #[arg(long, default_value = "3000")]
    port: u16,

    /// Enable hot reloading
    #[arg(long)]
    hot_reload: bool,

    /// Open browser automatically
    #[arg(long)]
    open: bool,
}

#[allow(clippy::unnecessary_wraps)]
pub fn run(_args: DevArgs) -> anyhow::Result<()> {
    println!("Starting development server...");
    println!("Development server not yet implemented");
    println!("Consider using tools like:");
    println!("   - cargo-watch for file watching");
    println!("   - trunk for web development");
    println!("   - axum for API servers");
    Ok(())
}
