use c::*;

pub struct Cat;

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}
