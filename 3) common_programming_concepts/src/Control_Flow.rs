fn main() {
    // IF EXPRESSION:
    let n = 2;
    println!("n è {n}");

    if n < 5 {
        println!("condizione < 5 ");
    } else if n > 5 {
        println!("condizione > 5 ");
    } else {
        println!("condizione falsa");
    }

    // Usare "if" in a let statment:
    let number = if n == 2 { 10 } else { 0 };
    println!("number diventa -> {number}");

    // - - - -
    // LOOP: (simile a un do while)
    let mut count = 0;
    let contatore = loop {
        println!("again");

        if count > 5 {
            break count * 0 + 10;
        }

        count += 1;
    };
    println!("count -> {contatore}");
    // posso anche dare una label se ci sono piu loop (es: 'primo_loop: loop{})
    // per uscire da un loop che ha un nome: break 'primo_loop;

    // - - - -
    // WHILE:
    let mut c = 3;
    while c != 0 {
        println!("while n. {c}");
        c -= 1;
    }
    // - - - -
    // FOR EACH:
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("element -> {element}");
    }

    // oppure uso un range
    for num in 1..4 {
        println!("num -> {num}");
    }
    
    // Esempio utile
    let arr = ["ciao", "rust", "world"];

    for (i, val) in arr.iter().enumerate() { // Quindi diventa un iteratore di coppie (i, val):
        println!("indice {i} -> {val}");     // (0, &"ciao"), (1, &"rust"), (2, &"world") 
    }
}
