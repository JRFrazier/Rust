use std::io;
use std::cmp.Ordering;
use rand::Rng;

fn main() {
    println!("Gues the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is : {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new(); // declares a new "mutable" var called "guess" with an empty instance of a string

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
