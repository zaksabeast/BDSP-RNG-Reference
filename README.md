# BDSP RNG Reference

This is a command line interface for BDSP RNG, primarily used as a reference implementation and as a tool for testing.

## Building

Build with `cargo build --release`

## Usage

Run `./bdsp-rng --help` for the most up to date examples.

Here is a small, incomplete overview:

```bash
# Show the main help
$ ./bdsp-rng --help

# Individual subcommands also have help
$ ./bdsp-rng find-state --help

# There are multiple subcommands
$ ./bdsp-rng --s0 36b503fb --s1 faa95b9c --s2 a5a7ce6b --s3 7886b960 --offset 100 find-shiny --delay 88
Shiny in 5284 advances
RNG states: f42d0824 2e34fc48 e1732030 62b3e536

# Some arguments can be shortened
$ ./bdsp-rng --s0 36b503fb --s1 faa95b9c --s2 a5a7ce6b --s3 7886b960 -o 100 find-shiny -d 88
Shiny in 5284 advances
RNG states: f42d0824 2e34fc48 e1732030 62b3e536
```
