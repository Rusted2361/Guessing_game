use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1,101);
    
    println!("the secret_number is {}", secret_number);

    loop {
       
    println!("please input your guess");

    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("failed to read line");
    
    let guess: u32 = guess.trim().parse().expect("please enter a number");
    
    println!("guess {}",guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Equal => {
            println!("you win");
            break;
        },
        Ordering::Greater => println!("too big"),
    }
    }
    
}
