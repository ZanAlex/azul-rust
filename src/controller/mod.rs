use rand::Rng;
use crate::data::{
    Tile,
    Bag
};

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