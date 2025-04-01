// File: src/lib.rs
// Questo è un semplice programma Rust che cerca una parola in un file di testo.
// Il programma prende in input due argomenti: la parola da cercare e il percorso del file.
// Se la parola viene trovata, il programma stampa la riga in cui è stata trovata.

use std::fs;
use std::error::Error;

pub struct Config {
    pub word: String,
    pub path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Usage: cargo run -- word file_path".into());
        }
        let word = args[1].clone();
        let path = args[2].clone();

        Ok(Config { word, path })
    }
}

// La funzione run prende in input la configurazione e cerca la parola nel file:
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Leggo il file:
    let contents = fs::read_to_string(config.path)?;
    let results = search(&config.word, &contents);

    for (line_number, line) in results {
        println!("Line {}: {}", line_number + 1, line);
    }

    Ok(())
}
// La funzione search prende in input la parola da cercare e il contenuto del file:
// Cerca la parola e restituisce un vettore di tuple contenenti il numero di riga e il contenuto della riga.
pub fn search<'a>(word: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(word))
        .collect()
}

// La funzione search_case_insensitive cerca la parola ignorando le maiuscole/minuscole.
pub fn search_case_insensitive<'a>(word: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let word = word.to_lowercase();
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&word))
        .collect()
}

// sono dei test per verificare che le funzioni funzionino correttamente:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let word = "test";
        let contents = "\
This is a test file.
It contains multiple lines.
Some lines contain the word test.";
        let result = search(word, contents);
        assert_eq!(result, vec![(0, "This is a test file."), (2, "Some lines contain the word test.")]);
    }

    #[test]
    fn test_search_case_insensitive() {
        let word = "tEsT";
        let contents = "\
This is a test file.
It contains multiple lines.
Some lines contain the word Test.";
        let result = search_case_insensitive(word, contents);
        assert_eq!(result, vec![(0, "This is a test file."), (2, "Some lines contain the word Test.")]);
    }

    #[test]
    fn test_search_no_match() {
        let word = "absent";
        let contents = "\
This is a test file.
It contains multiple lines.
Some lines contain the word test.";
        let result = search(word, contents);
        assert!(result.is_empty());
    }

    #[test]
    fn test_search_case_insensitive_no_match() {
        let word = "absent";
        let contents = "\
This is a test file.
It contains multiple lines.
Some lines contain the word test.";
        let result = search_case_insensitive(word, contents);
        assert!(result.is_empty());
    }
}
