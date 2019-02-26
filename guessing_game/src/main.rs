use std::io; //This allows us to get user input
use std::cmp::Ordering; //Less, Greater, Equal enum
use rand::Rng; //Rand crate

fn main() {
    println!("Guess the number!");

    //Generate random number
    let secret_number = rand::thread_rng()
        .gen_range(1, 101); //inclusive on lower bound, exclusive on upper bound

    //println!("Your secret num is {}", secret_number);

    loop {
        println!("Please input your guess.");

        //let creates a variable, for example,
        // let foo = 5; this is immutable
        // let mut foo = 5; this is mutable

        let mut guess = String::new(); //Create mutable var of string type

        //read_line returns a Result() type, which returns Ok or Err (enum type)
        //.expect() crashes the program and displays the error message if Err is returned.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        //Convert input to number, shadow the value of the previous guess varaible, then annotate to u32 after trimming whitespace and parsing to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>  {
                println!("You win!");
                break;
            }
        }
    }
}
