use std::cmp::Ordering;

fn main() {
    println!("Welcome To Guess Game!");

    let secret_number: u32 = 20;

    let mut i = 1;

    while i <= 3 {
        println!("Secret number is {}", secret_number);

        println!("Enter your guess");

        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter an integer!");
                continue
            }
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too large"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {println!("Correct guess at {} attempt", i);break}
        }
        i+=1;
    }
    

    

}
