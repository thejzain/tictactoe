use std::io;

fn draw(state: &[&str]) {
    println!("\n");

    for i in (0..3).rev() {
        let offset = i * 3;
        print!("---------------\n");
        print!(" | ");
        print!("{}", state[offset]);
        print!(" | ");
        print!("{}", state[offset + 1]);
        print!(" | ");
        print!("{}", state[offset + 2]);
        print!(" | ");
        print!("\n");
    }
    print!("---------------\n");
}

fn play<'a>(state: &mut [&'a str], player: &mut &'a str) {
    let mut user_input = String::new();
    if io::stdin().read_line(&mut user_input).is_err() {
        println!("Couldn't read line! Try again.");
    }
    let int: usize = user_input.trim().parse().unwrap();
    let index = int - 1;
    state[index] = player;
    if *player == "x" {
        *player = "o";
    } else {
        *player = "x";
    }
}
fn main() {
    let mut state = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut player = "x";

    loop {
        draw(&state);
        play(&mut state, &mut player);
    }

    // println!("{}", user_input);
}
