use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number is {secret_number}");
    
    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Readline failed.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Couldn't parse guess number. {err}");
                continue;
            },
        };

        println!("Our guess is {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less"),
            Ordering::Equal => {
                println!("Equal");
                break;
            },
            Ordering::Greater => println!("Greater"),
        }   
    }
}
