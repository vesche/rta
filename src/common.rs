/*
    RTA (Rust Text Adventure)
    https://github.com/vesche
*/

use std::io;
use std::io::Write;
use std::thread;

pub fn input(prompt: &str) -> String {
    // Returns a user input string without a trailing new line.
    // Accepts a prompt to display to the user before input.

    // write prompt to screen
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    // prompt user for input
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)
        .expect("Failed to read input");
    
    // strip new line and return
    user_input.pop();
    user_input
}

pub fn xprint(words: &str) {
    // Given a string, prints each character with .05 second increments

    for i in 0..words.len() {
        // print single letter
        print!("{}", words.chars().nth(i).unwrap());
        io::stdout().flush().unwrap();

        // sleep for .05 seconds
        thread::sleep_ms(50);
    }
    println!();
}

pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}

/*
pub fn tile_desc(pos_y: i32, pos_x: i32) {
    // Print tile description given a (y, x) coordinate

    let coords = [(4, 10), (5, 10)];
    let test = ["Beach", "Jungle"];
    
    for i in 0..coords.len() {
        if coords[i].0 == pos_y && coords[i].1 == pos_x {
            xprint(test[i]);
        }
    }
}
*/