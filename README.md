# tsk
A simple CLI task manager

## Installation

1. Download the latest binary from the [releases](https://github.com/em-s-h/xdelta_lui/releases)
page
2. Make it executable
3. Add it to a directory in your `PATH` variable

### Bash completions

1. Copy the contents of the file `tsk.sh`
2. Paste it in your .bashrc file or a file that is sourced by it

## Building from source

1. Clone the repository
```shell
git clone https://github.com/em-s-h/tsk.git
```
2. Inside the repository run `cargo b -r` for a release build

## Why?

Before tsk came to be I liked to use [please](https://github.com/NayamAmarshe/please) for managing
my tasks, but over time I started to have problems with its features, such as `move 2 6` not moving
2 to the position 6 but instead swapping 2 to 6.

So, because I don't know python, I decided to make a similar program that had the features I wanted
out of a simple task manager.
