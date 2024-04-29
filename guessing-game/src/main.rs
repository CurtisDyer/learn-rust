use std::env;
use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let debug = false;
    let args: Vec<String> = env::args().collect();
    let mut start = 1;
    let mut end = 100;

    println!("\nGuessing Game\n");

    if args.len() > 1 {
        start = (&args[1]).parse().expect("Integer expected");
        end = (&args[2]).parse().expect("Integer expected");
    }

    let answer: u32 = rand::thread_rng().gen_range(start..=end);
    let mut line = String::new();
    let mut attempts = 1;

    if debug {
        println!("[Debugging] - Answer = {answer}");
    }

    loop {
        print!("\nGuess a natural number between {}-{}: ", start, end);
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut line)
            .expect("Line read error");

        let guess: u32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: not a valid number, try again!");
                line.clear();
                continue;
            }
        };

        match guess.cmp(&answer) {
            Ordering::Less => println!("You guessed too low!"),
            Ordering::Greater => println!("You guessed too high!"),
            Ordering::Equal => {
                println!("\n*** YOU WIN! {guess} was the answer, and you took {attempts} {}!", if attempts > 1 { "attempts" } else { "attempt" });
                break;
            },
        }

        line.clear();
        attempts += 1;
    }
}