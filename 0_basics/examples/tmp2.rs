#![allow(unused)]

use rand::random_bool;



fn main() {

    // _____________________________________________________________________________________________
    // The DECISION whether to create a value or not can come at RUNTIME or COMPILE TIME, it doesn't matter. Neither need dyn Trait.
    // This is because every value occupies a different "place", so there's never a conflict.

    // Compile-time decision to create a value.
    let c = Cat;    // always runs, decided at compile time
    
    // Runtime decision to create a value.
    let mut v: Vec<Cat> = Vec::new();
    if rand::random_bool(0.50) {
        v.push(Cat);                // Every runtime-decided Cat occupies a different "place" - different index of the vector.
    }


    // When we DO need dyn Trait - is when the same place needs to refer to potentially different Trait implementation values. 
    // Whether the decision to create that underlying implementation value came from compile time or runtime is irrelevant.

    // Whats important is that we're trying to put different implementations into the same "place". 
    // Not whether we decided to create the implementing values at compile time or runtime.
    
    // Compile-time decision to create a value. More importantly, same "place" holding different implementations
    let mut a: Box<dyn Animal> = Box::new(Cat);
    a = Box::new(Dog);

    // Runtime decision to create a value. More importantly, same "place" holding different implementations
    let mut v: Vec<Box<dyn Animal>> = Vec::new();
    if random_bool(0.50) {
        v.push(Box::new(Cat));
        v.push(Box::new(Dog));
    }

    // _____________________________________________________________________________________________


    // _____________________________________________________________________________________________


    // _____________________________________________________________________________________________
    
    
}

trait Animal {
    fn sound(&self);
}

struct Cat;
impl Animal for Cat { 
    fn sound(&self) { println!("Meow!"); } 
}

struct Dog;
impl Animal for Dog { 
    fn sound(&self) { println!("Woof!"); } 
}

struct Forest<T: Animal> {
    animal: T
}