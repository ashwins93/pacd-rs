# Pacd

A static site generator based on shopify liquid.

[Liquid Reference](https://shopify.github.io/liquid/)

## Installation

You need `cargo` available on your `$PATH`. If you don't have Rust and Cargo you can install both with [`rustup`](https://www.rust-lang.org/tools/install)

### Option 1: Clone and Install (Recommended)

1. Clone this repository

```sh
git clone git@github.com:ashwins93/pacd-rs.git
```

2. Install with cargo

```sh
cd pacd-rs
cargo install --path .
```

### Option 2: Download release binaries

Download release binaries from [Releases](https://github.com/ashwins93/pacd-rs/releases) for your platform.

## Usage

```
Usage: pacd [OPTIONS] <SITE_DIR>

Arguments:
  <SITE_DIR>  Path to the source files

Options:
  -o, --output-dir <OUTPUT_DIR>  The path for output [default: ./build]
  -d, --data-path <DATA_PATH>    Path to the JSON data [default: ./data.json]
  -w, --watch                    Watch for directory changes
  -h, --help                     Print help
```

## Disclaimer

The tool is very early in development. If you need a general purpose static site generator based on `liquid`, check out [Cobalt](https://cobalt-org.github.io)
