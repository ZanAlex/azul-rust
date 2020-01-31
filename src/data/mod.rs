#[derive(std::cmp::PartialEq)]
pub enum Tile {
    Blue,
    Dark,
    Red,
    Turquoise,
    Yellow,
}

// Reserve

pub struct Bag {
    pub reserve: Vec<Tile>
}

// pub struct Discard {
    
// }

// Table

pub struct Coaster {
    pub tiles: Vec<Tile>,
}

pub struct Table {
    pub coasters: Vec<Coaster>,
    pub dump: Vec<Tile>
}

// Player board

pub struct Line {
    pub tiles: Vec<Tile>,
}

impl Line {
    pub fn capacity(&self) -> usize {
        return self.tiles.capacity();
    }
}

pub struct Wall {
    pub cells: [Option<Tile>; 25],
}

impl Wall {
    pub fn get_cell(&self, line: usize, column: usize) -> & Option<Tile> {
        return &self.cells[(line * 5) + column];
    }
}

pub struct PlayerBoard {
    pub number: usize,
    pub lines: [Line; 5],
    pub wall: Wall,
    pub garbage: [Option<Tile>; 7],
}