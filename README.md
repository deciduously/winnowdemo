# Winnow state machine demo

[Documentation](https://deciduously.github.io/winnowdemo/winnow_sm/index.html)

MVP to spec, minus graphics - just outputs to console for now.

## Dependencies

- [Rust](https://rustup.rs/) Stable

## Build

```
$ cd winnowdemo
$ cargo run
```

Optionally pass a filename to specify the input file: `cargo run -- otherFile.txt` or `winnowdemo.exe otherFile.txt`. If invoked with no arguments, will default to `input.txt`. Any extra trailing args are ignored if present.

Also available:

- `cargo test` - run tests.
- `make docs` - build the documentation and deploy to `docs/`
- `make deploy` - create a compressed Linux build
- `release-win.ps1` - create a compressed Windows build

## Crates

- [Pest](https://pest.rs) - parsing
- [pretty_assertions](https://crates.io/crates/pretty_assertions) - Test output format helper
