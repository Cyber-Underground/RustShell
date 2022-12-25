# RustShell

RustShell is a command line program that allows you to perform various tasks using commands on your computer.

## Features

- `AntiVM` : RustShell includes a very simple AntiVM

## Commands
RustShell offers a number of commands that you can use to interact with your computer:

- `tree` : list all files and folders in the specified directory in a tree-like way
- `clear` or `cls` : clear the command prompt
- `find` : search for a file or folder
- `where` : prints the current directory of the R(ust)Shell
- `scan` : scans the C: drive and saves every path except the blacklisted in a text file in 'C:\files\files.txt'
- `kill` : kills any process running using the PID
- `info` : gives info on the computer (disks, total memory, used memory, system type, system version, etc.)
- `help` : display a list of available commands
- `exit` : exits the program

## Usage

To run RustShell, simply run the following command file using rust or cargo:
rust:
```
$ rustc main.rs
$ ./main
```
cargo:
```
$ cargo run
```
If you don't have rust or cargo installed on your system you can follow [this tutorial](https://doc.rust-lang.org/cargo/getting-started/installation.html)
