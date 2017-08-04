/*
    RTA (Rust Text Adventure)
    https://github.com/vesche
*/

extern crate rand;

use rand::Rng;

mod common;
mod tiles;

const BANNER: &'static str = "
'||''|.   |''||''|     |    
 ||   ||     ||       |||   
 ||''|'      ||      |  ||  
 ||   |.     ||     .''''|. 
.||.  '|'   .||.   .|.  .||.
     Rust Text Adventure";

const HELP_TEXT: &'static str = "
n - move north
s - move south
e - move east
w - move west
q - quit the game
h - show this help text
i - inventory";

struct Position {
    x: i32,
    y: i32,
}

fn main() {
    // define map
    let mut map = [
    [" ","_","_","_","_","_","_","_","_","_","_","_","_","_","_","_","_","_","_","_","_"," "],
    ["|"," "," "," "," "," "," "," "," "," "," "," "," "," "," "," "," "," "," "," "," ","|"],
    ["|"," "," "," ","*"," "," "," "," "," "," ","*","*"," "," "," "," "," "," "," "," ","|"],
    ["|"," "," "," ","*","*"," "," "," ","*","*","*","*","*"," "," "," "," "," "," "," ","|"],
    ["|"," "," "," ","*","*","*"," "," "," ","*","*","*","*"," ","*","*","*"," "," "," ","|"],
    ["|"," "," "," "," "," ","*","*"," ","*","*","*"," ","*","*","*","*","*"," "," "," ","|"],
    ["|"," "," "," ","*","*","*","*","*","*","*","*"," "," ","*","*","*"," "," "," "," ","|"],
    ["|"," "," "," ","*","*","*","*","*","*"," ","*","*","*","*","*"," "," "," "," "," ","|"],
    ["|"," "," "," "," ","*","*","*","*","*"," "," ","*","*","*"," "," "," "," "," "," ","|"],
    ["|"," "," "," "," "," ","*","*"," "," "," "," "," "," "," "," "," "," "," "," "," ","|"],
    ["|","_","_","_","_","_","_","_","_","_","_","_","_","_","_","_","_","_","_","_","_","|"]];
    
    // starting position
    let mut pos = Position {
        x: 7,
        y: 7,
    };
    let mut mov = Position {
        x: 0,
        y: 0,
    };

    // initial variables
    let mut level = 0;
    let mut message = "You awake in a dark, damp jungle on a mysterious island...";
    let mut inventory = ["lamp", "sword", "hat"];
    
    // this is a placeholder to keep the random lib
    let r = rand::thread_rng().gen_range(1, 101);

    // start game
    common::clear_screen();
    println!("{}\n", BANNER);
    let name = common::input("What's your name? ");
    
    // main game loop
    loop {
        // clear the screen
        common::clear_screen();
        
        // set player position
        map[pos.y as usize][pos.x as usize] = "@";
        
        // print stats & map
        println!("RTA | {} | LV: {} | ({}, {}) | r: {}", name, level, pos.x, pos.y, r);
        for i in 0..11 {
            println!("{}", map[i].join(""));
        }
        
        // print message & tile description
        common::xprint(message);
        tiles::get_desc(pos.x, pos.y);
        
        // reset player position, message, and move modification
        map[pos.y as usize][pos.x as usize] = "*";
        message = "";
        mov.x = 0;
        mov.y = 0;
        
        // get player move
        let mut player_move = String::new();
        player_move = common::input("> ");
        player_move = player_move.to_lowercase();
        
        // act on player move
        match player_move.as_ref() {
            "n" => mov.y = -1,
            "s" => mov.y =  1,
            "e" => mov.x =  1,
            "w" => mov.x = -1,
            "q" => break,
            "h" => message = HELP_TEXT,
            "i" => message = "inv", // inventory.join(" "),
            _   => message = "Invalid move!",
        };
        
        // move in direction if possible
        if map[(pos.y + mov.y) as usize][(pos.x + mov.x) as usize] == " " {
            message = "You can't go that way!";
        } else {
            pos.x += mov.x;
            pos.y += mov.y;
        }
    }
}
