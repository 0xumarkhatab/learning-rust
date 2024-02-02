use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn variables() {
    let mut x: i32 = 10;
    println!("value of x {x}");
    x = 12;
    println!("value of x {x}");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, _) = tup;
    print!("{a} {b}");
    // let tup1 = ();
    // print!({ type_name(tup1) + "" });
}

fn geussing_game() {
    println!("------------ Guessing Game -----------");
    println!(" ---- Guess Correct Number and Win a Cake ----- ");
    let mut play_game = true;
    while play_game {
        println!("Generating a secret Number.....\nGenerated ✅ \n");
        let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
        println!("Please Input your guess : ");

        let mut guess: String = "".to_string();
        io::stdin()
            .read_line(&mut guess)
            .expect("Not a number value");

        let secret_number = secret_number.to_string();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
        let mut will: String = String::new();

        println!("Do you want to play again ?");
        println!("1 -> Yes \n2-> No ");
        io::stdin()
            .read_line(&mut will)
            .expect("You Entered invalid choice");

        let will: u32 = match will.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match (will).cmp(&1) {
            Ordering::Less => {
                println!("Thank you for playing my game ! \nTake care !");
                play_game = false;
            }
            Ordering::Greater => {
                println!("Thank you for playing my game ! \nTake care !");
                play_game = false;
            }
            Ordering::Equal => {}
        }
    }
}
fn main() {
    // variables()
    geussing_game()
}
