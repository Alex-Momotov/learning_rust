use std::ops::{Add, AddAssign};

fn main() {
    let mut o = "hi".to_string();
    let s = &mut o;
    
    s.push('!');            // 1. you can mutate it directly 
    *s = "bye".to_string();  // 2. you can reassign it to a different value of the same type (without shadowing)
    *s += " !";
    
    println!("{:?}", o);
    // _____________________________________________________________________________________________

    let mut i: i32 = 0;
    let m = &mut i;

    m.add_assign(10);   // 1. mutate through method call            (no deref needed because through method call)
    *m = 10;            // 2. mutate through assigning              (deref needed)
    *m += 1;            // 3. mutate through arithmetic operation   (deref needed)
    
    
}