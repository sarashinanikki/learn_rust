use std::io;

fn main() {
    println!("Guess the number!");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Faild to read line");
    println!("You guessed: {}", guess);
}
