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
    let mut pos_y: i32 = 7;
    let mut pos_x: i32 = 7;
    
    // initial variables
    let mut mod_y: i32 = 0;
    let mut mod_x: i32 = 0;
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
        map[pos_y as usize][pos_x as usize] = "@";
        
        // display game - stats & map
        println!("RTA | {} | LV: {} | y: {} x: {} | r: {}", name, level, pos_y, pos_x, r);
        for i in 0..11 {
            println!("{}", map[i].join(""));
        }
        
        // print message & tile description
        common::xprint(message);
        tiles::get_desc(pos_y, pos_x);
        
        // reset player position and message
        map[pos_y as usize][pos_x as usize] = "*";
        message = "";
        mod_y = 0;
        mod_x = 0;
        
        // get player move
        let mut player_move = String::new();
        player_move = common::input("> ");
        player_move = player_move.to_lowercase();
        
        // act on player move
        match player_move.as_ref() {
            "n" => mod_y = -1,
            "s" => mod_y =  1,
            "e" => mod_x =  1,
            "w" => mod_x = -1,
            "q" => break,
            "h" => message = HELP_TEXT,
            "i" => message = "inv", // inventory.join(" "),
            _   => message = "Invalid move!",
        };
        
        // move in direction if possible
        if map[(pos_y + mod_y) as usize][(pos_x + mod_x) as usize] == " " {
            message = "You can't go that way!";
        } else {
            pos_y += mod_y;
            pos_x += mod_x;
        }
    }
}
