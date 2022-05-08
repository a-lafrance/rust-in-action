use std::{fs::File, io::{self, BufRead, BufReader}};
use clap::Parser;
use regex::Regex;

#[derive(Parser)]
#[clap(about, author, version)]
struct Args {
    #[clap(help = "The pattern to search for")]
    pattern: String,

    #[clap(help = "Input file to search")]
    input: Option<String>,

    #[clap(short = 'n', help = "Show line numbers with matched lines")]
    show_line_numbers: bool,

    #[clap(short = 'c', default_value_t = 0, help = "Show context around matched lines")]
    show_context: usize,
}

fn process_lines(rd: impl BufRead, re: Regex, args: &Args) {
    let text = rd.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let matches = text.iter()
        .enumerate()
        .filter_map(|(i, line)| re.find(line).map(|_| i))
        .collect::<Vec<_>>();

    for (i, line) in text.iter().enumerate() {
        for j in matches.iter().copied() {
            let lower_bound = j.saturating_sub(args.show_context);
            let upper_bound = j + args.show_context;

            if i >= lower_bound && i <= upper_bound {
                if args.show_line_numbers {
                    println!("{}: {}", i, line);
                } else {
                    println!("{}", line);
                }
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let re = Regex::new(&args.pattern).unwrap();

    match args.input {
        Some(ref f) => {
            let infile = File::open(f).unwrap();
            let rd = BufReader::new(infile);
            process_lines(rd, re, &args);
        },

        None => {
            let stdin = io::stdin();
            let rd = stdin.lock();
            process_lines(rd, re, &args);
        },
    }
}
