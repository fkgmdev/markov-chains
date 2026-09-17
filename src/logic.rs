use itertools::Itertools;
use std::{
    collections::HashMap,
    fs,
    io::{self, Write, stdout},
};

use crate::{Profile, analyze::analyze};
pub fn is_vowel(a: char, vowels: &str, consonants: &str) -> bool {
    let lower = a.to_lowercase().next().unwrap_or(a);
    if vowels.contains(lower) {
        return true;
    } else if consonants.contains(lower) {
        return false;
    }
    false
}

pub fn list_add(paths: &[String]) {
    let mut list = fs::read_to_string("list.txt").unwrap_or_default();
    for path in paths {
        let pathsplit: Vec<&str> = path.split("/").collect();
        let language = pathsplit[1];
        let contents = format!("{path}-{language}\n");
        list.push_str(&contents);
    }
    fs::write("list.txt", list).unwrap();
}

pub fn cosine_similarity(a: &[f64; 9], b: &[f64; 9]) -> f64 {
    let dot_product: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f64 = a.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot_product / (norm_a * norm_b)
}

pub fn gen_data(vowels: &str, consonants: &str, path: &str) -> Result<Vec<Profile>, io::Error> {
    let list = fs::read_to_string(path)?;
    let length = list.lines().try_len().unwrap_or(0);

    let mut results = Vec::new();
    for (index, line) in list.lines().enumerate() {
        if let Some((sample_path, language)) = line.trim().split_once("-") {
            let sample = fs::read_to_string(sample_path)?;
            print!(
                "Processing: [{}/{}] {} {}: ",
                index + 1,
                length,
                sample_path,
                language
            );
            let _ = stdout().flush();

            results.push(analyze(vowels, consonants, &sample, language));
            println!();
        }
    }

    Ok(results)
}

pub fn avg_profiles(profiles: Vec<Profile>) -> Vec<Profile> {
    // let raw_json = fs::read_to_string(path).unwrap();
    // let profiles: Vec<Profile> = serde_json::from_str(&raw_json).unwrap();
    println!("Averaging profiles...");
    let mut map: HashMap<String, (Profile, usize)> = HashMap::new();

    for sample in profiles {
        map.entry(sample.name.clone())
            .and_modify(|(acc, count)| {
                acc.vvvp += sample.vvvp;
                acc.vvcp += sample.vvcp;
                acc.vcvp += sample.vcvp;
                acc.vccp += sample.vccp;
                acc.cvvp += sample.cvvp;
                acc.cvcp += sample.cvcp;
                acc.ccvp += sample.ccvp;
                acc.cccp += sample.cccp;
                acc.cvrp += sample.cvrp;
                *count += 1;
            })
            .or_insert((sample, 1));
    }

    map.into_iter()
        .map(|(name, (mut total, count))| {
            let n = count as f64;
            total.name = name;
            total.vvvp /= n;
            total.vvcp /= n;
            total.vcvp /= n;
            total.vccp /= n;
            total.cvvp /= n;
            total.cvcp /= n;
            total.ccvp /= n;
            total.cccp /= n;
            total.cvrp /= n;
            total
        })
        .collect()
}
