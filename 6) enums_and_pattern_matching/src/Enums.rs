
// Nell'Enum, creo 4 varianti con diversi tipi

// Sono particolarmente utili per modellare situazioni che richiedono una scelta 
// tra diverse opzioni o per gestire casi che possono assumere stati diversi.
enum Message {
    Quit, 
    Move { x: i32, y: i32 },    // struct
    Write(String),              // una tupla singola che contiene un tipo stringa
    ChangeColor(i32, i32, i32), // tupla  

}

fn main() {
    impl Mesage {
        fn call(&self) {
            println!("Metodo call");
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
