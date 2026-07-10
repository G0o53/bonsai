# Bonsai 🪴
![Static Badge](https://img.shields.io/badge/license-MIT-blue)

A simple, yet beautiful, `YOM` plugin designed to show you symbols on the left hand side as commands run!

```text
▙ foo

▚ /foo

▛ read

▞ /foo
```

_if you like this, give it a 🌟_

* Simply 4 things to remember, `▙` means echo, `▚` means cd, `▞` means pwd, and `▌` means read.
* Prints to `stderr`, so it won't mess up your pipes!
* Written in Rust for maximum performance.

## To install

Make sure you have Rust installed on your system, if not, go to [rustup.rs](https://rustup.rs/) and install it,
then just
```bash
git clone https://github.com/G0o53/bonsai
cd bonsai
cargo install --path .
```
and `bonsai` is on your system! to use it, simply do
```bash
hook builtins ~/.cargo/bin/bonsai
hook exit default
```
and you're done!
