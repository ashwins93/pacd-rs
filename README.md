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

Make sure the binary `pacd` is on your `$PATH`.

```
$ pacd help
A static site generator based on shopify liquid

Usage: pacd <COMMAND>

Commands:
  build  Build your site from liquid templates
  pack   Pack your template files into an archive
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## `pacd build`

The build command copies files recursively from a given `<SITE_DIR>` into the `output-dir`. `Liquid` template files are
transformed into `html` files.

### Globals

You can supply globals for use within the template files by passing the `data-path` argument to the build command. Pacd
looks for a `json` file in the given path and provides them to you as `data` variable in your template files. For instance
if you had a data file that looks like this

```json
{
  "person": {
    "name": "John Doe"
  }
}
```

You can access this data using the liquid expression `{{ data.person.name }}`. All standard liquid operators are supported.

### Collection templates

Templates with file name having a pattern `[my_list].liquid` are special. Pacd will look for a key `my_list` inside your
json file. If that key points to an array of objects which at least has an `id` attribute, then a `html` file is generated
for each item in the array. The output has a name `<id>.html`.

**Note**: Pacd assumes `id`s within the array are unique. Files will get overwritten if they aren't unique.

For collection templates you have access to one extra global variable `{{ page.current_index }}` which will contain the
position of the object in the array being processed. Check the [example file](./examples/site/%5Bcollection%5D.liquid) to
see how the `page` global is used.

```
Build your site from liquid templates

Usage: pacd build [OPTIONS] <SITE_DIR>

Arguments:
  <SITE_DIR>  Path to the source files

Options:
  -o, --output-dir <OUTPUT_DIR>  The path for output [default: ./build]
  -d, --data-path <DATA_PATH>    Path to the JSON data [default: ./data.json]
  -w, --watch                    Watch for directory changes
  -h, --help                     Print help
```

### `pacd pack`

```
Pack your template files into an archive

Usage: pacd pack [OPTIONS] <TEMPLATE_DIR>

Arguments:
  <TEMPLATE_DIR>  Path to the source files

Options:
  -o, --output-path <OUTPUT_PATH>  The path for output [default: ./build.tar.gz]
  -h, --help                       Print help
```

Check the [examples](./examples/) directory for more usage instructions.

## Disclaimer

The tool is very early in development. If you need a general purpose static site generator based on `liquid`, check out [Cobalt](https://cobalt-org.github.io)
