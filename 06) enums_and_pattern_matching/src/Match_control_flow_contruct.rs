// The match expression it's like a sorting machine, the values go through each pattern in a match and
// at the first pattern the value "fits", the values falls into the associate code block.

#[derive(Debug)] // Enables printing for UsState with {:?}
enum UsState {
    Alabama, 
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime, 
    Quarter,
    Test(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // these are called match arms, and MUST COVER ALL POSSIBILITIES
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        // Per espressioni piu lunghe: {}
        Coin::Test(state) => {
            println!("Questo è un Coin test -> {:?}", state);
            0
        }
    }
}

// Seems very familiar to the "if" conditional, but there's a big difference:
// with if, the condition needs to evaluate to a boolean value, but here it can be any type.

fn main() {
    let coin = Coin::Penny;
    let coin2 = Coin::Test(UsState::Alabama);

    value_in_cents(coin);
    value_in_cents(coin2);

    // Altro esempio usando il _ placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => fun1(),
        7 => fun2(),
        _ => reroll(), // _ placeholder per tutti gli altri casi
    }

    /* Altro esempio con if let, utile per:
        1) Controlla e usa il valore solo se è presente 
        2) Evitare i match troppo dettagliati: È una scorciatoia per i casi in cui ti serve gestire solo una variante specifica.
    */
    let config_max = Some (3u8);
    // Controlla se config_max contine un valore (Some), se sì quel valore viene assegnato alla variabile max.
    if let Some(max) = config_max {
        //poi printa
        println!("The maximu is configured to  be {max}");
    }

    /* Se c'è un valore (Some): Viene estratto e il blocco di codice è eseguito
       Se non c'è un valore (None): Il blocco di codice viene ignorato.
       - - - -

    Posso aggiungere un else al posto del placeholder: (quindi faccio il mio caso variante specifica, e tutto il resto con else)
   
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("caso speifico..");
    } else {
        count += 1;
    }
    
    */
}
