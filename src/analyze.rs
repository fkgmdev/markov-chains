use crate::is_vowel;
use itertools::Itertools;
use std::fs;

pub fn analyze(vowels: &str, consonants: &str, args: Vec<&str>) {
    let a = fs::read_to_string(args[0]).unwrap();
    let language = &args[1];
    let fullpath: Vec<&str> = args[0].split("/").collect();
    let path = fullpath[2];
    let mut vvv = 0;
    let mut vvc = 0;
    let mut vcv = 0;
    let mut vcc = 0;
    let mut cvv = 0;
    let mut cvc = 0;
    let mut ccv = 0;
    let mut ccc = 0;

    let mut cons = 0;
    let mut vow = 0;
    for (a1, a2, a3) in a.chars().tuple_windows() {
        if a1.is_alphabetic() && a2.is_alphabetic() && a3.is_alphabetic() {
            match (
                is_vowel(a1, vowels, consonants),
                is_vowel(a2, vowels, consonants),
                is_vowel(a3, vowels, consonants),
            ) {
                (true, true, true) => vvv += 1,
                (true, true, false) => vvc += 1,
                (true, false, true) => vcv += 1,
                (true, false, false) => vcc += 1,
                (false, true, true) => cvv += 1,
                (false, true, false) => cvc += 1,
                (false, false, true) => ccv += 1,
                (false, false, false) => ccc += 1,
            }
        }
    }
    for char in a.chars() {
        if char.is_alphabetic() {
            if is_vowel(char, vowels, consonants) {
                vow += 1;
            } else {
                cons += 1;
            }
        }
    }
    let total = vvv + vvc + vcv + vcc + cvv + cvc + ccv + ccc;
    let vvvp = (vvv as f64 / total as f64) * 100.00;
    let vvcp = (vvc as f64 / total as f64) * 100.00;
    let vcvp = (vcv as f64 / total as f64) * 100.00;
    let vccp = (vcc as f64 / total as f64) * 100.00;
    let cvvp = (cvv as f64 / total as f64) * 100.00;
    let cvcp = (cvc as f64 / total as f64) * 100.00;
    let ccvp = (ccv as f64 / total as f64) * 100.00;
    let cccp = (ccc as f64 / total as f64) * 100.00;

    let ratio = (cons as f64 / vow as f64) * 100.00;

    println!(
        "VVV:{:.4}% VVC:{:.4}% VCV:{:.4}% VCC:{:.4}% CVV:{:.4}% CVC:{:.4}% CCV:{:.4}% CCC:{:.4}%",
        vvvp, vvcp, vcvp, vccp, cvvp, cvcp, ccvp, cccp,
    );
    let write = format!(
        "{language} VVV:{:.4}% VVC:{:.4}% VCV:{:.4}% VCC:{:.4}% CVV:{:.4}% CVC:{:.4}% CCV:{:.4}% CCC:{:.4}% C/V:{:.4}% Path: {path}\n",
        vvvp, vvcp, vcvp, vccp, cvvp, cvcp, ccvp, cccp, ratio,
    );
    let write = fs::read_to_string("data.txt").unwrap() + &write;
    fs::write("data.txt", write).unwrap();
}
