# data-uri

Simple CLI to convert a file to data URI.

## Install

```shell
cargo install --git https://github.com/0x6b/data-uri
```

## Uninstall

```shell
cargo uninstall data-uri
```

## Usage

```console
$ data-uri --help
Convert file to data URI, then output it to stdout

Usage: data-uri [OPTIONS] <FILE>

Arguments:
  <FILE>  Path to a file to convert. If text, assume UTF-8

Options:
  -m, --media-type <MEDIA_TYPE>  Internet media type specification (with optional parameters.) If none specified, will determine automagically
  -h, --help                     Print help
  -V, --version                  Print version
```

## Privacy

This CLI never sends your data to any server.

## License

This CLI is released under the GNU General Public License v3.0. This is because this CLI contains the magic definitions distributed under the GPL-2.0 or later. See [tree_magic/magic_db at mini · mbrubeck/tree_magic](https://github.com/mbrubeck/tree_magic/tree/adc1b4024cf0a44fdf9692ee721d36e5e8a03665/magic_db) and [LICENSE](LICENSE) for details.
