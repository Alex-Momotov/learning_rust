use std::ops::AddAssign;



fn main() {
    let mut s = "hi".to_string();
    let mut idx_last_valid = s.chars().count() - 1;

    println!("{:?}", idx_last_valid);
    println!("{:?}", s.chars().take(idx_last_valid + 1).collect::<String>());

    s.pop();
    println!("{:?}", s);

    let s = "hi".to_string();
    let s2: String = s.chars().take(0).collect();
    println!("{:?}", s2);

    // _____________________________________________________________________________________________
    // String slices - simple

    // String slices - byte indices 

    // _____________________________________________________________________________________________
    // Auto-dereferencing
    let mut a: i32 = 1;
    let r1 = &a;
    let r2 = &r1;
    let r3 = &r2;

    let mut b: i32 = 1;
    let mut m1 = &mut b;
    let mut m2 = &mut m1;
    let m3 = &mut m2;

    r3.abs();       // r3 is technically &&&i32, so auto-dereferencing does allows us to call the method straight on the underlying value rather than doing manual deref 3 times: ***r3
    (***r3).abs();  // desugared

    // Only method calls dereference. Assignments, passing a variable as parameter, and arithmetic operations doesn't do that
    fn i_take_num(n: i32) {}

    i_take_num(***r3);      // Passing as parameter - Have to manually dereference N times until we reach the bare value
                            // i_take_num(r3);      won't work
    let c: i32 = ***r3;     // Assignments - Have to manually dereference N times until we reach the bare value
                            // let b: i32 = r3;     won't work
    let _ = ***r3 + 1;      // Arithmetic operations (simple read) - Have to manually dereference N times until we reach the bare value
                            // r3 + 1;  won't work
    ***m3 += 1;             // Arithmetic operations (mutata a number) - Have to manually dereference N times until we reach the bare value
                            // m3 += 1;  won't work

    // Auto-dereferencing (String example)
    let mut s = "hi".to_string();
    let mut m1 = &mut s;
    let mut m2 = &mut m1;
    let m3 = &mut m2;
    
    (m3).chars().count();       // m3 is technically &mut &mut &mut String. No need to do ***m3 due to dereferencing.
    (***m3).chars().count();

    
    // _____________________________________________________________________________________________
    // Reference vs value; when deref is needed; three ways to mutate through reference

    // 1. You can mutate a value directly through a mutable reference (even a primitive)
    let mut x = 10;
    let reference = &mut x;     // e.g. this is an input to a function or a for loop

    *reference += 1;                // DO THIS

    let mut a = *reference;    // NOT THIS (you'd be modifyign a copy)
    a += 1;

    // 2. You can reassign through a mutable reference
    fn i_take_mutable(s: &mut String) {
        s.push('n');                    // you can obviously mutate the actual string (only works wihout dereferencing because it's a method call)
        *s = "new stuff".to_string();   // reassigning through mutable reference
        *s += " ";                      // string concatenations and 
    }

    // 3. You can do += through a mutable reference
                            
    // _____________________________________________________________________________________________
    // Mutable variable means two things: 
    let mut s = "hi".to_string();
    s.push('!');            // 1. you can mutate it directly 
    s = "bye".to_string();  // 2. you can reassign it to a different value of the same type (without shadowing)
    // Another perspective of the above rule: reassign it to a different value of the same type is a mutating operation.
    // Meaning:
    //  1. it must be mut
    //  2. you can't do it in a loop that does into

    // ---------------
    // You can even do that through a mutable reference - String example
    let mut o = "hi".to_string();
    let s = &mut o;
    
    s.push('!');             // 1. mutate through method call            (no deref needed, because through method call)
    *s = "bye".to_string();  // 2. mutate through assigning              (deref needed)
    *s += "!";               // 3. mutate through concat operation       (deref needed)

    // ---------------
    // You can even do that through a mutable reference - number example
    let mut i: i32 = 0;
    let m = &mut i;

    m.add_assign(10);   // 1. mutate through method call            (no deref needed, because through method call)
    *m = 10;            // 2. mutate through assigning              (deref needed)
    *m += 1;            // 3. mutate through arithmetic operation   (deref needed)
    

    // _____________________________________________________________________________________________
    // Reborrow vs move
    // Reborrow - means borrow a borrowed value, after which it returns to the first borrow, and eventually to the ogiginal

    let mut a = 10;             // original
    let mut m = &mut a;    // mut reference
    
    let j = &mut m;    // reborrow (m is still alive after j borrow ends)
    let k = m;              // move (m is dead/moved to k)

    // _____________________________________________________________________________________________
    // Loops - mutating while iterating

    // 1. for i in v       consumes v
    
    // 2. for i in &v      v elemnts are read only
    let mut s = "hi".to_string();
    for ch in s.chars() {
        
    }
    
    // 3. for i in &mutv   v elemnts can be mutated during iteration
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 1;
    }

    // Other ways to mutate elements and also the collection itself that you're iterating over
    // 1. for - through direct indexing
    for i in 0..v.len() {
        v[i] += 1;      // can mutate elements
        v.push(i);      // can mutate collection itself
    }

    // 2. while loop
    while !v.is_empty() {
        v[0] += 1;  // can mutate elements
        v.pop();    // can mutate collection itself
    }

    // 3. loop
    loop {
        v[0] += 1;  // can mutate elements
        v.pop();    // can mutate collection itself
    }
    
}

