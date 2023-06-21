# rs-sudoku

[![Build Status](https://github.com/elliottneilclark/rs-sudoku/workflows/build/badge.svg)](https://github.com/elliottneilclark/rs-sudoku/actions)
[![Docs](https://docs.rs/rs_sudoku/badge.svg)](https://docs.rs/rs_sudoku/)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Welcome to `rs-sudoku`, a comprehensive exploration into Sudoku solving algorithms, written entirely in Rust. This project serves as an educational resource for those interested in both Rust as a language and Sudoku as a problem space. It aims to implement explainable Sudoku solving techniques, and it uses the powerful Criterion benchmarking library to evaluate performance.

## Features

- Implementation of various Sudoku solving techniques.
- Detailed explanations and documentation for each solving technique.
- Performance evaluation using Criterion benchmarking.

## Prerequisites

To build and run `rs-sudoku`, you need to have the Rust programming language and its package manager, Cargo, installed on your system. If you haven't installed Rust and Cargo yet, you can download it from [here](https://www.rust-lang.org/tools/install).

## Getting Started

To get a local copy up and running, follow these steps:

1. Clone the repository:

   ```
   git clone https://github.com/elliottneilclark/rs-sudoku.git
   ```

2. Navigate into the cloned directory:

   ```
   cd rs-sudoku
   ```

3. Build the project:

   ```
   cargo test
   cargo build --release
   ```

## Documentation

Documentation for `rs-sudoku` is hosted on `docs.rs`. You can access it [here](https://docs.rs/rs_sudoku/). The documentation provides a detailed explanation of each Sudoku solving technique implemented in the project.

## Benchmarking

`rs-sudoku` uses the Criterion benchmarking library for Rust to evaluate the performance of the Sudoku solving techniques. To run the benchmarks:

1. Navigate to the project's root directory:

   ```
   cd rs-sudoku
   ```

2. Run the benchmarks:

   ```
   cargo bench
   ```

The results will be displayed in your terminal and stored in the `target/criterion` directory.

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated. If you have a feature request, or you want to propose changes, please open an issue on the [GitHub repository](https://github.com/elliottneilclark/rs-sudoku).

## License

This project is licensed under the Apache License, Version 2.0. See the `LICENSE` file for details.
