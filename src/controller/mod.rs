use rand::Rng;
use crate::data::{ Tile, Bag, Table, Coaster, PlayerBoard, Line, Wall };

pub fn create_bag() -> Bag {
    let mut result = Bag {
        reserve: Vec::with_capacity(100)
    };

    for _ in 0..20 {
        result.reserve.push(Tile::Blue);
    }

    for _ in 0..20 {
        result.reserve.push(Tile::Dark);
    }

    for _ in 0..20 {
        result.reserve.push(Tile::Red);
    }

    for _ in 0..20 {
        result.reserve.push(Tile::Turquoise);
    }

    for _ in 0..20 {
        result.reserve.push(Tile::Yellow);
    }

    return result;
}

pub fn shuffle_bag(bag: &mut Bag) {
    let length = bag.reserve.len();

    for i in 1..length {
        let other = rand::thread_rng().gen_range(0, i);
        bag.reserve.swap(i, other);
    }
}

pub fn create_table(players_count: usize) -> Table {
    if players_count < 2 || players_count > 4 {
        panic!("Invalid players_count: {}. Expects [2; 4]", players_count);
    }

    let coasters_count : usize = (players_count * 2) + 1;

    let mut result = Table {
        coasters: Vec::with_capacity(coasters_count),
        dump: Vec::with_capacity(coasters_count * 4),
    };

    for _ in 0..coasters_count {
        result.coasters.push(Coaster {
            tiles: Vec::with_capacity(4)
        });
    }

    return result;
}

pub fn create_playerboards(players_count: usize) -> Vec<PlayerBoard> {
    let mut result = Vec::with_capacity(players_count);

    for index in 0..players_count {
        result.push(PlayerBoard {
            number: index + 1,
            lines: [
                Line { tiles: Vec::with_capacity(1) },
                Line { tiles: Vec::with_capacity(2) },
                Line { tiles: Vec::with_capacity(3) },
                Line { tiles: Vec::with_capacity(4) },
                Line { tiles: Vec::with_capacity(5) },
            ],
            wall: Wall { cells: Default::default() },
            garbage: Default::default(),
         });
    }

    return result;
}

pub fn setup_table(bag: &mut Bag, table: &mut Table) {
    for coaster in table.coasters.iter_mut() {
        assert!(coaster.tiles.len() == 0);
        for _ in 0..4 {
            coaster.tiles.push(bag.reserve.pop().unwrap());
        }
    }
}