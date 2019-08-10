#![warn(missing_docs)]

//! This crate is a library to find approximate string from a sequence.
//! It also offers the `fzq` command for fuzzy filter matching lines.
//!
//! ```rust
//! use fzq::{Fzq, Metric};
//!
//! let mut fzq = Fzq::new();
//! let fzq = fzq
//!     .buffer_size(100)
//!     .metric(Metric::Jaro)
//!     .threshold(0.85);
//!
//! assert_eq!(fzq.is_similar("test 1"), false);
//! assert_eq!(fzq.is_similar("test 2"), true);
//! assert_eq!(fzq.is_similar("hello"), false);
//! assert_eq!(fzq.is_similar("test 3"), true);
//! ```

use structopt::clap::arg_enum;

arg_enum! {
    /// String metrics to use for matching
    #[allow(missing_docs)]
    #[derive(Debug)]
    pub enum Metric {
        DamerauLevenshtein,
        Levenshtein,
        Jaro,
        JaroWinkler,
    }
}

fn metric_fn(m: Metric) -> fn(&str, &str) -> f64 {
    match m {
        Metric::DamerauLevenshtein => strsim::normalized_damerau_levenshtein,
        Metric::Jaro => strsim::jaro,
        Metric::JaroWinkler => strsim::jaro_winkler,
        Metric::Levenshtein => strsim::normalized_levenshtein,
    }
}

/// A struct for finding approximate strings
pub struct Fzq {
    buffer: Vec<String>,
    buffer_size: usize,
    metric_fn: fn(&str, &str) -> f64,
    threshold: f64,
}

impl Fzq {
    /// Create a new instance of Fzq with default parameters. See the source for
    /// the actual values.
    pub fn new() -> Fzq {
        Fzq {
            buffer: Vec::new(),
            buffer_size: 100,
            metric_fn: metric_fn(Metric::Jaro),
            threshold: 0.85,
        }
    }

    /// Set the buffer size
    pub fn buffer_size<'a>(&'a mut self, size: usize) -> &'a mut Fzq {
        self.buffer_size = size;
        self.buffer.truncate(self.buffer_size);
        self
    }

    /// Set the string metric to search
    pub fn metric<'a>(&'a mut self, metric: Metric) -> &'a mut Fzq {
        self.metric_fn = metric_fn(metric);
        self
    }

    /// Set the threshold to check similarity is equal or greater than it.
    /// The value is between 0.0 and 1.0, where 1.0 means an exact match.
    pub fn threshold<'a>(&'a mut self, threshold: f64) -> &'a mut Fzq {
        self.threshold = threshold;
        self
    }

    /// Check if the string is similar to buffered strings
    pub fn is_similar(&mut self, s: &str) -> bool {
        let mut is_similar = false;
        for (i, b) in (&self.buffer).iter().enumerate() {
            if (self.metric_fn)(s, b) >= self.threshold {
                self.buffer.remove(i);
                is_similar = true;
                break;
            }
        }
        self.buffer.insert(0, String::from(s));
        self.buffer.truncate(self.buffer_size);
        is_similar
    }
}
