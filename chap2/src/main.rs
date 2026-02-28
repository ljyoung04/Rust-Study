use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret : u32 = rand::thread_rng().gen_range(1..=100);
    
    println!("업 다운 게임!!");
    
    loop {
        let mut guess = String::new();

        println!("What is your input?");

        io::stdin().read_line(&mut guess).expect("Fail to read line.");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            },
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Up"),
            Ordering::Greater => println!("Down"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        }
        println!();
    }

}
