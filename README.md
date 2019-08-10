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

## Using programmatically

Find the document on [Docs.rs](https://docs.rs/fzq). Add the following to your
`Cargo.toml`.

```toml
[dependencies]
fzq = "0.1"
```

And then, in your rust file:

```rust
use fzq::{Fzq, Metric};

let mut fzq = Fzq::new();
let fzq = fzq
    .buffer_size(100)
    .metric(Metric::Jaro)
    .threshold(0.85);

assert_eq!(fzq.is_similar("test 1"), false);
assert_eq!(fzq.is_similar("test 2"), true);
assert_eq!(fzq.is_similar("hello"), false);
assert_eq!(fzq.is_similar("test 3"), true);
```

## License

MIT or Apache-2.0
