# picture-viewer

A simple picture viewer for the terminal.

## Help

```
Usage: picture-viewer <COMMAND>

Commands:
  describe  
  show      
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

```


## Installation

```bash
$ git clone git@github.com:Light-Keeper/picture-viewer.git
$ cd picture-viewer
$ cargo build --release
```

## Usage

```bash
$ ./target/release/picture-viewer show <path to image>
$ ./target/release/picture-viewer describe <path to image>
```
