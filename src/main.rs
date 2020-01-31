mod data;
mod controller;
mod view;

extern crate termcolor;

fn main() {
    // TODO For now, main will contain the Model...
    
    let mut view = view::View::new();

    let mut bag = controller::create_bag();
    let mut table = controller::create_table(2);
    let mut playerboards = controller::create_playerboards(2);

    println!("Entity Created.");

    controller::shuffle_bag(&mut bag);
    
    println!("Bag Shuffled:");

    view.present_bag(&bag);

    controller::setup_table(&mut bag, &mut table);

    println!("Table Setup:");

    view.present_table(&table);
    view.present_bag(&bag);

    for playerboard in playerboards.iter() {
        view.present_playerboard(&playerboard);
    }
  
}
