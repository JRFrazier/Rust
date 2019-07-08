use std::io;

fn main() {
    println!("Gues the number!");
    println!("Please input your guess.");

    let mut guess = String::new(); // declares a new "mutable" var called "guess" with an empty instance of a string

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
