# Command-Line MD5

A simple MD5 hasher to be run from the command line, written in Rust.

## Installation

Using Cargo, build the project with `cargo build --release`; the executable will be in /target/release, and can be moved to a relevant place/added to PATH/etc.

## Usage

Example run:

```
$ md5 "The quick brown fox jumps over the lazy dog."
9e107d9d372bb6826bd81d3542a419d6
```
