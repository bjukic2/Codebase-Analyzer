# Codebase Analyzer - Rust CLI Tool

This is a high-performance Rust-based code analysis tool designed to parse and analyze modern TypeScript (more languages in the future) codebases using Tree-Sitter. It extracts detailed information about functions, calculates complexity metrics, and provides insights commonly found in professional static analysis tools. The analyzer is built for speed, accuracy, and extensibility, ideal for developers who want deep insight into their code structure.

## Features

    Detects all function types:

        function declarations

        arrow functions

        function expressions

        class methods

    Extracts:

        function name

        start and end line

        total line count

    Calculates:

        Cyclomatic Complexity

        Detailed complexity contributions (if, for, &&, ||, switch, ternary)

    Uses Tree-Sitter for precise AST parsing

    Extremely fast thanks to Rust performance

    Designed to be extended with:

        Halstead metrics

        Cognitive complexity

        Dead code detection

        Call graph generation

        Multi-language support

## Tech Stack

    Language: Rust

    Parser: Tree-Sitter

    AST Grammar: TypeScript

    CLI: Native Rust binary

    Output: Terminal (JSON coming soon)

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/bjukic2/codebase-analyzer.git

cd codebase-analyzer
```

### 2. Build the project

```bash
cargo build
```

### 3. Run the analyzer

```bash
cargo run -- <path-to-your-typescript-file>

Example:

cargo run -- ./examples/sample.ts
```

## Example Output

```bash
Function: hello
Lines: 1–21 (20 total)
Complexity: 4
Contributions:

    if at line 5 (Branch introduced by if-statement.)

    && at line 5 (Logical operator '&&' introduces a branch)

    for at line 12 (Branch introduced by for-statement.)

    ternary at line 18 (Branch introduced by ternary operator.)
```

## Development

### Build in debug mode

```bash
cargo build
```

### Build in release mode

```bash
cargo build --release
```

### Run tests (coming soon)

```bash
cargo test
```

## Roadmap

    JSON output

    Halstead metrics

    Cognitive complexity

    Dead code detection

    Call graph generation

    Multi-language support (JS, Python, Go, Rust)

    HTML report generator

    VS Code extension

## Author

Made by Bruno Jukić
https://github.com/bjukic2
