/*
ERROR HANDLING
Rust non ha try-catch come altri linguaggi. Invece, quando una funzione può fallire,
deve restituire un Result, che obbliga il programmatore a gestire l’errore.
Così, nessun errore può essere ignorato per sbaglio.

- - - -
In Rust ci sono 2 tipi di errori:
1) IRRECUPERABILI -> panic!()
2) RECUPERABILI   -> Result<T, E>           
    
    enum Result<T, E> {            <- struttura di Result
        Ok(T),              tutto è andato bene, il valore è T
        Err(E),             qualcosa è andato male, l’errore è E

    }
- - - - 

Per gestire bene gli errori:
	•	Usa Result<T, E> per le funzioni che possono fallire.
	•	Implementa From, Display, std::error::Error per errori personalizzati.
	•	Evita unwrap() in produzione.
	•	Sfrutta l’operatore ? per propagare gli errori in modo elegante.
- - - -
*/

use std::fs::File;
use std::io::Error;

// Esempio con panic! ... irrecuperabile:
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero!");
    }
    a / b
}

// Esempio con Result, recuperabile:
// provo ad aprire un file, e potrebbe fallire. Se va bene, restituisco il File, se no restituisco l’errore.
fn open_file() -> Result<File, Error> {
    File::open("my_file.txt")
}

// Esempio gestione con match:
// Questo approccio è esplicito e sicuro. Nessun errore può passare inosservato.
match apri_file() {
    Ok(file) => println!("File aperto con successo: {:?}", file),
    Err(e) => println!("Errore nell'aprire il file: {}", e),
}

// Esempio con l'operatore '?', Propagare errori in modo elegante:
// Scrivere match ogni volta è verboso. Se vuoi solo passare l’errore a chi 
// chiama la funzione, l’operatore ? è la forma elegante per farlo:
fn leggi_file() -> Result<String, io::Error> {
    let mut file = File::open("config.txt")?; // se fallisce, restituisce Err e esce
    let mut contenuto = String::new();
    file.read_to_string(&mut contenuto)?;     // stesso discorso
    Ok(contenuto)
}

/* Esempio errore personalizzato
   Quando una funzione può fallire per più motivi diversi (es. I/O, parsing, logica tua…),
   è utile creare un tuo tipo di errore.

   Così puoi:
	•	restituire un Result<T, MyError>
	•	usare ? anche con errori standard (grazie a From)
	•	stampare messaggi chiari (grazie a Display)
*/

// PASSO 1, Definiamo un enum:
#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Parsing(std::num::ParseIntError),
    MancanzaConfig,
}

// PASSO 2, Implementiamo From per poter usare ?:
impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        MyError::Io(e)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(e: std::num::ParseIntError) -> Self {
        MyError::Parsing(e)
    }
}

// PASSO 3, Implementiamo Display (per messaggi di errore leggibili):
use std::fmt;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "Errore di I/O: {}", e),
            MyError::Parsing(e) => write!(f, "Errore di parsing: {}", e),
            MyError::MancanzaConfig => write!(f, "File di configurazione mancante"),
        }
    }
}

impl std::error::Error for MyError {}

// ESEMPIO COMPLETO:
fn carica_config() -> Result<i32, MyError> {
    let mut file = File::open("config.txt")?; // usa From per convertire automaticamente
    let mut contenuto = String::new();
    file.read_to_string(&mut contenuto)?;
    
    if contenuto.trim().is_empty() {
        return Err(MyError::MancanzaConfig);
    }

    let valore = contenuto.trim().parse::<i32>()?; // se parsing fallisce, viene convertito
    Ok(valore)
}


// evitare "unwrap" in produzione perchè è come non gestire errori
