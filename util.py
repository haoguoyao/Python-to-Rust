import os
def generate_cargo_toml(output_dir):
    """Generate a Cargo.toml file for the Rust project."""
    cargo_content = """[package]
name = "toml_rust"
version = "0.1.0"
edition = "2021"
authors = ["Generated"]
description = "TOML parser implemented in Rust, converted from Python"

[lib]
name = "toml_rust"
path = "src/lib.rs"

[dependencies]
chrono = "0.4"
regex = "1.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
anyhow = "1.0"
lazy_static = "1.4"
"""
    
    # Create src directory if it doesn't exist
    src_dir = os.path.join(output_dir, "src")
    os.makedirs(src_dir, exist_ok=True)
    
    # Create lib.rs to import our output file
    lib_content = """//! TOML parser implemented in Rust, converted from Python

use std::collections::HashMap;
use std::io;
use thiserror::Error;

pub mod parser;

/// Custom error type for TOML parsing
#[derive(Debug, Error)]
pub enum TomlError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Parse error: {0}")]
    Parse(String),
}

pub type TomlResult<T> = Result<T, TomlError>;

// Re-export main types
pub use parser::{load, loads, TomlDecoder, TomlDecodeError};
"""
    
    lib_path = os.path.join(src_dir, "lib.rs")
    with open(lib_path, 'w', encoding='utf-8') as f:
        f.write(lib_content)
    
    # Write Cargo.toml
    cargo_path = os.path.join(output_dir, "Cargo.toml")
    with open(cargo_path, 'w', encoding='utf-8') as f:
        f.write(cargo_content)
    
    print(f" Cargo.toml written to {cargo_path}")
    print(f" lib.rs written to {lib_path}")
