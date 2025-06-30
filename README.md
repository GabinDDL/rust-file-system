# Rust file system RTS

The aim of this project is to create a file system, initially for the “Advanced Systems” project in the M1S2 Computer Science program at Paris Cité, this time in the rust language.

## Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Cargo](https://doc.rust-lang.org/cargo/)
- Linux or WSL (for certain system calls and locking primitives)

You can install Rust using rustup:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs
```

You may want to check the project's test coverage. To do this, you can install tarpaulin using the command:
```bash
cargo install tarpaulin
```

## Run the project

To build the project, you can use:
```bash
cargo build
```

To build and run the project, you can use:
```bash
cargo run
```

## Format the project

To format the code, you can use:
```bash
cargo fmt
```

## Test the project

To run project tests, you can use:
```bash
cargo test
```

To test the coverage of the project and generate a html file, and you have installed tarpaulin, you can use:

```bash
cargo tarpaulin --out Html 
```

## Authors

Gabin Dudillieu (@GabinDDL)
Thomas Arrous (@ThomasArr)
