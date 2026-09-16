#[allow(unused)]
use markov_chains::{
    analyze::{analyze, write_line},
    logic::list_add,
};
use std::fs;
use std::time::Instant;
use std::{
    env::args,
    io::{Write, stdout},
};

fn main() {
    let start = Instant::now();
    let vowels = "aeiouäöüéèêëîïôûùıàâæœ";
    let consonants = "bcdfghjklmnpqrstvwxyzßñçğş";

    let contents = fs::read_to_string("list.txt").unwrap();
    let list: Vec<&str> = contents.lines().collect();

    let mut results = Vec::new();

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
        if let Some((path, language)) = line.split_once("-") {
            print!(
                "Processing: [{}/{}] {} {}: ",
                index + 1,
                length,
                path,
                language
            );
            let _ = stdout().flush();
            let text = match fs::read_to_string(path) {
                Ok(str) => str,
                Err(e) => {
                    eprintln!("Skipped {path}: {e}");
                    continue;
                }
            };
            results.push(analyze(vowels, consonants, &text, language));
            // if let Err(e) = write_line("data.txt".to_string()) {
            //     eprintln!("failed: {e}");
            // }
        }
    }

    let json = serde_json::to_string(&results).unwrap();
    if let Err(e) = fs::write("data.json", json) {
        eprintln!("Failed json write: {e}");
        return;
    }
    let duration = start.elapsed();
    println!("Done! in {:?}", duration);
}
