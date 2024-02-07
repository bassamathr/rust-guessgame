use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn mult1(a: i32, b:i32) -> i32{
    a * b
}
fn subtract(m1: i32, m2: i32) -> i32 {
    m1 - m2
}
fn main() {
    println!("Guess the number");

     loop{  
        let secret_number = rand::thread_rng().gen_range(1, 102);

        let mut Guess = String::new();

        io::stdin().read_line(&mut Guess).expect("hello this is the error");
        let Guess:i32 = Guess.trim().parse().expect("not the right");

        
        let sec_secret_number = rand::thread_rng().gen_range(1, 102);

        let mut sec_guess = String::new();
        io::stdin().read_line(&mut sec_guess).expect("not the right choice mate");
        let sec_guess:i32 = sec_guess.trim().parse().expect("not the right choice");
        println!("here is the input {}", Guess);
        println!("this is your first number {}", Guess);
        println!("this is your second number{}", sec_guess);
        println!("this is your secret number {}", secret_number);
        println!("this is your secret number {}", sec_secret_number);


        let multi1: i32 = mult1(Guess, secret_number);
        let multi2: i32 = mult1(sec_guess, sec_secret_number);

        println!("{},{}", multi1, multi2);

        let sub = subtract(multi1, multi2);
        println!("this is your final answer {}", sub);
        println!("write quit to exit");

        match sub.cmp(&0) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
}
}
