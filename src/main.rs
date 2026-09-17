use clap::Parser;
use markov_chains::{
    Profile,
    analyze::detect,
    args::Cli,
    logic::{avg_profiles, gen_data},
};
#[allow(unused)]
use markov_chains::{
    analyze::{analyze, write_line},
    logic::list_add,
};
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let vowels = "aeiouäöüéèêëîïôûùıàâæœ";
    let consonants = "bcdfghjklmnpqrstvwxyzßñçğş";

    let cli = Cli::parse();

    match cli.command {
        markov_chains::args::Commands::Train { input, output } => {
            let _ = fs::write(
                output,
                serde_json::to_string(&avg_profiles(gen_data(vowels, consonants, &input).unwrap()))
                    .unwrap(),
            );
        }
        markov_chains::args::Commands::Detect {
            text,
            file,
            database,
        } => {
            let profiles: Vec<Profile> =
                serde_json::from_str(&fs::read_to_string(&database).unwrap()).unwrap();
            if let Some(t) = text {
                dbg!(detect(analyze(vowels, consonants, &t, ""), &profiles));
            } else if let Some(f) = file {
                let t = fs::read_to_string(&f).unwrap();
                dbg!(detect(analyze(vowels, consonants, &t, ""), &profiles));
            } else {
                eprintln!("no");
            }
        }
    }
    let duration = start.elapsed();
    println!("Done! in {:?}", duration);
}
