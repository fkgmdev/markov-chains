#[allow(unused)]
use itertools::Itertools;
use markov_chains::{analyze::analyze, logic::list_add};
use std::env::args;
use std::time::Instant;
use std::{fs, process};

fn main() {
    let start = Instant::now();
    let vowels = "aeiouäöüéèêëîïôûùıàâæœ";
    let consonants = "bcdfghjklmnpqrstvwxyzßñçğş";

    let contents = fs::read_to_string("list.txt").unwrap();
    let list: Vec<&str> = contents.lines().collect();

    let args: Vec<String> = args().collect();
    if args.len() == 2 && args[1] == "redo" {
        fs::write("data.txt", "").unwrap();
    }
    let length = args.len();

    if args.len() > 2 && args[1] == "list" {
        if args[2] == "clear" {
            fs::write("list.txt", "").unwrap();
            process::exit(0);
        } else if args[2] == "add" {
            list_add(&args[3..length]);
            process::exit(0);
        }
    }

    let length = list.len();
    for (index, line) in list.iter().enumerate() {
        let options: Vec<&str> = line.split("-").collect();
        print!(
            "Processing: [{}/{}] {} {}: ",
            index + 1,
            length,
            options[0],
            options[1]
        );
        analyze(vowels, consonants, options);
    }
    let duration = start.elapsed();
    println!("Done! in {:?}", duration);
}
