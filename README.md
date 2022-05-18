# deduper

It's a simple program that helps you to delete duplicated files from a chosen folder. It uses crc32 checksum checking to find out duplicates.

## Usage

```
dedup 0.1.1
rzru <razzaru@yandex.ru>
Deletes duplicated files in chosen folder

USAGE:
    deduper [OPTIONS] <PATH>

ARGS:
    <PATH>    Path to directory

OPTIONS:
    -h, --help          Print help information
    -i, --ignore-dir    Delete duplicate file even if files in different directories (default =
                        false)
    -r, --recursive     Should also process inner directories (default = false)
    -V, --version       Print version information
```

**NOTE**: If you are using debug version with `cargo run` - pass arguments after `--` like this `cargo run -- ~/Downloads`

## License

[MIT](https://opensource.org/licenses/MIT)