use std::io;

fn main() {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Couldn't read line!");

    let number: i32 = guess
        .trim()
        .parse()
        .expect("You're supposed to enter a number!");

    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }
}
