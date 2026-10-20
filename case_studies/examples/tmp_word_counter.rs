#![allow(unused)]
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;


fn main() {

    // first real argument (args()[0] is the program name itself)
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: temp <file>");
            process::exit(1);
        }
    };

    // read the whole file into a String; exit with the io error if it fails
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error reading {path}: {e}");
            process::exit(1);
        }
    };

    // split, lowercase, filter out punctuation
    let words: Vec<String> = text.split_whitespace().map(
        |word| word.chars().filter(|c| !c.is_ascii_punctuation()).map(|c| c.to_ascii_lowercase()).collect()
    ).collect();

    // count words
    let mut counts: HashMap<String, i32> = HashMap::new();
    for word in words {
        *counts.entry(word).or_insert(0) += 1;
    }

    // sort by count
    let mut sorted: Vec<(String, i32)> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    for (word, count) in sorted {
        println!("{} {}", word, count);
    }
    
    // _____________________________________________________________________________________________

    
}

