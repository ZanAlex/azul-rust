#[derive(std::fmt::Debug)]
pub enum Tile {
    Blue,
    Dark,
    Red,
    Turquoise,
    Yellow,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tile::Blue => write!(f, "B"),
            Tile::Dark => write!(f, "D"),
            Tile::Red => write!(f, "R"),
            Tile::Turquoise => write!(f, "T"),
            Tile::Yellow => write!(f, "Y"),
        }
    }
}

// Reserve

#[derive(std::fmt::Debug)]
pub struct Bag {
    pub reserve: Vec<Tile>
}

impl std::fmt::Display for Bag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Err(e) = write!(f, "(") {
            return std::fmt::Result::Err(e);
        }

        for val in self.reserve.iter() {
            if let Err(e) =  write!(f, " {} ", val) {
                return std::fmt::Result::Err(e);
            }
        }

        if let Err(e) =  write!(f, ")") {
            return std::fmt::Result::Err(e);
        }

        return std::fmt::Result::Ok(());
    }
}

// pub struct Discard {
    
// }

// // Table

// pub struct Table {

// }

// pub struct Coaster {

// }

// pub struct Dump {

// }

// // Player board

// pub struct PlayerBoard {
    
// }

// pub struct Line {

// }

// pub struct Wall {

// }

// pub struct Garbage {

// }