# Memrun

A simple in-memory executor for Linux ELF binaries. Inspired by
[memrun](https://github.com/guitmz/memrun),
[ezuri](https://github.com/guitmz/ezuri), and
[fileless-xec](https://github.com/ariary/fileless-xec).
All of these examples are written in Go, using C standard library functions. This program uses the same approach, but in Rust.

This program relies on the `memfd_create -> fexecve` pattern described
[here](https://www.guitmz.com/running-elf-from-memory/) and
[here](https://0x00sec.org/t/super-stealthy-droppers/3715).
Therefore, it can only run on Linux kernels >= 3.17 (5 Oct 2014).

Before running the in-memory binary, the process daemonizes, following the SysV approach described
[here](https://stackoverflow.com/a/38818264/5202294).