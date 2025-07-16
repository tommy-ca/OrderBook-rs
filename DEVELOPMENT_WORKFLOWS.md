# Development Workflows - OrderBook-rs

This document outlines the development workflows for the OrderBook-rs project, including best practices for Python (with uv), Node.js, and Rust development.

## 🐍 Python Development Workflow (with uv)

### Prerequisites
- **uv**: Fast Python package manager and project management tool
- **ruff**: Fast Python linter and formatter (installed via uv)

### Setup

```bash
# Install uv (if not already installed)
curl -LsSf https://astral.sh/uv/install.sh | sh

# Add uv to PATH
export PATH="$HOME/.local/bin:$PATH"

# Install development tools using uv
uv tool install ruff
uv tool install pre-commit
```

### Python Binding Development

#### Building Python Bindings

```bash
# Install maturin for Python binding builds
uv tool install maturin

# Build Python bindings for development
maturin develop --features python

# Build release wheels
maturin build --release --features python
```

#### Code Quality with uv + ruff

```bash
# Lint Python code (using uvx for execution)
uvx ruff check src/bindings/tests/

# Format Python code
uvx ruff format src/bindings/tests/

# Fix linting issues automatically
uvx ruff check src/bindings/tests/ --fix

# Fix with unsafe fixes enabled
uvx ruff check src/bindings/tests/ --fix --unsafe-fixes
```

#### Testing Python Bindings

```bash
# Run Python tests
python3 src/bindings/tests/python_tests.py

# Run benchmarks
python3 src/bindings/tests/benchmark_tests.py

# Test import functionality
python3 -c "import orderbook_rs; print('Success!')"
```

### Best Practices for Python + uv

1. **Use uvx for tool execution**: Always use `uvx ruff` instead of `pip install ruff`
2. **Avoid pip or uv pip**: Use `uv tool install` for development tools
3. **Project dependencies**: Use `uv add` for project dependencies
4. **Virtual environments**: Use `uv venv` for isolated environments
5. **Lock files**: Maintain `uv.lock` for reproducible builds

#### Example Python Project Structure

```
python/
├── pyproject.toml          # Python project configuration
├── uv.lock                 # Dependency lock file
├── src/bindings/           # Python binding source
│   ├── tests/              # Python tests
│   └── python.rs           # Rust binding code
└── ruff.toml              # Ruff configuration
```

## 📦 Node.js Development Workflow

### Prerequisites
- **Node.js**: v16+ (tested with v22.16.0)
- **npm**: Package manager
- **@napi-rs/cli**: NAPI binding builder

### Setup

```bash
# Install Node.js dependencies
npm install

# Install global tools
npm install -g eslint prettier @typescript-eslint/eslint-plugin @typescript-eslint/parser
```

### Node.js Binding Development

#### Building Node.js Bindings

```bash
# Build for development
npm run build

# Build with features
npm run build -- --features nodejs

# Development build (via Makefile)
make dev-nodejs
```

#### Code Quality

```bash
# Lint JavaScript/TypeScript
eslint src/bindings/tests/*.js

# Format code
prettier --write src/bindings/tests/*.js

# Type checking (if using TypeScript)
tsc --noEmit
```

#### Testing Node.js Bindings

```bash
# Test basic functionality
node -e "const { OrderBook } = require('./index.js'); console.log('Success!');"

# Run Node.js tests (when available)
node src/bindings/tests/nodejs_tests.js
```

### Node.js Issues & Solutions

#### Known Issue: Module Loading
- **Problem**: `Module did not self-register` error with Node.js v22+
- **Cause**: NAPI version compatibility
- **Solution**: Use Node.js v16-18 or update NAPI dependencies

```bash
# Check Node.js version
node --version

# Rebuild bindings
npm run build

# Clean and rebuild
rm -f index.node && npm run build
```

## 🦀 Rust Development Workflow

### Prerequisites
- **Rust**: 1.88.0+ (latest stable)
- **Cargo**: Rust package manager
- **rustfmt**: Code formatter
- **clippy**: Linter

### Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install components
rustup component add rustfmt clippy

# Install additional tools
cargo install cargo-sort
```

### Rust Development

#### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Build with specific features
cargo build --features python
cargo build --features nodejs
```

#### Testing

```bash
# Run all tests
cargo test

# Run with specific log level
LOGLEVEL=WARN cargo test

# Run benchmarks
cargo bench
```

#### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Lint code
cargo clippy --all-targets --all-features -- -D warnings

# Fix linting issues
cargo clippy --fix --all-targets --all-features --allow-dirty --allow-staged
```

#### Documentation

```bash
# Generate documentation
cargo doc --open

# Check for missing docs
cargo clippy -- -W missing-docs
```

## 🔧 Pre-commit Hooks Configuration

### Setup with uv

```bash
# Install pre-commit using uv
uv tool install pre-commit

# Install hooks
pre-commit install

# Run on all files
pre-commit run --all-files
```

### Pre-commit Configuration (`.pre-commit-config.yaml`)

```yaml
repos:
  # Rust hooks
  - repo: https://github.com/doublify/pre-commit-rust
    rev: v1.0
    hooks:
      - id: fmt
      - id: clippy
      - id: cargo-sort

  # Python hooks using ruff (modern approach)
  - repo: https://github.com/astral-sh/ruff-pre-commit
    rev: v0.7.4
    hooks:
      - id: ruff
        args: [ --fix ]
      - id: ruff-format

  # JavaScript/TypeScript hooks
  - repo: https://github.com/pre-commit/mirrors-eslint
    rev: v9.0.0
    hooks:
      - id: eslint
        
  - repo: https://github.com/pre-commit/mirrors-prettier
    rev: v4.0.0-alpha.8
    hooks:
      - id: prettier
```

## 🚀 Makefile Targets

### Python Targets

```bash
# Development
make dev-python              # Build Python bindings for development
make python                  # Build Python bindings
make package-python          # Package Python bindings
make bench-python           # Run Python benchmarks

# Testing
make test-bindings          # Test all bindings
make quick-test            # Quick development cycle
```

### Node.js Targets

```bash
# Development
make dev-nodejs             # Build Node.js bindings for development
make nodejs                 # Build Node.js bindings
make package-nodejs         # Package Node.js bindings
make bench-nodejs          # Run Node.js benchmarks
```

### Rust Targets

```bash
# Building
make build                  # Debug build
make release               # Release build

# Testing
make test                  # Run tests
make bench                 # Run benchmarks

# Code Quality
make fmt                   # Format code
make lint                  # Lint code
make check                 # Pre-push checks
```

### Combined Targets

```bash
# All operations
make build-all             # Build everything
make test-all              # Test everything
make bench-all             # Benchmark everything
make ci                    # Full CI pipeline
```

## 📊 Performance Testing

### Python Performance

```bash
# Run comprehensive benchmarks
python3 src/bindings/tests/benchmark_tests.py

# Expected results:
# - Single Order Insertion: ~40,000 ops/sec
# - Price Queries: ~110,000 ops/sec
# - Mixed Operations: ~46,000 ops/sec
# - Overall Average: ~70,000 ops/sec
```

### Rust Performance

```bash
# Run criterion benchmarks
cargo bench

# Save baseline
make bench-save

# Compare with baseline
make bench-compare
```

## 🔍 Troubleshooting

### Python Issues

1. **Import Error**: Rebuild with `maturin develop --features python`
2. **Linting Errors**: Use `uvx ruff check --fix --unsafe-fixes`
3. **Version Conflicts**: Use `uv tool upgrade ruff`

### Node.js Issues

1. **Module Loading**: Try older Node.js version (v16-18)
2. **Build Errors**: Clean with `rm index.node && npm run build`
3. **NAPI Errors**: Update `@napi-rs/cli` to latest

### Rust Issues

1. **Clippy Warnings**: Use file-level `#![allow(unsafe_op_in_unsafe_fn)]` for bindings
2. **Build Errors**: Check feature flags: `--features python` or `--features nodejs`
3. **Test Failures**: Run with `LOGLEVEL=WARN cargo test`

## 📚 Best Practices Summary

### Python with uv
- Use `uvx` for tool execution
- Use `uv tool install` for development tools
- Maintain `uv.lock` for reproducibility
- Use `ruff` for linting and formatting

### Node.js
- Keep Node.js version compatible (v16-18)
- Use ESLint + Prettier for code quality
- Test module loading after builds

### Rust
- Use `cargo fmt` and `clippy` for code quality
- Test with different feature combinations
- Document public APIs
- Use appropriate `#[allow]` attributes for binding code

### Git Workflow
- Use pre-commit hooks for quality gates
- Test all bindings before committing
- Run full CI pipeline locally: `make ci`