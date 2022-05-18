# deduper

It's a simple program that helps you to delete duplicated files from a chosen folder. It uses crc32 checksum checking to find out duplicates.

## Usage

```
dedup 0.1.0
rzru <razzaru@yandex.ru>
Deletes duplicated files in chosen folder

USAGE:
    deduper <PATH>

ARGS:
    <PATH>    

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

**NOTE**: If you are using debug version with `cargo run` - pass arguments after `--` like this `cargo run -- ~/Downloads`

## License

[MIT](https://opensource.org/licenses/MIT)