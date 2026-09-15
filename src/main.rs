#[allow(unused)]
use markov_chains::{
    analyze::{analyze, write_line},
    logic::list_add,
};
use std::env::args;
use std::fs;
use std::time::Instant;

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

    if args.len() > 2 && args[1] == "list" {
        if args[2] == "clear" {
            fs::write("list.txt", "").unwrap();
            return;
        } else if args[2] == "add" {
            list_add(&args[3..]);
            return;
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
        if let Err(e) = write_line(analyze(vowels, consonants, options), "data.txt".to_string()) {
            eprintln!("failed: {e}");
        }
    }
    let duration = start.elapsed();
    println!("Done! in {:?}", duration);
}
