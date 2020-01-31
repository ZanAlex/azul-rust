mod data;
mod controller;
mod view;

fn main() {
    // TODO For now, main will contain the Model...
    
    let mut bag = controller::create_bag();
    controller::shuffle_bag(&mut bag);

    print!("Bag = {}", bag);
}
