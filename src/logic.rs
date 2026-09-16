use std::fs;
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
