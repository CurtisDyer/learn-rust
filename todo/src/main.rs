use std::io;
use std::io::Write;
use rand::Rng;

fn main() {
    println!("Guessing Game\n");

    let answer: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        let mut line = String::new();

        print!("\nGuess a natural number between 1-100: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut line)
            .expect("Line read error");

        let guess = line.trim();
        let n: u32 = guess.parse().expect("Number parse error");

        if answer == n {
            println!("\n** The answer is indeed {answer}! **");
            break
        }
        else {
            println!("Guess too {}", if n < answer { "low" } else { "high" })
        }
    }
}