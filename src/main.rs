use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("猜数!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("神秘数字是: {}", secret_number);

    loop {
        println!("猜测一个数字: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数是: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
