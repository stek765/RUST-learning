fn main() {
    another_function(5, 'a');

    // Statements:  sono istruzioni che fanno qualcosa ma non ritornano nessun valore
    // Expressions: ritorna un valore risultante

    // Esempio complicato:
    let y = {
        let x = 3;
        x + 1
        // senza ";" ritorna il valore             -> Expression
        // con   ";" non ritorna nulla, quindi err -> Statment
    };

    println!("y -> {y}");

    ritorna_value();

    let y = ritorna_value();
    println!("y -> {y}");
}

fn another_function(x: i32, lettera: char) {
    println!("Another function. Value of x -> {x}. Letter -> {lettera}");
}

fn ritorna_value() -> i32 {
    5
}
