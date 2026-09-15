use itertools::Itertools;
use std::{fs, process};
pub fn is_vowel(a: char, vowels: &str, consonants: &str) -> bool {
    let lower = a.to_lowercase().next().unwrap_or(a);
    if vowels.chars().contains(&lower) {
        return true;
    } else if consonants.chars().contains(&lower) {
        return false;
    }
    false
}

pub fn list_add(paths: &[String]) {
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
