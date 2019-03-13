use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
   

    let secret_num = rand::thread_rng().gen_range(1, 101);
    loop {
         let mut guess = String::new();
            io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
        let guess: u32 = guess.trim().parse()
        .expect("Enter a number");
        match guess.cmp(&secret_num){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
