use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Let's play guess game!");
    let secret = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is {}.", secret);

    print!("Here is a number, ");
    let mut guess = String::new();

    loop {
        guess.clear();
        println!("guess it:");

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Great, You guessed right!");
                break;
            }
        }
    }
}
