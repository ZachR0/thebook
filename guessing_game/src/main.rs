use std::io; //This allows us to get user input
use rand::Rng; //Rand crate

fn main() {
    println!("Guess the number!");

    //Generate random number
    let secret_number = rand::thread_rng()
    .gen_range(1, 101);

    println!("Your secret num is {}", secret_number);

    println!("Please input your guess.");

    //let creates a variable, for example,
    // let foo = 5; this is immutable
    // let mut foo = 5; this is mutable

    let mut guess = String::new(); //Create mutable var of string type

    //read_line returns a Result() type, which returns Ok or Err (enum type)
    //.expect() crashes the program and displays the error message if Err is returned.
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
