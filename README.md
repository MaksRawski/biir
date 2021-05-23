# BIIR - Brainfuck Interpreter In Rust

# Installation
`cargo install biir`

# Usage
`biir filename.bf`

## Numerical mode
Available through `-n`or `--numerical-mode`.
Prints byte values instead of their ascii representations.

## Debug mode
Entered via `-d` or`--debug-mode`.
Allows the usage of `!TAPE` to print 10 nearby tape values and
also prints some debug info at the end of the execution.

## Notes
* `!TAPE` can be put at any point in the program,
but will only do anything when running in debug mode.

* Every `,` (comma) will take input the same way as `getchar` in C.
It's also possible to provide it externally using pipes,
however it will exit once all the input was consumed.


# TODO:
- [x] write a proper README
- [x] clap - argument parsing
- [x] parse files,
- [x] `-d` debug mode, prints tape at the end, enables the use of `!TAPE` to print 10 nearest values on the tape at any point of the program
- [x] `-n` numerical mode, prints values themselves instead of interpreting them as ascii.
- [ ] `-i` input file to use for stdin, could also be taken from a pipe
- [ ] write tests
- [ ] configure gitlab ci to run tests automatically
- [ ] add a CI badge to README
