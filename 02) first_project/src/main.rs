// Progetto: l'utente deve indovinare un numero casuale generato tra 1 e 100

// Crates:
use rand::Rng;
use std::cmp::Ordering;
use std::io; // I/O library

// Main:
fn main() {
    println!("Guess the number!");

    // Genero il numero casuale
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // DEBUG LINE = println!("The secret number is: {secret_number}");

    // Loop richiesta <-> confronto
    loop {
        println!("\nPlease input your guess:");

        // let apple = "Mela"; variabile immutabile
        let mut guess = String::new(); // variabile mutabile mut.
                                       // :: -> new è una funzione associata al tipo String ( new instance )

        // si poteva scrivere anche senza importare la libreria in alto -> std::io::stdin()
        // è come scrivere -> io::stdin().read_line(&mut guess).expect("Failed to read line");  MENO LEGGIBILE
        io::stdin()
            .read_line(&mut guess) // inserisce dalla stdin -> e salva il contenuto in mutable guess
            .expect("Failed to read line");

        /*

        Questo print va a capo dopo guess perchè quando prendo dall'stdin la stringa dell'utente, lui preme invio per inviarla
        e quidni viene registrato: "numero\n":

        println!(You guessed: {guess} Tieni ti meriti una {}",apple);

        */

        // trasformo da string a int u32
        // trim taglia, parse converte tipo, expect gestisce err (in questo caso fatto a mano)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // confronto num_generato con val_utente
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Troppo piccolo\n- - - -"),
            Ordering::Greater => println!("Troppo grande\n- - - -"),
            Ordering::Equal => {
                println!("Hai indovinato");
                break;
            }
        }
    }
}
