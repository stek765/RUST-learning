/*
    THE STACK = LIFO organized structure. All data stored on the stack MUST HAVE a KNOWN FIXED SIZE.
    if not,

    they are put in THE HEAP = less organized, when u put data on the heap, u request a certain amount of space
                               The memory allocator finds an empty spot in the heap that is big enough (FRAGMENTATION),
                               marks it "being used" and returns a pointer to it.  (processo of ALLOCATING on the heap)

    Since this pointer is a known fixed size, we can store it in the stack.

    Pushing to the stack is FASTER then the heap cause there's no allocation time.
    Accessing is FASTER too cause u dont' follow pointers jumping around.

    When a function is called, all the values passed, the function's local variables (potentially pointers to heap too)
    get pushed onto the stack. When the function is over, those values get popped off the stack.
*//*

    OWNERSHIP RULES:
        - Each value in Rust has an Owner
        - There can only be one owner at a time
        - When the owner goes out of scope, the value will be dropped.

        scope = the range within a program for which an item is valid

        THE IDEA:
        Rust automatically return the memory once the variable that owns it goes out of scope. (calls drop function)
*/

fn main() {
    let s1 = String::from("hello");

    // Guarda qua:
    let s2 = s1;
    let s2 = s1.clone();
    /* cosa cambia?
       Beh essendo le String un tipo di dato complesso e di dimensione non fissa,
       vengono salvate nello heap. I tipi con un fixed size sono organizzati gia in tempo di compilazione
       però i tipi complessi invece devono richiedere a Runtime lo spazio necessario nello heap.

       Quindi.. essendo dei puntatori dire s2 = s1 (processo di move) è come aggiungere una freccia che va sia da
       s1 che da s2 verso lo stesso blocco di memoria dello heap. Rust risolve problemi di conflitto
       con lo shadowing ovvero invalidando s1. Una volta finito di usare s2, ovvero usciti dallo scope,
       il pezzo di heap verrà liberato perchè s2 lo puntava.

  !!!  Se uso clone invece, viene allocato un ulteriore spazio nello heap come avessi fatto una copia con una nuova istanza
       in un punto diverso in memoria.
    */
}
