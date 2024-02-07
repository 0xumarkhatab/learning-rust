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
struct Rectangle {
    height: i32,
    width: i32,
}
impl Rectangle {
    fn rect_area(&self) -> i32 {
        return self.width * self.height;
    }
    fn can_hold(&self, rect2: &Rectangle) -> bool {
        return self.height > rect2.height && self.width > rect2.width;
    }
    fn Square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    // variables()
    // geussing_game()
    let rect = Rectangle {
        height: 30,
        width: 30,
    };
    let rect2: Rectangle = Rectangle {
        height: 29,
        width: 29,
    };
    let rect3: Rectangle = Rectangle {
        height: 31,
        width: 31,
    };
    let square = Rectangle::Square(23);

    println!("The rect area is {}", rect.rect_area());
    println!(
        "Rect 1 {}x{} can hold Rect 2 {}x {} : {}",
        rect.width,
        rect.height,
        rect2.width,
        rect2.height,
        rect.can_hold(&rect2)
    );
    println!(
        "Rect 1 {}x{} can hold Rect 3 {}x {} : {}",
        rect.width,
        rect.height,
        rect3.width,
        rect3.height,
        rect.can_hold(&rect3)
    );
    println!("Square area is {} ", square.rect_area())
}
