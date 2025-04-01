/* AUTOMATED TESTING:
   Rust ha un sistema di test integrato nel linguaggio, "cargo test"
   e genera un report file.
*/

// scrivere un test in Rust: (viene eseguita solo da cargo test)
#[test]
fn test_somma() {
    assert_eq!(2 + 2, 4);
}

// Dove si scrivono i test?
// 1) nello stesso file del codice 
// 2) in un file separato 

// 1:
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)] // dice a Rust: “questa parte di codice si compila solo durante i test.”
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// 2: 
// Creo una cartella tests/ nella root del progetto e dentro, creo file tipo main_tests.rs
use tuo_crate_name::somma;

#[test]
fn test_somma() {
    assert_eq!(somma(1, 1), 2);
}
// I test in tests/ non hanno accesso ai moduli privati, solo alle funzioni pubbliche (pub)
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


/* 
Macro di test utili:
	•	assert!(condizione) → passa se condizione è vera
	•	assert_eq!(a, b) → passa se a == b
	•	assert_ne!(a, b) → passa se a != b
	•	panic!() → test fallisce se viene raggiunto
	•	should_panic → test che deve fallire
*/
#[test]
#[should_panic]
fn test_panico() {
    panic!("Errore volontario");
}
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// setup e teardown
// Setup = tutto ciò che prepari prima di un test.
// teardown = tutto ciò che ripulisci dopo un test. (non serve perchè lo fa rust)

// Una funzione che simula il setup di un ambiente
fn setup() -> Vec<i32> {
    println!("Inizializzazione...");
    vec![1, 2, 3]
}

#[test]
fn test_con_setup() {
    let dati = setup(); // esegui setup
    assert_eq!(dati.len(), 3); // fai il test
}
