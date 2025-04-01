// BORROWING e References:

/*
    In Rust, per evitare di perdere dei dati DELLO HEAP che si stanno usando quando li passo ad una funzione
    ( questo per il discorso che essendo nello heap si lavora di puntatori e quindi una volta usciti dallo scope
    vengono deallocati automaticamente) e per evitare di returnare ogni volta una variabile nuova del puntatore,

    è comodo usare i referimenti o references, ovvero creo un riferimento che come s1 punta alla stessa zona dell'heap
    ma senza ownership, infatti i references non posso andare a modificare il valore a cui puntano perchè non lo ownano.

    Il processo di creazione di references è detto: BORROWING.
*/

fn main() {
    let s1 = String::from("hello"); //String nello heap

    let len = funzione_es(&s1); //creo reference

    println!("s1 -> {s1} è lunga: {len}");

    secondaparte();
}

fn funzione_es(s: &String) -> usize {
    s.len()
}

// - - - - - - -- - - - - - - - - -

/*
    Per modificare i valori borrowati, devo "autorizzarlo" tramite mutable.
    REGOLA: se esiste una mutable reference per un valore, non posso esistere altre
            reference di nessun tipo (per evitare race condition)!
*/
fn secondaparte() {
    let mut s = String::from("hello"); //rendo un valore mutable

    change(&mut s); // creando una mutable reference
}

// gli passo una mutable reference String
fn change(some_string: &mut String) {
    some_string.push_str(", world"); // cambio la stringa s aggiungendo una parola
    println!("stringa -> {some_string}");
}

/* QUINDI:

    - In ogni momento, o hai una sola mutable reference e basta per valore, oppure quante non mutable vuoi.
    - References devono sempre essere valide.
*/



// (equivalente di malloc su c, ma sicuro)
let x = Box::new(42);
println!("x vale: {x}");