use clap::Parser;
use phonotax::{
    Profile,
    analyze::{analyze, detect},
    args::Cli,
    logic::{avg_profiles, gen_data},
};
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let vowels = "aeiouäöüéèêëîïôûùıàâæœ";
    let consonants = "bcdfghjklmnpqrstvwxyzßñçğş";

    let cli = Cli::parse();

    match cli.command {
        phonotax::args::Commands::Train { input, output } => {
            let _ = fs::write(
                output,
                serde_json::to_string(&avg_profiles(gen_data(vowels, consonants, &input).unwrap()))
                    .unwrap(),
            );
        }
        phonotax::args::Commands::Detect {
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
