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
    Quest Maps
*/
const CHESTS: Tile = Tile {
    coord:  (7, 7),
    desc:   "Trees whisper in the wind all around you, strange hyroglifics have been etched into some of there trunks. A massive gold chest and a small silver chest are here in the clearing.",
    items:  "Mainsail Rudder",
};
const GOBLINS: Tile = Tile {
    coord:  (6, 7),
    desc:   "You are deep in the jungle of the island. Three bright red goblins are skipping about amongst the trees. The goblins show their sharp teeth in hunger.",
    items:  "SilverKey",
};
const APPLE_TREE_A: Tile = Tile {
    coord:  (8, 7),
    desc:   "The jungle grows less thick here. There is a sorry looking apple sitting on the jungle floor.",
    items:  "Apple",
};
const APPLE_TREE_B: Tile = Tile {
    coord:  (10, 5),
    desc:   "You march atop a large hill. There is an enormous apple tree here adorned with huge shiny red apples perfect to pick. There is a sign standing in front of the tree that reads: 'Take one, or else!'",
    items:  "Apple",
};
const APPLE_TREE_C: Tile = Tile {
    coord:  (16, 5),
    desc:   "A beautiful beach is around you in every direction. A little girl sits crossed legged here reading with a book in one hand, and an apple in the other.",
    items:  "Apple",
};
const OLD_MAN: Tile = Tile {
    coord:  (14, 8),
    desc:   "The rocky coastline becomes flat here. A small seaside shack has been built near the water. An ancient man is sitting in a chair weaving a large net. His eyes shine a bluish grey that sends a shiver down your spine.",
    items:  "Shovel",
};
const GOLD_FISH: Tile = Tile {
    coord:  (4, 2),
    desc:   "The sun shines down on the ocean with a fantastic golden color. There are tons of fish in the water, more than you have ever seen!",
    items:  "GoldFish",
};
const DIRT_MOUND: Tile = Tile {
    coord:  (6, 4),
    desc:   "You walk on a narrow beach with water on both sides. There is an extremely odd mound of dirt here that's been dried by the sun, you'd need a tool of some sort to uncover it.",
    items: "GoldKey",
};
const DOCK: Tile = Tile {
    coord:  (12, 2),
    desc:   "A short wooden dock here extends out to the water. There is a run down sailboat tied to the end of the dock. This might be the only way to escape! However, you notice the sailboat is missing a mainsail and the rudder is broken beyond repair.",
    items: "",
};

static ALL_TILES: [Tile; 9] = [CHESTS, GOBLINS, APPLE_TREE_A, APPLE_TREE_B,
    APPLE_TREE_C, OLD_MAN, GOLD_FISH, DIRT_MOUND, DOCK];

pub fn get_desc(pos_x: i32, pos_y: i32) {
    // Print tile description given a (y, x) coordinate

    for i in 0..ALL_TILES.len() {
        if ALL_TILES[i].coord == (pos_x, pos_y) {
            xprint(ALL_TILES[i].desc);
        }
    }
}
