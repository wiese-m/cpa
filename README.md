# cpa
cpa (copy-all) is a very lightweight CLI tool to copy whole shell command output to OS-level [clipboard](https://github.com/aweinstock314/rust-clipboard).

## Prerequisites
On Linux you need the x11 library:
```shell
sudo apt-get install xorg-dev
```
For build you will need [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
## Build
1. Clone this repository and `cd` into it
2. Run `cargo build --release`
3. For convenience, make executable present in `$PATH`:
```shell
sudo cp target/release/cpa /usr/local/bin/
```

## Usage
cpa is intended to use with pipe operator, for example:
```shell
ls -la | cpa
```
