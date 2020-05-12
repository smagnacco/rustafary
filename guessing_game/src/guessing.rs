use std::io; //io lib from std lib
use rand::Rng;
use std::process::exit;
use std::cmp::Ordering;

fn main() {
    println!("Juego de adivina el número");  //the bang! means call macro fn

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Mete tu número bocazas");

        let mut guess = String::new();  //let vs val... scala I'll be missing you
        //let defines a variable
        //mut defines a mutable variable (puaj)
        //:: means associated function on a Type (static method)
        //String::new(); creates an empty string 

        io::stdin()   // stdin() function returns a Stdin instance
            .read_line(&mut guess)  //read_line requires a mutable string to be updated
            //& means that argument is a "reference"
            //references are inmutables, however, you can make it mutable adding mut
            //read_line returns io::Result (enum Ok|Err)
            //Result.expect crash program and shows the message
            .expect("Ops, no pude leer la línea");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim() == "quit" {
                    exit(1);
                } else {
                    continue;
                }
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Chico"),
            Ordering::Greater => println!("Demasiado grande"),
            Ordering::Equal => {
                println!("gotcha");
                break;
            }
        }
    }
}


//NOTES
//Docs, cargo doc --open generates an html doc of project and libs
