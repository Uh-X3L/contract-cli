# 🧾 contract-cli

A simple, testable smart contract simulation CLI — built in Rust with SQLite and clap, extended with CSV profiling and JSON-based metadata support.

This CLI lets you:
- Simulate wallet-like smart contract behaviors: deposits, withdrawals, balance checks, transaction logs
- Profile CSV files to summarize schema, data types, row counts, and missing values
- Manage table and column metadata via JSON configs

## ✨ Features

### Smart Contract Simulation
- **Owner Identity** hashed using SHA-512 for secure, consistent IDs
- **Deposits & Withdrawals** with safe database checks and transaction logging
- **Transaction History** view (last 5 entries)
- **SQLite Storage** for persistent state
- **Safe SQL** via parameterized queries (`params![]`) to prevent injection
- **Unit Tests** covering core logic
- **CLI** built with clap

### Data Profiling & Metadata
- **CSV Profiler**: quick summary of schema & data quality
- **Polars Integration** for fast DataFrame operations (CSV, Parquet, JSON)
- **Parallel Processing** with Rayon
- **JSON Configs**: define tables (`config/tables.template.json`) and columns (`config/table_columns/*.template.json`)

## 📦 Installation

### Prerequisites
- Rust (via rustup)
- SQLite development libraries

On Debian/Ubuntu:
```bash
sudo apt update
sudo apt install libsqlite3-dev
```

### Clone & Build
```bash
git clone https://github.com/Uh-X3L/contract-cli.git
cd contract-cli
cargo build --release
```

## 🔧 Setup

1. **Config Templates**  
   Copy the provided JSON templates to real config files:
   ```bash
   cp config/tables.template.json config/tables.json
   cp config/table_columns/*.template.json config/table_columns/
   ```
2. **Environment Variables**  
   Copy `.env.example` to `.env` and fill in your values:
   ```bash
   cp .env.example .env
   ```
3. **Build & Run**  
   ```bash
   cargo run --release -- [COMMAND]
   ```

## 🚀 Usage

### Smart Contract Commands
```bash
# Check balance
cargo run -- profile --owner alice status

# Deposit funds
cargo run -- --owner alice deposit --amount 200

# Withdraw funds
cargo run -- --owner alice withdraw --amount 50

# Show history
cargo run -- --owner alice history
```

### CSV Profiling
```bash
cargo run -- profile --input path/to/file.csv [--delimiter ,]
```

#### Example
```bash
cargo run -- profile --input data/examples/sample.csv
```

Sample output:
```
Rows: 3

COLUMN             TYPE       NULLS
--------------------------------------------------
id                 Int64      0
name               Utf8       0
amount             Int64      1
notes              Utf8       1
```

## 🧪 Testing

Run all tests:
```bash
cargo test
```

Show test output:
```bash
cargo test -- --nocapture
```

## 📁 Project Structure

```
contract-cli/
├── config/
│   ├── tables.template.json
│   └── table_columns/
│       └── users.template.json
├── data/
│   └── examples/
│       └── sample.csv
├── src/
│   ├── main.rs
│   ├── contract.rs
│   ├── db/
│   │   ├── mod.rs
│   │   └── migrations.rs
│   ├── profiler.rs
│   ├── utils/
│   │   ├── hash.rs
│   │   └── display.rs
│   └── config.rs
├── tests/
│   └── integration_test.rs
├── Cargo.toml
├── .gitignore
└── README.md
```

## 🔐 Security Notes
- All SQL uses parameterized queries (`params![]`)
- Owner hashing with `sha2::Sha512` ensures collision resistance

## 👨‍🔬 Learning Highlights
- Modular Rust design with submodules
- Safe error handling with `anyhow` and `Result`
- Data profiling via Polars & parallelism with Rayon
- JSON-based config management with Serde & Chrono
- CLI design using Clap and logging via env_logger

## 🛣️ Roadmap
- JSON export of profiling results
- Parquet & JSON profiling extensions
- Caching & incremental profiling
- Integration with NoSQL metadata store
- JSON export of profiling results
- Parquet & JSON profiling extensions
- Caching & incremental profiling
- Integration with NoSQL metadata store

