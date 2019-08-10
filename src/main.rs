use failure::ResultExt;
use fzq::Fzq;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

/// Filter fuzzy matching lines from INPUT (or standard input), writing to standard output.
#[derive(StructOpt)]
#[structopt(author = "", rename_all = "kebab_case")]
struct Opt {
    /// Print similar lines
    #[structopt(short = "D", long)]
    all_similar: bool,

    /// Buffer last N lines to filter
    #[structopt(short, long, value_name = "N", default_value = "100")]
    buffer_size: usize,

    /// Compare no more than N characters in lines
    #[structopt(short = "w", long, value_name = "N")]
    check_chars: Option<usize>,

    /// Ignore differences in case when comparing
    #[structopt(short = "i", long)]
    ignore_case: bool,

    /// Print line number with output lines
    #[structopt(short = "n", long)]
    line_number: bool,

    /// Search similar lines using the string metric
    #[structopt(
        short,
        long,
        raw(
            possible_values = "&fzq::Metric::variants()",
            case_insensitive = "true"
        ),
        default_value = "Jaro"
    )]
    metric: fzq::Metric,

    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,

    /// Avoid comparing the first N characters
    #[structopt(short, long, value_name = "N")]
    skip_chars: Option<usize>,

    /// Avoid comparing the first N fields
    #[structopt(short = "f", long, value_name = "N")]
    skip_fields: Option<usize>,

    /// Filter lines if similarity is equal or greater than the threshold.
    /// The value is between 0.0 and 1.0, where 1.0 means an exact match.
    #[structopt(short, long, default_value = "0.85")]
    threshold: f64,
}

struct ExitError(failure::Error);

impl<T: Into<failure::Error>> From<T> for ExitError {
    fn from(t: T) -> Self {
        ExitError(t.into())
    }
}

impl std::fmt::Debug for ExitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fail = self.0.as_fail();
        write!(f, "{}", fail)?;

        if let Ok(x) = std::env::var("RUST_BACKTRACE") {
            if x != "0" {
                write!(f, "\n{}", self.0.backtrace())?
            }
        }
        Ok(())
    }
}

fn read(path: &Option<std::path::PathBuf>) -> Result<Box<io::Read>, Box<io::Error>> {
    if let Some(path) = path {
        let f = File::open(&path)?;
        Ok(Box::new(f))
    } else {
        Ok(Box::new(io::stdin()))
    }
}

fn main() -> Result<(), ExitError> {
    let opt = Opt::from_args();
    let input = read(&opt.path).with_context(|e| {
        if let Some(path) = &opt.path {
            format!("{}: {}", path.display(), e)
        } else {
            e.to_string()
        }
    })?;

    let mut fzq = Fzq::new();
    let fzq = fzq
        .buffer_size(opt.buffer_size)
        .metric(opt.metric)
        .threshold(opt.threshold);

    for (i, line) in BufReader::new(input).lines().enumerate() {
        let line = line?;
        let line_to_compare = &line;

        let line_to_compare = if let Some(skip_fields) = opt.skip_fields {
            let mut fields = 0;
            let mut in_field = false;
            let mut iter = line_to_compare.char_indices().skip_while(|(_, c)| {
                if *c == ' ' || *c == '\t' {
                    in_field = false;
                    fields < skip_fields
                } else {
                    if !in_field {
                        fields += 1;
                        in_field = true;
                    }
                    true
                }
            });
            iter.next();
            if let Some((i, _)) = iter.next() {
                &line_to_compare[i..]
            } else {
                ""
            }
        } else {
            line_to_compare
        };

        let line_to_compare = if let Some(skip_chars) = opt.skip_chars {
            if let Some((i, _)) = line_to_compare.char_indices().nth(skip_chars) {
                &line_to_compare[i..]
            } else {
                line_to_compare
            }
        } else {
            line_to_compare
        };

        let line_to_compare = if let Some(check_chars) = opt.check_chars {
            if let Some((i, _)) = line_to_compare.char_indices().nth(check_chars - 1) {
                &line_to_compare[..i]
            } else {
                line_to_compare
            }
        } else {
            line_to_compare
        };

        let line_to_compare = if opt.ignore_case {
            line_to_compare.to_lowercase()
        } else {
            String::from(line_to_compare)
        };

        let is_similar = fzq.is_similar(&line_to_compare);
        if (!is_similar && !opt.all_similar) || (is_similar && opt.all_similar) {
            if opt.line_number {
                println!("{}:{}", i + 1, line);
            } else {
                println!("{}", line);
            }
        }
    }
    Ok(())
}
