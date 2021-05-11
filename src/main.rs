use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let a = [1,2,3];

    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("Not an number");
    
    let index: usize = index.trim().parse().expect("Not an number");

    println!("Index value is {}", a[index]);
}

fn guess_rnd()
{
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Secret is {}", secret_number);

    loop
    {
        println!("Please input your guess.");

        let mut guess = String::new();

        let _x = io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

        //let guess:u32 = guess.trim().parse().expect("Please type a number!");

        let guess:u32 = match guess.trim().parse()
        {
            Ok(num)=>num,
            Err(_)=>continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!");break;},
        }
    }
}
