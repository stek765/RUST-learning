fn main() {
    /*
    TIPI DI VARIBILI:

    - - - -
    INTEGERS -> u8/i8, u16/i16, u32/i32, u64/i64, u128/i128, usize/isize
               (8 bit) (16 bit) etc..                        (bit_architettura_computer)

        u = unsigned, un valore sempre positivo  [ grandezza = +- 2^(n-1) - 1, con n = numero bit]
        i = signed,   può essere negativo        [ grandezza = +  2^(n) - 1,   con n = numero bit]

        gli integers possono essere scritti anche con:
            - Decimale -> 1_000 (mille)
            - Hex      -> 0xff
            - Octal    -> 0o77
            - Binary   -> 0b1111_0000

    - - - -
    FLOATING-POINT -> f64 , f32

        let x = 2.0          // f64
        let y: f32 = 3.0     // f32

    - - - -
    BOOLEAN:
        let t = true;
        let f: bool = false  // se si vuole esplicitare il tipo

    - - - -
    CHARACTER:
        let c = 'z';
        let z: char = 'Z';   // se si vuole esplicitare il tipo

    - - - -
    TUPLE: (!!!PUÒ AVERE TIPI DIVERSI!!!)
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let tup2 = (100, 2.4, 3);

        // prendere gli elementi dalla tupla:
        1) PRIMO MODO:
            let (x, y, z) = tup2;
            println!("Valore y : {y}");

        2) SECONDO MODO:
            let primo_tup2 = tup2.0;
            let secondo_tup2 = tup2.1;
            let terzo_tup2 = tup2.2;

    - - - -
    ARRAY:  (!!!SOLO TIPI UGUALI!!!) + (utile per avere una LUNGHEZZA fissa di elementi) + (utile per usare stack e non heap)
        let array = [1, 2, 3, 4, 5];

        let a: [i32; 5] = [1, 2, 4, 5, 6];  // dichiarazione esplicita
        let a = [3; 5];   // array inizializzato con tutti i valori a 3

        Accesso agli elementi Array:
            let primo_elemento_a   = a[0];
            let secondo_elemento_a = a[1];
    */
}
