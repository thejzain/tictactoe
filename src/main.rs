use std::io;

fn main() {
    println!("Tic Tok Toe");
    println!("press y to start");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    println!("{}", user_input);
}
