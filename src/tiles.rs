/*
    RTA (Rust Text Adventure)
    https://github.com/vesche
*/

use common::xprint;

// description and items are going to pop off as needed
// probably have to keep the state of each struct externaL? gross

// i'll keep a huge state list that will match with ALL_TILES
// state = [0, 0, 0, 0, 0, 1, 1, 1]
// ALL_TILES = [BEACH, JUNGLE_A ..

struct Tile {
    coord:  (i32, i32),
    desc:   &'static str,
    items:  &'static str,
}

/*
    Jungle (south west side of the map)
*/
const JUNGLE_A: Tile = Tile {
    coord:  (6, 7),
    desc:   "JUNGLE_A",
    items:  "",
};
const JUNGLE_B: Tile = Tile {
    coord:  (7, 7),
    desc:   "JUNGLE_B",
    items:  "Fishing Pole",
};
const JUNGLE_C: Tile = Tile {
    coord:  (8, 7),
    desc:   "JUNGLE_C",
    items:  "",
};

/*
    Quest Maps?
*/
const OLD_MAN: Tile = Tile {
    coord:  (14, 8),
    desc:   "OLD_MAN",
    items:  "something for gold fish",
};

static ALL_TILES: [Tile; 4] = [JUNGLE_A, JUNGLE_B, JUNGLE_C, OLD_MAN];

pub fn get_desc(pos_x: i32, pos_y: i32) {
    // Print tile description given a (y, x) coordinate

    for i in 0..ALL_TILES.len() {
        if ALL_TILES[i].coord == (pos_x, pos_y) {
            xprint(ALL_TILES[i].desc);
        }
    }
}