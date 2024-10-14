# cpa
cpa (copy-all) is a very lightweight CLI tool to copy whole shell command output to OS-level [clipboard](https://github.com/aweinstock314/rust-clipboard).

## Prerequisites
On Linux you need the x11 library:
```shell
sudo apt-get install xorg-dev
```

## Usage
cpa is intended to use with pipe operator, for example:
```shell
cat random_file.txt | cpa
```
