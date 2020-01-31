mod data;

mod controller;
mod view;

fn main() {
    // For now, main will contain the Model...
    controller::do_stuff();
    view::present();
}
