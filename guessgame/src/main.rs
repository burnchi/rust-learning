// use std::io;
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("please input your guess:");
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guess {}", { guess });

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Greater => println!("Too big"),
            std::cmp::Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
