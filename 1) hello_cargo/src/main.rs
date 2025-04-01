fn main() {
    println!("Hello, world!");
}

// Cargo new <nome_prog> -> crea progetto con cargo
// Cargo build -> compila
// ./target/debug/file_binario -> i binari sono nel debug
// Cargo run -> compila e runna da solo

// Cargo check -> controlla il file sia compilabile MA NON LO COMPILA. (piu veloce)

// Cargo build --release -> Compila con ottimizzazioni (fine progetto) {Crea l'eseguibile in target/release}

// Cargo update -> di base una volta installata le dipendencies messe nel file "Cargo.toml" rust
//     le salva nel Cargo.lock e bona.. però se vuoi aggiornarle di versione usi update.

// Cargo odoc --open -> da la documentazione di tutte le mie dipendencies del proj
