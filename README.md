# Bonsai 🪴

A simple, yet beautiful, `YOM` plugin designed to show you symbols on the left hand side as commands run!

```text
▙ foo

▚ /foo

▌ read

▞ /foo
```

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
```
and you're done!

<p align="center">
	<a href="https://github.com/G0o53/bonsai/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=363a4f&colorB=b7bdf8"/></a>
</p>

