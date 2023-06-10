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
    println!("Enter the place to put {} : ", player);
    if io::stdin().read_line(&mut user_input).is_err() {
        println!("Couldn't read line! Try again.");
    }
    let int: usize = user_input.trim().parse().unwrap();
    let index = int - 1;
    state[index] = player;
    switch_player(player);
}

fn game_check(state: &[&str]) -> bool {
    for tmp in 0..3 {
        if state[tmp] == state[tmp + 3] && state[tmp] == state[tmp + 6] {
            return true;
        }
        let tmp = tmp * 3;
        if state[tmp] == state[tmp + 1] && state[tmp] == state[tmp + 2] {
            return true;
        }
    }
    if (state[0] == state[4] && state[0] == state[8])
        || (state[2] == state[4] && state[2] == state[6])
    {
        return true;
    }
    false
}

fn switch_player(player: &mut &str) {
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
        let check = game_check(&state);
        if check {
            draw(&state);
            switch_player(&mut player);
            println!("Player {} win", player);
            break;
        }
    }
}
