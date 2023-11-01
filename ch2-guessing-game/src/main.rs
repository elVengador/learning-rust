use rand::Rng;           // bring random library into the scope
use std::cmp::Ordering;  // bring comparator library into the score
use std::io;             // bring the io library into the scope

fn main() {
    println!("Guess the number!");

    // define a immutable variable
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("the secret_number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // define a mutable variable
        // call the new() function associated to the type String
        let mut guess = String::new();

        // create an instance of stdin
        io::stdin()
            // send a mutable reference to read_line method
            .read_line(&mut guess)
            // handing error
            .expect("Failed to read line");

        // match expression, will execute the expression will fit the condition
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // callback when operation is successful
            Err(_) => continue,  // callback when operation has an error
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Low!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }

        // print the guess variable
        println!("You guessed: {guess}");
    }
}
