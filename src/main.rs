extern crate getopts;
extern crate rand;

use getopts::Options;
use rand::{thread_rng, Rng};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::vec::Vec;

fn lines_from_file<P>(filename: &P) -> Result<Vec<String>, io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    Ok(buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect())
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} DICTIONARY_FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn random_words_from_dictionary(dictionary: &Vec<String>, words: &usize) -> Vec<String> {
    let mut rng = thread_rng();
    let max_idx = dictionary.len() - 1;

    let mut vec = Vec::with_capacity(*words);
    for _ in 0..*words {
        let n: usize = rng.gen_range(0, max_idx);
        let ref token = dictionary[n];
        vec.push(token.to_string());
    }

    vec
}

#[test]
fn test_random_words_from_dictionary() {
    let test_dict = vec![
        "This".to_string(),
        "is".to_string(),
        "a".to_string(),
        "test".to_string(),
        "vector".to_string(),
    ];
    let num_words = 3;

    let new_vec = random_words_from_dictionary(&test_dict, &num_words);
    assert_eq!(new_vec.len(), 3);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "w",
        "",
        "number of words to use in passphrase, default is 4",
        "WORDS",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f),
    };

    let words_opt = matches.opt_str("w");

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let num_words;
    match words_opt {
        Some(x) => match x.parse::<usize>() {
            Ok(n) => num_words = n,
            Err(_e) => {
                println!("Please enter an integer for the -w flag");
                return;
            }
        },
        None => num_words = 4,
    }

    let lines;

    match lines_from_file(&input) {
        Ok(n) => lines = n,
        Err(_e) => {
            println!(
                "Cannot read file {}. Please ensure it exists and you have permission to \
                 read it.",
                input
            );
            return;
        }
    }

    let passphrase = random_words_from_dictionary(&lines, &num_words).join(" ");

    println!("{}", passphrase);
}
