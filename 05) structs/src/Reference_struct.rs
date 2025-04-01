// Obbiettivo: Capire come usare il "lifetime" per mantenere validi eventuali puntatori delle variabili della struct
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}



fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area rettangolo -> {}", area(&rect1));

    /* println!("rect1 is {}", rect1) -> NON FUNZIONA da errore
       PER RISOLVERE: 
            si aggiunge in alto nel file  ->  #[derive(Debug)]
            
            e POI: 
                modifico il println!("rect1 is {:?}", rect1);
    */
    println!("rect1 is {:?}", rect1); // printa rect1 is Rectangle { width: 30, height: 50 }

    /* oppure  usando dbg!:
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };
        
        dbg!(&rect1);
    */
}

// passo una reference di Rectangle. We borrow the struct rathern than take ownership of it
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

