# Command-Line MD5

A simple MD5 hasher to be run from the command line, written in Rust.

## Installation

Using Cargo, build the project with `cargo build --release`; the binary will be in md5/target/release, and can be moved to a relevant place/added to PATH/etc.

## Usage

Example run:

```
$ md5 "The quick brown fox jumps over the lazy dog."
e4d909c290d0fb1ca068ffaddf22cbd0
```
