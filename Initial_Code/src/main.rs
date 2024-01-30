// Initial code of the Rust's book

use std::io;

fn main() {
    println!("Guessing game!");
    println!("Type a number!: ");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Error reading guess");

    println!("Number typed!: {}", guess);
}
