use rand::Rng;
use std::cmp::Ordering;
use std::io; // come using namespace in C++. così non serve scrivere std::io::stdin() ma solo io::stdin() //enum

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100); // genera un numero casuale tra 1 e 100 (estremi inclusi)

    println!("Please input your guess (from 1 to 100)");
    println!("type 'quit' for exit");
    loop {
        let mut guess = String::new(); // stringa mutabile (non costante).

        io::stdin() // legge da standard input
            .read_line(&mut guess) // legge una linea e la salva in guess. ritorna enum "Ok" o "Err" e lo passa alla funzione expect
            .expect("Failed to read line"); // se non riesce a leggere la linea, stampa il messaggio

        // converto la strimga in numero. posso ridichiarare la variabile. in Rust si usa per convertire il tipo

        // let guess: u32 = guess
        //     .trim() // tolgo spazi e \r\n
        //     .parse() // coverte da string a un altro tipo. in questo caso ho specificato u32
        //     .expect("Please type a number"); // per bloccare l'esecuzione del programma e stampare l'errore
        let guess: u32 = match guess.trim().parse() {
            //metto match davanti che fa da switch
            Ok(num) => num,  // ritorno il numero
            Err(_) => break, //in caso di errore non blocca l'esecuzione del programma. uso _ (catchall) perchè non mi interessa il valore e prendo tutto
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            // ritorna enum di tipo Ordering
            // cmp non centra con std::cmp. e posso usarlo per comparazioni tra variabili dello stesso sito.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
