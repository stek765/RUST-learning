// VECTORS, STRINGS, HASHMAPS: (collections con dimensione variabile)

// Vector contiene elementi dello STESSO TIPO.
fn vector() {
    // creare un nuovo vettore:
    let v: Vec<i32> = Vec::new();

    // assegnare valori con !:
    let v = vec![1, 2, 3];

    // aggiungere elementi:
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    // leggere elementi del vector: 
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];        
    println!("The third element is {third}");     // Un errore qui fa crushare il programma

    let thirds: Option<&i32> = v.get(2);
    match third{
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),     // Un errore qui viene gestito meglio
    }

    // iterare sui valori del vettore:
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    //oppure:
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // TRICK: posso mettere diversi tipi all'interno dello stesso "tipo" usando enum:
    enum SpreadsheetCell {
        Int(i32), 
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ]

    // uscire dallo scope libera (free) il vettore.

    // non posso avere una reference mutable e una immutable all'interno dello scope
    // questo per evitare di avere una reference che ad esempio punta alla testa del vettore
    // ma poi aggiungendo con push un nuovo elemento in coda, l'intero vettore viene spostato
    // interamente da un'altra parte nell'heap per avere lo spazio necessario per avere tutti gli elementi consecutivi in memoria, 
    // lasciando il primo reference della testa non aggiornato.. quindi NO!
    // - - - - 
}

// String, NON POSSO USARE INDICI sulle stringhe, devo usare "nth", n-esimo carattere se c'è..
fn stringhe() {
    // create a New String:
    let mut s = String::new();

    // to_string method (trasforma in tipo stringa):
    let data = "initial content";
    let s = data.to_string();

    let s = "initial contents".to_string();  //oppure direttamente
    let s = String::from("initial content"); //altro metodo, esattamente uguale
   
    // appendere a una stringa con "push_str":
    let mut s = String::from("foo");
    s.push_str("bar");

    // concatenazione con + =>(fn add(self, s: &str) -> String):
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 can no longer be used perchè rust non duplica la stringa per non sprecare memoria  
    
    // format! :
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1} - {s2} - {s3}") //invece di s1 + "-" + &s2 + "-" + &s3 che confonde visivamente;

    // ci sono un sacco di API utili per le strings. (anche per hashmaps)
}

// HashMaps<K,V> 
use std::collection::HashMap;
fn hasmaps() {
    // creare una nuova Hashmap:
    // HashMap<K, V> => questo per dire che possono avere qualsiasi tipo ma poi va mantenuto per tutta la tabella
    let mut scores = HashMap::new();

    // inserire valori:
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // accedere a valori dell'HashMap:
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // cerca team_name nella HashMap, copia il valore se esiste, altrimenti restituisce 0.

    // aggiungere una chiave solo se non esiste:
    scores.entry(String::from("Yellow")).or_insert(50); // usato anche come tabella di frequenza
}