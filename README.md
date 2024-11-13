# swapall

swapall is a CLI tool used to help swap a pair of strings from stdin, and outputting to stdout.

It assumes the pair of strings are formatted as a csv file.

## Installation

Assuming you have cargo setup:

```sh
cargo install --git https://github.com/stefafafan/swapall
```

## Usage

Try `--help`

```sh
$ swapall --help

Usage: swapall <CSV_PATH>

Arguments:
  <CSV_PATH>  The filepath to the CSV file.

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Example

Let's say you have a CSV containing the following contents. (swapall assumes the CSV does *not* have a header row!)

```sh
$ cat replacements.csv
hello,bye
123,456
こんにちは,さようなら
```

Try piping a string and see how each occurrence of the first value is replaced as the second.

```sh
$ echo "hello (こんにちは) my favorite number is 123456123." | swapall ./replacements.csv
bye (さようなら) my favorite number is 456456456.
```
