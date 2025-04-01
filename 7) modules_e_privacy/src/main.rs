// Riepilogo: Packages, Crates, Modules, Paths, Privacy...

/* 
    Package ->  è il Cargo.toml che tiene tutte le librerie esterne al progetto usate + info utili
    Crate   ->  sono le unità di compilazione: binario o libreria (main.rs o lib.rs) e sono in src/ come tutti gli altri file
    Module  ->  sono i file "contenitori" di funzioni e codice e servono per gestire come mostrare 
                il loro codice verso l'esterno, se renderlo pubblico in alcuni punti etc..
                (meglio creare più moduli per separare funzioni che non c'entrano l'una con l'altra)

                I MODULI DEVONO ESSERE DICHIARATI NEL MAIN.RS/LIB.RS OPPURE CREANDO FILE CON NOME DELLA CARTELLA
                IN CUI SONO CONTENUTI 
                (es: src/pippo.rs  in cui inserisco mod pippo
                     src/pippo/pluto.rs
                )
    
    Path    ->  Può essere Relativo o Assoluto, è semplicemente il percorso verso un file/funzione (crate::pippo::pluto)
                • crate::  <- parte dalla radice del progetto
                • super::  <- dal modulo padre
                • self::   <- dal modulo corrente
    
    pub     -> rende visibile una funzione/modulo fuori dal modulo (pub fn, pub mod)
               DI DEFAULT LA VISIBILITÀ DI UN MODULO È PRIVATA
    
    use/as  -> "use" porta qualcosa nello scope attuale (use crate::pippo::ciao) così posso direttamente chiamare ciao senza riscriverlo
               "as"  in combinazione con use serve a dare un alias (use crate::pippo::ciao as MyName)

               "pub use" posso combinare pub con use per ri-esportare la funzione che sto usando,
                         nel senso che la rendo pubblica/chiamabile dall'esterno nel modo in cui l'ho definita chiamandola
                         (es: pub use crate::pippo::ciao as MyName, sarà visibile dall'esterno e chiamabile con MyName)

    Nested paths -> per evitare in progetti grandi di usare use n volte e riempire lo schermo, 
                    si può scrivere: "use std::{cmp::fn1, fn2, fn3, fn4, ...}"
    
    - - - - - - - - -
    
    SUMMARY dell'idea: Creo diversi moduli separati in base al loro scopo, li dichiaro nel file main o nome_cartella con mod nome.rs
                       Poi dentro i singoli moduli decido cosa rendere pubblico con pub.

*/
fn main() {
    
}
