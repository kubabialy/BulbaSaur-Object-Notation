# BulbaSaur Object Notation (BSON)

This is a toy project designed to demonstrate how **Abstract Syntax Trees (AST)** are built, using a custom format called **BSON** (BulbaSaur Object Notation) as the target language.

The project explores the fundamentals of compiler frontend design—specifically **lexical analysis (tokenization)** and **parsing**—across multiple programming languages.

[READ MORE](https://theliverr.substack.com/p/parsing-the-grass-type)

## Project Structure

The repository contains three independent implementations of the BSON parser:

*   **TypeScript** (`ts-bson/`): A Node.js implementation using Jest for testing.
*   **Go** (`go-bson/`): A Go implementation using the standard `testing` package.
*   **C++** (`cpp-bson/`): A C++ implementation with a custom test runner.
*   **Rust** (`rs-bson/`): A Rust implementation with tests.

## The BSON Format

BSON is a whimsical, indentation-based configuration format inspired by Pokémon. It features:
*   **Headers**: `BULBA!` (Magic bytes)
*   **Comments**: `zZz This is a comment`
*   **Key-Value Pairs**: `key ~~~~~~> value` (variable length arrows)
*   **Blocks**: `(o) block_name (o)`
*   **Lists**: `<| "item1", "item2" |>`

See `BSON_Format.md` for the full specification.

## Running the Code

### TypeScript
```bash
cd ts-bson
npm install
npm test
```

### Go
```bash
cd go-bson
go test -v
```

### C++
```bash
cd cpp-bson
g++ -o test_suite main.cpp Lexer.cpp BSONParser.cpp
./test_suite
```

### Rust
```bash
cd rs-bson
cargo test # or
cargo run --release # -- [/path/to/your/file.bson]
```
