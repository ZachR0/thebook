use std::io; //This allows us to get user input

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    //let creates a variable, for example,
    // let foo = 5; this is immutable
    // let mut foo = 5; this is mutable

    let mut guess = String::new(); //Create mutable var of string type

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
}
