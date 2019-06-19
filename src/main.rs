use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    print!("Guess the number! ");
    io::stdout().flush();

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line!");
    
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number: {}", secret_number);
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too big"),
    }
}