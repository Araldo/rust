use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number:");

    let secret = rand::thread_rng().gen_range(1..101);

    loop {
        let mut value = String::new();

        io::stdin().read_line(&mut value).expect("Failed");

        let value: u32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", value);

        match value.cmp(&secret) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("correct");
                break;
            }
        }
    }
}
