use rand::Rng;
use std::io;
use std::fmt;

#[derive(Debug)]
enum Banana {
    Rotten,
    Unripe,
    Ripe,
    PerfectlyRipe,
    OverlyRipe
}

impl fmt::Display for Banana {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Banana::Rotten => write!(f, "rotten"),
            Banana::Unripe => write!(f, "unripe"),
            Banana::Ripe => write!(f, "ripe"),
            Banana::PerfectlyRipe => write!(f, "perfectly ripe"),
            Banana::OverlyRipe => write!(f, "overly ripe")
        }
    }
}

impl Banana {
    fn random() -> Banana {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=4) {
            0 => Banana::Rotten,
            1 => Banana::Unripe,
            2 => Banana::Ripe,
            3 => Banana::PerfectlyRipe,
            _ => Banana::OverlyRipe
        }
    }
    fn value(&self) -> f64 {
        match self {
            Banana::Rotten => 0.05,
            Banana::Unripe => 1.25,
            Banana::Ripe => 2.00,
            Banana::PerfectlyRipe => 3.50,
            Banana::OverlyRipe => 0.75
        }
    }
}

fn message(bananas_eaten: u32) -> &'static str {
    match bananas_eaten {
        0..=3 => "You need to eat more bananas!",
        4..=6 => "You are doing great!",
        7..=10 => "You are getting a bit chubby!",
        11..=15 => "You are getting too fat!",
        _ => "You need to stop eating bananas it could kill you!"
    }
}
fn main() {
    // This is a program that has mystery bananas. Each banana you peel can be one of these option:
    // 1. rotten 
    // 2. unripe
    // 3. ripe
    // 4. perfectly ripe
    // 5. overly ripe
    // The user should start out with $50 and they can use the money to open a banana each $1.50
    // They will randomly receive one of the top 5 varities and they can sell the banana for a
    // a revenue based on its condition
    // along with this, the user should be able to eat or sell the banana after every opening
    // The program should keep track of their total money and how many bananas were eaten 
    // and tell the user if they are getting too fat
    // for example: 
    // 0-3 bananas eaten: "You need to eat more bananas!"
    // 4-6 bananas eaten: "You are doing great!"
    // 7-10 bananas eaten: "You are getting a bit chubby!"
    // 11-15 bananas eaten: "You are getting too fat!"
    // 16+ bananas eaten: "You need to stop eating bananas it could kill you!"
    let mut money: f64 = 50.00;
    let mut bananas_eaten: u32 = 0;
    let banana_price: f64 = 1.50;

    println!("");
    println!("🏵️🍌 Welcome to Banana Lucky Draw 🍌🏵️");

    loop {
        if money < banana_price {
            println!("You don't have enough money to buy a banana!");
            break;
        }
        println!("You have ${:.2} and {} bananas eaten", money, bananas_eaten);

        println!("Would you like to buy a banana for $1.50? (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "y" {
            money -= banana_price;
            let banana = Banana::random();
            println!("🍌 You got a {} banana!", banana);

            println!("Would you like to eat or sell it? (e/s)");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "e" {
                bananas_eaten += 1;
                println!("🤤 You ate the banana");
                println!("🪧 {}\n", message(bananas_eaten));
            } else if input.trim() == "s" {
                money += banana.value();
                println!("💰 You sold the {} banana for ${:.2}\n", banana, banana.value());
            }
        } else {
            break;
        }
    } 

    println!("\nGame over!");
    println!("Final money: ${:.2}", money);
    println!("Total bananas eaten: {}", bananas_eaten);
}
