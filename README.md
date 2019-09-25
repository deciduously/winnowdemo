# Winnow state machine demo

[API Documentation](https://deciduously.github.io/winnowdemo/winnow_sm/index.html)

## Usage

Download the appropriate [release bundle](https://github.com/deciduously/winnowdemo/releases/tag/v1.4.1).  There are two demos bundled:

* Holy Grail demo: double-click the "winnowdemo" executable file.
* AdReadiness demo: double-click the "RunAdReadiness" batch file.

If you'd like to run a different input, navigate to this directory in a terminal and specify which at the command line:

```
C:\Users\me\winnowdemo> .\winnowdemo AdReadiness.txt
```

## Dependencies

To build the source code, you will need to install the [Rust](https://www.rust-lang.org/) toolchain.  First, you will need to install the [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/downloads/#other) if you do not already use Visual Studio, which can be downloaded at the bottom of the "Tools for Visual Studio 2019" section of that link.

Then, obtain [rustup](https://rustup.rs/).  This is the official toolchain manager for Rust.  It is in charge of installing components, switching between stable, beta, and nightly channels, and upgrading.  The default stable toolchain obtained by executing the installation file they provide will be sufficent to build this code.- [Rust](https://rustup.rs/) Stable

## Build

Clone the [GitHub repository](https://github.com/deciduously/winnowdemo) or download the source code from the [Release page](https://github.com/deciduously/winnowdemo/releases/tag/v1.4.1).  Execute the following commands to build the code:

```
C:\Users\me\> cd winnowdemo
C:\Users\me\winnowdemo> cargo run
```

You can pass an alternate file as well:

```
C:\Users\me\winnowdemo> cargo run -- AdReadiness.txt
```

```
$ cd winnowdemo
$ cargo run
```

Optionally pass a filename to specify the input file: `cargo run -- otherFile.txt` or `winnowdemo.exe otherFile.txt`. If invoked with no arguments, will default to `input.txt`. Any extra trailing args are ignored if present.

Also available:

- `cargo test` - run tests
- `release-win.ps1` - create a compressed Windows build

## Crates

- [Pest](https://pest.rs) - parsing
- [pretty_assertions](https://crates.io/crates/pretty_assertions) - Test output format helper
