# fzq

fzq is a command-line tool for filtering fuzzy matching lines. It also offers a
library to find approximate string from a sequence. It's useful when you want to
get a summary from a log file, for example.

```sh
$ fzq many.log

# Or read from stdin
$ cat many.log | fzq

# With different string metric and threshold
$ fzq --metric Levenshtein --threshold 0.7 many.log

# See help for more options
$ fzq -h
```

## Installation


### Using cargo for Rust programmers

```sh
$ cargo install fzq
```

## License

MIT or Apache-2.0
