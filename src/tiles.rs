/*
    RTA (Rust Text Adventure)
    https://github.com/vesche
*/

use common::xprint;

struct Tile {
    pos_y: i32,
    pos_x: i32,
    desc: &'static str,  // this might need to be an array
    items: &'static str, // this might need to be an array (pop off in interaction)
    state: i32,
}

/* TEMPLATE
const NAME: Tile = Tile {
    pos_y: 0,
    pos_x: 0,
    desc: "",
    items: "",
    state: 0,
}
*/

const BEACH: Tile = Tile {
    pos_y: 4,
    pos_x: 10,
    desc: "Beach",
    items: "",
    state: 0,
};

const JUNGLE_A: Tile = Tile {
    pos_y: 7,
    pos_x: 6,
    desc: "There are thick trees all around you. JUNGLE_A",
    items: "",
    state: 0,
};

// Starting map!
const JUNGLE_B: Tile = Tile {
    pos_y: 7,
    pos_x: 7,
    desc: "There are thick trees all around you. JUNGLE_B",
    items: "shovel",
    state: 0,
};

const JUNGLE_C: Tile = Tile {
    pos_y: 7,
    pos_x: 8,
    desc: "There are thick trees all around you. JUNGLE_C",
    items: "",
    state: 0,
};

static ALL_TILES: [Tile; 4] = [BEACH, JUNGLE_A, JUNGLE_B, JUNGLE_C];

pub fn get_desc(pos_y: i32, pos_x: i32) {
    // Print tile description given a (y, x) coordinate

    for i in 0..ALL_TILES.len() {
        if ALL_TILES[i].pos_y == pos_y && ALL_TILES[i].pos_x == pos_x {
            xprint(ALL_TILES[i].desc);
        }
    }
}