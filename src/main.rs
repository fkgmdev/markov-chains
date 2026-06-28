#[allow(unused)]
use itertools::Itertools;
use std::fs;
use std::env::args;
use std::process;

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
fn main() {
    let vowels = "aeiouäöüéèêëîïôûùı";
    let consonants = "bcdfghjklmnpqrstvwxyzßñç";

    let args: Vec<String> = args().collect();
    if args.len() < 4 {
        eprintln!("Usage: words <path> Language Comment -test ");
        process::exit(1);
    }

    let a = fs::read_to_string(&args[1]).unwrap();
    let language = &args[2];
    let comment = &args[3];
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
    drop(a);
    let total = cc + cv + vc + vv;
    let ccp = (cc as f64 / total as f64) * 100.00;
    let cvp = (cv as f64 / total as f64) * 100.00;
    let vcp = (vc as f64 / total as f64) * 100.00;
    let vvp = (vv as f64 / total as f64) * 100.00;
    let ratio = (cons as f64 / vow as f64) * 100.00;
    println!("CC:{:.4}% CV:{:.4}% VC:{:.4}% VV:{:.4}% C/V:{:.4}%", ccp, cvp, vcp, vvp, ratio);
    if args.len() != 5 || args[4] != "-print" {
        let write = format!("{language} CC:{:.4}% CV:{:.4}% VC:{:.4}% VV:{:.4}% C/V:{:.4}% Comments: {comment}\n", ccp, cvp, vcp, vvp, ratio);
        let write = fs::read_to_string("data.txt").unwrap() + &write;
        fs::write("data.txt", write).unwrap();
    }
}
