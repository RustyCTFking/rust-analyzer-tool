# Rust Analyzer Tool

A simple static code analyzer written in Rust. Scans `.rs` files for unsafe patterns such as `unwrap()` usage, unused `mut`, and TODOs in comments.

## Features
- Flags `unwrap()` usage
- Detects unsafe blocks
- Identifies commented-out code
- Outputs log to console and markdown report

## Usage
Compile and Run:
```bash
cargo run target.rs
[WARNING] unwrap() at line 2: let x = Some(5).unwrap();
[TODO] todo!() at line 3: todo!();
[DANGER] unsafe block at line 4: unsafe {
