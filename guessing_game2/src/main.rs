use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!(" ");

    println!("============================================");

    println!(" ");

    println!(" - Guess the number between 1 and 100!");

    println!(" ");

    println!(" - There are 3 rounds (⚔️)! ");

    println!(" ");

    println!(" - You have 7 hearts (❤️)!");

    println!(" ");

    println!(" - 0 Victories (🏆), yet... ");

    println!(" ");

    println!("============================================");

    let mut rounds = 3;
    let mut victories = 0;
    let mut quit = 0;



    // println!("The secret number is: {secret_number}");

    loop {
        let mut hearts = 7;

        if quit == 1{
            break;
        }

        if rounds == 0 && victories == 3{
            println!("YOU HAVE WON THE GAME WITH 3 VICTORIES! 🔥");
            break;
        }else if rounds == 0 && victories == 0{
            println!("YOU HAVE LOST THE GAME WITH 0 VICTORIES");
            break;
        }

        let secret_number = rand::thread_rng().gen_range(1..=100);

        loop {




            println!(" ");

            println!("❤️: {}  ⚔️: {}  🏆: {}", hearts, rounds, victories);

            if hearts == 0 {
                println!(" ");
                println!(" ");
                println!("🔐: {}", secret_number,);
                println!(" ");
                println!(" ");

                break;
            }

            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess = guess.trim();

            if guess == "q" {
                quit += 1;
                break;
            }

            hearts -= 1;

            let guess: i32 = match guess.parse() {
                Ok(num) => num,
                Err(_) => continue,
                /*continue, which tells the program to go to the next iteration of the loop*/
            };

            // println!("You guessed: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Higher"),
                Ordering::Greater => println!("Less"),
                Ordering::Equal => {
                    victories += 1;
                    println!("");
                    println!("You won a vitorie");
                    break;
                }
            }
        }

        rounds -= 1;
    }
}
