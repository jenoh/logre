use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::seq::SliceRandom; // Add this to your dependencies in Cargo.toml
use rand::thread_rng;

pub struct Game {
    pub pointer: usize,
    pub sentence: String,
    pub sentence_size: usize,
    pub letters: Vec<char>
}

fn create_sentence() -> String {
    let file = File::open("src/random-words.txt").unwrap();

    let reader = BufReader::new(file);
    // Collect all lines into a vector.
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    // Shuffle the lines.
    let mut rng = thread_rng();
    let mut shuffled_lines = lines.clone();
    shuffled_lines.shuffle(&mut rng);

    // Select the first 15 lines.
    shuffled_lines[..4].to_vec().join(" ")
}

impl Default for Game {
    fn default() -> Self {
        let random_sentence = create_sentence();
        Self { pointer: 0, sentence: random_sentence.clone(), sentence_size: random_sentence.clone().len(), letters: random_sentence.clone().to_string().chars().collect() }
    }
}