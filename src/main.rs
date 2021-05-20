use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Hello, world! {}", sum(10, 20));

    ownership();
    
}

fn ownership(){

    let mut s = String::from("Hello");

    some(&mut s);

    // Some Comments.
    println!("{}", s);

    // Feature 3 PR 4 Commit 2. Hello


}

fn some(a : &mut String){
    a.push_str(" some");

}

fn array_sample(){
    let a = [1,2,3];

    let mut index = String::new();

    println!("Enter Index");

    io::stdin()
    .read_line(&mut index)
    .expect("Not an number");
    
    let index: usize = index.trim().parse().expect("Not an number");

    println!("Index value is {}", a[index]);
}


fn tuple_sample()
{
    let tup = (10, 20, 30);

    let (x, y, z) = tup;

    println! ("{} {} {}", x, y, z);
}

fn sum(x: usize, y: usize) -> usize{
    x + y
}

fn get_and_return(x: usize) -> usize
{
    x
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
