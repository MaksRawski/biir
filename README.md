# BIIR - Brainfuck Interpreter In Rust

[![pipeline status](https://gitlab.com/MaksRawski/biir/badges/develop/pipeline.svg)](https://gitlab.com/MaksRawski/biir/-/commits/develop)

# Installation
`cargo install biir`

# Usage
`biir filename.bf`

## Numerical mode
Available through `-n` or `--numerical-mode`.
Prints byte values instead of their ascii representations.

## Debug mode
Entered via `-d` or `--debug-mode`.
Allows the usage of `!TAPE` to print 10 nearby (already created) cells and
also prints some debug info at the end of the execution.

## Notes
* `!TAPE` can be put at any point in the program,
but will only do anything when running in debug mode.

* Every `,` (comma) will take input the same way as `getchar` in C.
It's also possible to provide it externally using pipes,
however it will exit once the whole input was consumed.

* (note to point above) Keep in mind that by default `echo` passes `\n` as last character.
This behaviour can be avoided by using its `-n` flag.


# TODO:
- [ ] `--wrapping` allow to wrap around the tape
- [ ] `--size` set tape size
- [ ] add a cool logo
- [ ] `!TAPE` doesn't work sometimes
