// Slice è un riferimento a una parte di una collezione

fn main() {
    // Esempio con Stringhe:
    let s1 = String::from("hello world");
    let slice = &s1[5..10]; // Prendi gli indici da 5 a 9 ("mondo")

    println!("{}", slice); // Output: "mondo"

    // Esempio con Array:
    let array = [10, 20, 30, 40, 50];
    let slice = &array[1..4]; // Prendi gli elementi da indice 1 a 3 (20, 30, 40)

    println!("{}", slice); // Output: [20, 30, 40]
}
