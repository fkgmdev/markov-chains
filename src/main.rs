#[allow(unused)]
use itertools::Itertools;
use std::{fs, process};
use std::env::args;
use std::time::Instant;

fn is_vowel(a: char, vowels: &str, consonants: &str) -> bool {
    let lower = a.to_lowercase().next().unwrap_or(a);
    if vowels.chars().contains(&lower) {
        return true
    }
    else if consonants.chars().contains(&lower) {
        return false
    }
    false
}

fn list_add(paths: &[String]) {
    
    let mut list = fs::read_to_string("list.txt").unwrap_or_default();
    for path in paths {
        let pathsplit: Vec<&str> = path.split("/").collect();
        let language = pathsplit[1];
        let contents = format!("{path}-{language} \n");
        list.push_str(&contents);
    }
    fs::write("list.txt", list).unwrap();
    process::exit(0);
}

fn analyze(vowels: &str, consonants: &str, args: Vec<&str>) {
    let a = fs::read_to_string(&args[0]).unwrap();
    let language = &args[1];
    let fullpath: Vec<&str> = args[0].split("/").collect();
    let path = fullpath[2];
    let mut cc = 0;
    let mut cv = 0;
    let mut vc = 0;
    let mut vv = 0;
    let mut cons = 0;
    let mut vow = 0;
    for (a1, a2) in a.chars().tuple_windows() {
        if a1.is_alphabetic() && a2.is_alphabetic() {
            match (is_vowel(a1, &vowels, &consonants), is_vowel(a2, &vowels, &consonants)) {
                (false, false) => {cc += 1;}
                (false, true) => {cv += 1;}
                (true, false) => {vc += 1;}
                (true, true) => {vv += 1;}
            }
        }
    }
    for char in a.chars() {
        if char.is_alphabetic() {
            if is_vowel(char, vowels, consonants) {
                vow += 1;
            }
            else {
                cons += 1;
            }
        }
    }
    let total = cc + cv + vc + vv;
    let ccp = (cc as f64 / total as f64) * 100.00;
    let cvp = (cv as f64 / total as f64) * 100.00;
    let vcp = (vc as f64 / total as f64) * 100.00;
    let vvp = (vv as f64 / total as f64) * 100.00;
    let ratio = (cons as f64 / vow as f64) * 100.00;
    println!("CC:{:.4}% CV:{:.4}% VC:{:.4}% VV:{:.4}% C/V:{:.4}%", ccp, cvp, vcp, vvp, ratio);
    let write = format!("{language} CC:{:.4}% CV:{:.4}% VC:{:.4}% VV:{:.4}% C/V:{:.4}% Path: {path}\n", ccp, cvp, vcp, vvp, ratio);
    let write = fs::read_to_string("data.txt").unwrap() + &write;
    fs::write("data.txt", write).unwrap();
}

fn main() {
    let start = Instant::now();
    let vowels = "aeiouäöüéèêëîïôûùı";
    let consonants = "bcdfghjklmnpqrstvwxyzßñç";

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
        }
        else if args[2] == "add" {
            list_add(&args[3..length]);
            process::exit(0);
        }
    }

    let length = list.len();
    for (index, line) in list.iter().enumerate() {
        let options: Vec<&str> = line.split("-").collect();
        print!("Processing: [{}/{}] {} {}: ", index, length, &options[0], &options[1]);
        analyze(vowels, consonants, options);
    }
    let duration = start.elapsed();
    println!("Done! in {:?}", duration);
}
