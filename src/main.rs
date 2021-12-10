use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    print_prime();

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

fn print_prime() {
    println!("Prime table in 1000");
    let mut num = -1;
    for i in 2..1000 {
        if is_prime(i) {
            num += 1;
            if num % 19 == 0 {
                println!()
            }
            print!("{} ", i)
        }
    }
    println!();
}

fn is_prime(x: u32) -> bool {
    return if x < 2 {
        true
    } else {
        for j in 2..x {
            if x % j == 0 && j != x {
                return false;
            }
        }
        true
    };
}