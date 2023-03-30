# picture-viewer

A simple picture viewer for the terminal.

## Help

```
Usage: picture-viewer <COMMAND> <PATH|URL>

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

## Azure computer vision configuration
This tool uses Azure CV to generate description for an image.
You should provide foloving environment variables for this to work

- `AZURE_COMPUTER_VISION_API_KEY` - API key for Azure CV
- `AZURE_COMPUTER_VISION_API_ENDPOINT` - API endpoint for Azure CV

see more int [azure docs](https://learn.microsoft.com/en-us/azure/cognitive-services/computer-vision/quickstarts-sdk/client-library?pivots=programming-language-csharp&tabs=visual-studio#prerequisites)
on how to get these values.