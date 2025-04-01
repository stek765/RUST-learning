#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

// Creo un'implementazione:
// Metodo per la struct Rectangle, in modo da calcolare l'area passando come parametro se stesso. 
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height // uso self. !!!
    }

    // Creo un metodo a piu parametri, in questo caso con input self, la reference di un altra istanza di rettangolo
    // restituendo un boolean, confronta i lati.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Chiamo il metodo della struct
    println!("Area rettangolo -> {}", react1.area());
}

/*
    in C e C++, uso object.metodo         (quando chiamo il metodo direttamente sull'oggetto)
            e       object->metodo       (quando chiamo il metodo ma da un puntatore all'oggetto)

    esempio:
        p1.distance(&p2);
        (&p1).distance(&p2) => lo scrivo come => p1 -> distance(&p2)

    in RUST è fatto in automatico, non c'è il '->' .
    scrivendo: object.metodo e ci pensa lui.

    esempio: 

        p1.distance(&p2);
        (&p1).distance(&p2);

        sono uguali in rust, niente '->'.
*/

