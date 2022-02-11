use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
// use clap::lazy_static::lazy_static;

use clap::Parser;
// use regex::Regex;

#[derive(Debug, Parser)]
struct Opts {
    /// Path to the input file without the language extension.
    #[clap(short, long)]
    input: String,

    /// Path to the output file without the language extension.
    #[clap(short, long)]
    output: String,

    /// Language codes of all the languages that should be filtered.
    #[clap(short, long)]
    languages: Vec<String>,

    /// Minimum number of words allowed per line.
    #[clap(long, default_value_t = 3)]
    min: i32,

    /// Maximum number of words allowed per line.
    #[clap(long, default_value_t = 80)]
    max: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();

    let mut readers: Vec<_> = opts
        .languages
        .iter()
        .map(|lang| {
            let filepath = format!("{}.{}", opts.input, lang);
            BufReader::new(match File::open(&filepath) {
                Ok(file) => file,
                Err(e) => panic!("Could not open file {}: {}", filepath, e),
            }).lines()
        })
        .collect();
    let mut out_files: Vec<_> = opts
        .languages
        .iter()
        .map(|lang| {
            let filepath = format!("{}.{}", opts.output, lang);
            match File::create(&filepath) {
                Ok(file) => file,
                Err(e) => panic!("Could not open file {}: {}", filepath, e),
            }
        })
        .collect();

    let mut words: i32;
    let mut write: bool;

    let mut lines: Vec<String> = vec![String::new(); readers.len()];

    // lazy_static! {
    //     static ref RE: Regex = Regex::new(r"\s+").unwrap();
    // }

    'outer: loop {
        write = true;

        for (index, reader) in readers.iter_mut().enumerate() {
            match reader.next() {
                Some(Ok(line)) => {
                    words = line.split_ascii_whitespace().count() as i32;
                    // words = line.split_whitespace().count() as i32;
                    // words = line.matches(' ').count() as i32;
                    // words = RE.find_iter(&line).count() as i32;
                    if words < opts.min || words > opts.max {
                        write = false
                    } else {
                        lines[index] = line;
                    }
                }
                _ => break 'outer,
            }
        }

        if write {
            for (line, out) in lines.iter_mut().zip(out_files.iter_mut()) {
                line.push('\n');
                out.write_all(line.as_bytes())?;
            }
        }
    }

    Ok(())
}
