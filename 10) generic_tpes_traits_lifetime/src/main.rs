/* GENERIC TYPES, TRAITS AND LIFETIME
   
- Generic types rappresentano dei placeholder per il tipo del dato ed evitano duplicazioni di codice
- Un trait è come un’interfaccia: definisce un insieme di metodi che i tipi possono implementare.

- I lifetimes servono a dire a Rust quanto dura una referenza (&T), così da evitare dangling pointers (riferimenti a roba distrutta). 
  (Di solito Rust capisce tutto da solo, ma a volte serve dirglielo esplicitamente.)
*/

// - - - - - - - - - - - - - - - - - 
// GENERIC TYPES:

// generic type su una def. di funzione:
fn generics<T>(list: &[T]) -> &T {}

// gen. type su una def. di struct:
struct Point<T, U> {
    x: T;
    y: U;
}
let intgr = Point {x: 5, y: 2.3};

// gen. type su una def. di Enum:
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// gen. type su una def di Metodo (oggetti):
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// - - - - - - - - - - - - - - - - - 
// TRAITS:
// Serve a dire: “qualsiasi tipo che implementa questo trait può essere usato qui”.

// Esempio:
trait Salutabile {
    fn saluta(&self);   //  Questo dice: “qualunque tipo che voglia essere Salutabile, 
}                       //                deve avere un metodo saluta().”

// COME USARE I TRAIT: 
// Puoi scrivere funzioni che accettano qualunque tipo che implementa un trait.
fn presenta(essere: &impl Salutabile) {
    essere.saluta();
}

fn presenta<T: Salutabile>(essere: &T) {            // oppure, in modo equivalente
    essere.saluta();
}



// Puoi dare un comportamento base, sovrascrivibile dal tipo:
// Chi implementa il trait non è obbligato a definire saluta(), a meno che voglia personalizzarlo.
trait Salutabile {
    fn saluta(&self) {
        println!("Ciao da qualcosa di generico!");
    }
}


// Supponiamo di avere una struct:
struct Persona {
    nome: String,
}
// Puoi implementare il trait così:
impl Salutabile for Persona {
    fn saluta(&self) {
        println!("Ciao, sono {}!", self.nome);
    }
}



// • Esempio con polimorfismo (trait + più tipi):
struct Cane;
struct Gatto;

trait Verso {
    fn fai_verso(&self);
}

impl Verso for Cane {
    fn fai_verso(&self) {
        println!("Bau!");
    }
}

impl Verso for Gatto {
    fn fai_verso(&self) {
        println!("Miao!");
    }
}

// - - - - - - - - - - - - - - - - - 
// LIFETIMES (si usa quando ho piu reference in input e una reference in output)
// Rust non ha garbage collector. Quindi, per evitare reference rotte (tipo puntatori a 
// memoria liberata), controlla che ogni & viva abbastanza.


// Caso 1 - Nessun lifetime esplicito richiesto
// Funzione semplice che prende un riferimento e lo stampa.
// Non serve dire a Rust quanto vive il riferimento: lo capisce da solo.
fn stampa(s: &str) {
    println!("Hai scritto: {}", s);
}

// Caso 2 - Lifetime implicito: Rust capisce da solo
// Restituiamo una parte della stringa.
// Siccome c'è solo un parametro e un ritorno, Rust deduce che il valore restituito
// deve vivere quanto il parametro.
fn primo_carattere(s: &str) -> &str {
    &s[0..1]
}

// ❌ Caso 3 - Errore: Rust non sa chi dei due input vivrà di più
// Questo codice darebbe errore perché Rust non può sapere se il valore
// restituito viene da `a` o da `b`, e se vivrà abbastanza.
/*
fn piu_lunga(a: &str, b: &str) -> &str {
    if a.len() > b.len() { a } else { b }
}
*/

// ✅ Versione corretta - Lifetime esplicito
// Diciamo a Rust: il riferimento restituito vive almeno quanto `'a`,
// che è il lifetime comune tra `a` e `b`.
fn piu_lunga<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}




// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// SUMMARIO DI TUTTO INSIEME:  In parole semplici:“Dammi due stringhe e un annuncio. 
//                             Ti stampo l’annuncio e ti restituisco la stringa più lunga. 
//                             Ma mi devi garantire che tutte le stringhe coinvolte vivano almeno quanto 'a.”
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

// Importiamo il trait Display, che ci serve per stampare l'annuncio
use std::fmt::Display;

// La funzione prende:
// - due stringhe slice con lo stesso lifetime 'a
// - un generico T che rappresenta l'annuncio
// La funzione restituisce una stringa slice con lo stesso lifetime 'a
fn longest_with_an_announcement<'a, T>(
    x: &'a str,  // prima stringa: vive almeno quanto 'a
    y: &'a str,  // seconda stringa: anche questa vive almeno quanto 'a
    ann: T       // annuncio generico, può essere qualsiasi tipo
) -> &'a str     // restituiamo una delle due stringhe, quindi anche lei deve vivere quanto 'a
where
    T: Display,  // ma T deve implementare Display, così possiamo stamparlo con {}
{
    // Stampiamo l'annuncio a schermo
    println!("Announcement! {ann}");

    // Confrontiamo le lunghezze e restituiamo la stringa più lunga
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("ciao");
    let s2 = String::from("buongiorno");

    // Chiamiamo la funzione passando due stringhe slice e un annuncio
    let risultato = longest_with_an_announcement(&s1, &s2, "Confronto completato!");

    // Stampiamo il risultato
    println!("La stringa più lunga è: {}", risultato);
}