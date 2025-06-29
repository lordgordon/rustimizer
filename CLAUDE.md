# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**rustimizer** is a Rust library for multi-criteria decision making. It helps users find optimal decisions by comparing multiple options across multiple criteria using mathematical optimization (L2 norm minimization).

## Essential Commands

### Development Workflow
```bash
# Install development environment
make install

# Build and test (standard workflow)
make

# Run specific operations
cargo build          # Build dev
cargo test           # Run tests
cargo clippy         # Lint code
cargo fmt --check    # Check formatting

# Release build
make release
```

### Code Quality
```bash
# Run all checks (clippy, fmt, cargo check, pre-commit)
make check

# Apply formatting fixes
make format

# Individual checks
make check-clippy
make check-fmt
make check-cargo
```

## Code Architecture

### Core Modules
- **`solver/`**: High-level problem solving interface
  - `problemdefinition.rs`: Polars DataFrame-based API
  - `solvableproblem.rs`: Core problem validation and matrix building
  - `vector.rs`: Mathematical optimization using L2 norm

- **`variables/`**: Variable type system with trait-based design
  - All variables implement `VariableProperties` trait
  - `VariableAutoscale`: Higher values are better (0-1 scaling)
  - `VariableInvertedAutoscale`: Lower values are better (inverted scaling)

### Key Patterns
- **Strategy Pattern**: Different variable types with same interface
- **Strong Typing**: Validated types (`Name`, `Values`) prevent invalid data
- **Trait-Based Polymorphism**: `VariableProperties` enables flexible variable behavior

### Dependencies
- **ndarray**: Core numerical computations and matrix operations
- **polars**: DataFrame handling for data input/manipulation
- **clap**: CLI interface (minimal implementation currently)
- **thiserror**: Custom error types

## Testing

All tests use `cargo test`. Floating-point comparisons use the `approx` crate for precision handling.

## Algorithm Overview

The decision-making process:
1. Normalize all variables to 0-1 scale (with inversion for "lower is better")
2. Build decision matrix where each row = one option
3. Compute L2 norm (Euclidean distance) for each row
4. Return option with minimum distance to optimal point

## Development Notes

- Uses Rust 2024 edition with development optimizations enabled
- Strong emphasis on type safety and validation
- Functional programming patterns with immutable data structures
- Comprehensive error handling with `Result` types throughout
