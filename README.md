# brainfk-rs

A brainfuck to x86_64 assembly written in Rust.

## Usage

```sh
Usage: brainfk-rs [OPTIONS] <INPUT_PATH>

Arguments:
  <INPUT_PATH>  Brainfuck source file

Options:
  -o, --output-path <OUTPUT_PATH>  Output path
  -O                               Enable optimizations
  -S, --assembly                   Output generated assembly
      --keep-files                 Keep intermediate files
      --ast                        Print generated AST
  -h, --help                       Print help
  -V, --version                    Print version
```

## TODO

* Better error handling
* Remove `as` and `gcc` dependencies and start using `nasm` or `fasm`
* Allow outputting as object file
