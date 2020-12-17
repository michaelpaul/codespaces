use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let secret = rand::thread_rng().gen_range(1, 100);

    loop {
        print!("Chute um numero: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Como assim falhou?");
        let guess: u16 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("We have a winner!");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}
