use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Converting string to number
        /*
            @NOTE: When the user presses enter, a newline character is added to the string.
            For example, if the user types 5 and presses enter, guess looks like this: 5\n.
            The \n represents “newline,” the result of pressing enter.
            The trim method eliminates \n, resulting in just 5.
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            /*
                @NOTE: The underscore, _, is a catchall value;
                in this example, we’re saying we want to match all Err values,
                no matter what information they have inside them.
            */
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
