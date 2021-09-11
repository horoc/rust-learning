use std::io; 
use rand::Rng; // trait
use std::cmp::Ordering;

fn main() {
    println!("Number guess ~");
    let secret_number = rand::thread_rng().gen_range(1..11);
    println!("guess a number: ");
    
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("read error");
        println!("the number you guess is : {}", guess);

        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
               println!("You win!");
               break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
}
