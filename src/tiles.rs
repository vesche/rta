/*
    RTA (Rust Text Adventure)
    https://github.com/vesche
*/

use common::xprint;

struct Tile {
    pos_y: i32,
    pos_x: i32,
    desc: &'static str,
    state: i32,
}

const BEACH: Tile = Tile {
    pos_y: 4,
    pos_x: 10,
    desc: "Beach",
    state: 0,
};

const JUNGLE: Tile = Tile {
    pos_y: 5,
    pos_x: 10,
    desc: "Jungle",
    state: 0,
};

static ALL_TILES: [Tile; 2] = [BEACH, JUNGLE];

pub fn get_desc(pos_y: i32, pos_x: i32) {
    // Print tile description given a (y, x) coordinate

    for i in 0..ALL_TILES.len() {
        if ALL_TILES[i].pos_y == pos_y && ALL_TILES[i].pos_x == pos_x {
            xprint(ALL_TILES[i].desc);
        }
    }
}