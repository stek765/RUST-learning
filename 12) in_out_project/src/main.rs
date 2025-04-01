// IL PROGETTO CONSISTE NEL CREARE IL NOSTRO "GREP" DEL TERMINALE:

use std::env;
use ioproject::Config;

fn main(){
    let args: Vec<String> = env::args().collect();

    // creo la configurazione prendendo gli argomenti in input:
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    // stampo la parola e il file in cui cercarla:
    println!("Searching for {}, in file {}...", config.word, config.path);

    // eseguo la funzione run che cerca la parola nel file:
    if let Err(e) = ioproject::run(config) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}



    
    
