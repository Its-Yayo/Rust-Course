// TODO: Check if number input works
use std::io;

fn main() {
    println!("Inout number game!");
    println!("Type a number: ");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Error reading guess");

    println!("Number typed: {}", guess);
}
