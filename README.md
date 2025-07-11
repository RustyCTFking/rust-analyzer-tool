# 🛡️ Rust Analyzer Tool

A simple static code analysis tool written in Rust that flags potential vulnerabilities in `.rs` files such as:

- Usage of `unwrap()`  
- `todo!()` placeholders  
- `unsafe` blocks

---

## 🔧 Features

- ✅ Line-by-line scan of any Rust source file
- ⚠️ Detects risky patterns used during early dev stages
- 🧠 Terminal output for fast auditing
- 📂 Built for use in CTF prep and Web3 smart contract toolchains

---

## 🚀 Usage

### 1. Compile & Run

```bash
cargo run target.rs
