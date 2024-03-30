# BIIR - Brainfuck Interpreter In Rust

[![tests](https://gitlab.com/MaksRawski/biir/badges/master/pipeline.svg)](https://gitlab.com/MaksRawski/biir/-/commits/master)
[![coverage](https://codecov.io/gh/MaksRawski/biir/branch/master/graph/badge.svg?token=6YIXCT104V)](https://codecov.io/gh/MaksRawski/biir)

# Installation
`cargo install biir`

# Usage
`biir filename.bf`

## Numerical mode
Available through `-n` or `--numerical-mode`.
Prints byte values instead of their ascii representations.

## Notes
- You can put `!TAPE` at any point in your program to print 10 nearby (already created) cells

- Every `,` (comma) will take input the same way as `getchar` in C.
It's also possible to provide it externally using pipes,
however it will exit once the whole input is consumed.

- (note to point above) Keep in mind that by default `echo` passes `\n` as last character.
This behaviour can be avoided by using its `-n` flag.

# TODO:
- [ ] add a cool logo (generate something like "beer exploding head emoji" with stable-diffusion)
- [ ] repl!
