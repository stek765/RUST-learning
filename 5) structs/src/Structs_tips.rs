// La STRUCT è come una tupla, ma col vantaggio di poter gestire i nomi di ogni cosa.

// Va definita all'esterno

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// posso anche creare delle  tuple con nomi:
struct Esempio (i32, i32, f64);



fn main() {
    // Creazione istanza di tipo User
    let user1 = User {
        active: true, 
        username: String::from("someusername12"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Per modificare un elemento la variabile deve essere mutabile:
    let mut user1 = User {
        email: String::from("someone@example.com"),
        // etc.. non importa l'ordine in cui li creo
    };

    user1.email = String::from("anotheremail@gmail.com"); // cambio la variabile mail perchè mut


    // funzione che costruisce istanze struct, con delle abbreviazioni:
    let user2 = build_user("email@gmail.com", "Nome random");


}   

fn build_user(email: String, username: String) -> User {
    let email = String::from("b@example.com");
    let username = String::from("bob");

    User {
        email,          // equivale a: email: email,
        username,       // equivale a: username: username,
        ..user1         // copia gli altri campi da user1
    }
    /* diverse cose successe: 
        1) posso dire campo: campo_input semplicemente con campo, (email: email -> diventa email,)
        2) posso scrivere solo quelle che cambiano se il resto delle variabile è uguale a qualche altra istanza
    */
}