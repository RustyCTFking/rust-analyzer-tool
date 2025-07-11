# Rust Analyzer Tool

A simple static code analyzer written in Rust. Scans `.rs` files for unsafe patterns such as `unwrap()` usage, unused `mut`, and TODOs in comments.

## Features
- Flags `unwrap()` usage
- Detects unsafe blocks
- Identifies commented-out code
- Outputs log to console and markdown report

## Usage
```bash
cargo run path/to/target.rs
