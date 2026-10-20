#![allow(unused)]
use std::ops::{Add, AddAssign};




fn main() {

    // _____________________________________________________________________________________________
    // COPY / MOVE / BORROW

    // COPY - Some types are copy (primitives)
    // MOVE - All other types are move - (e.g. String, Vec, custom struct)
    //        Functions kill - because move variables move inside.

    // BORROW - solution to variables being killed - &mut (exclusive write access) XOR & (shared read access)

    // & itself is a copy type
    let mut s = "hi".to_string();
    let r = &s;
    let r2 = r;             // r is copied (alive)
    
    // &mut itself is a move type
    let mut s = "hi".to_string();
    let m = &mut s;
    let m2 = m;         // m is moved (dead)

    
    // REBORROW
    // Syntax:      &mut *foo (reborrow as &mut reference)        &*foo (reborrow as & reference)
    // &mut borrowing is like passing an exclusive access to the borrower, but what if the borrower needs to pass that exclusive access to someone else before continueing its operation?
    // That's why reborrowing exists - the original borrower freezes (temporarily relinquishes it's access), and passes the exclusive access to another borrower. Once the other borrower
    // is finished, the original borrower unfreezes and continues. Since &mut reference is a move type, the reborrow syntax is to create a fresh &mut reference from the original value - &mut *foo.
    // See examples below. The only reborrowing you CANT do is reborrow & as &mut - original borrower cant give out more permissions than it itself has.

    // Reborrow &mut as &mut
    let mut s = "hi".to_string();
    let m1 = &mut s;
    let m2 = &mut *m1;  // Reborrow. m2 now has exclusive access, m1 is frozen.     
    m2.push('s');                    // After m2 is done with its writing, m1 unfreezes.            Think of it as a stack, s → m1 → m2, where only the top is usable.
    m1.push('k');                    // m1 is now unfrozen now holds exclusive access

    // Reborrow & as &      You can reborrow read references, but read reference is copy type, so there's no practical difference between reborrowing and copying it.
    let s = "hi".to_string();
    let r1 = &s;
    let r2 = r1;    // copy 
    let r3 = &*r1;  // reborrow

    // Reborrow &mut as &   You can reborrow &mut reference as & reference
    let mut s = "hi".to_string();
    let m1 = &mut s;
    let m2 = &*m1;          // reborrow &mut as &

    
    // AUTO REBORROW
    // Auto reborrow means the compiler automatically inserts the reborrow (&mut *foo or &*foo) in certain places, so we don't have to do it by hand.
    // It happens whenever &mut is expected (annotated).
    // Note: & reference gets auto reborrowed as well. But because & is copy type there is no practical difference between it being reborrowed or copied. So no implication of this.

    // 1. annotated variables
    let mut s = "hi".to_string();
    let m1 = &mut s;
    let m2 = &mut *m1;  // explicit reborrowing
    let m3: &mut String = m1;        // auto reborrowing (because of annotation) - compiler inserts '= &mut *m1' automatically
    m1;                              // m1 is still alive (not moved)
    
    // 2. function params
    fn foo(s: &mut String) {}
    let mut s = "hi".to_string();
    let m1 = &mut s;
    foo(m1);                        // auto reborrowing - compiler inserts 'foo(&mut *m1)' automatically
    m1;                             // m1 is still alive (not moved)
    
    // 3. method params
    struct Meow; impl Meow { fn foo(&mut self) {} };
    let mut a = Meow;
    let m = &mut a;
    m.foo();                       // auto reborrowing - compiler inserts Meow::foo(&mut *m);
    m;                             // m is still alive (not moved)
    
    // Where auto reborrow does NOT happen - wherever there is no explicit &mut annotation

    // 1. generic function / method params
    let mut s = "hi".to_string();
    let m1 = &mut s;
    fn bar<T>(a: T) {}          // generic function has no explicit &mut param annotation
    bar(m1);                    // m1 gets killed (moved)

    // 2. for loop - really case 1, because for loop desugars to a generic function consuming self, where self is the generic &mut param 
    let mut v: Vec<i32> = Vec::new();
    let m = &mut v;
    for i in m {}           // no auto reborrowing, m is dead 
    
	
	// REFERENCE OF REFERENCE
    // Why is reference of a reference possible if reborrowing is usually what you want?
    // Reference of a reference isn't a separate feature. It's the ordinary & / &mut operator applied to a variable that itself happens to be a reference. 
    // References are values, and you can borrow any value. Rust doesn't treat it specially.
    let mut s = "hi".to_string();
    let mut m1 = &mut s;
    let m2 = &mut m1;

    // When you actually want it, the difference between the two is:
    // - Reborrow, &mut *m: gives access to the target. You can change the String.
    // - Sub-borrow, &mut m: gives access to the reference itself. You can make it point somewhere else.

    // Reference of a reference is often used in parsers when the goal is to change what a reference is pointing to
    let mut rest: &str = "hello big world";   // &str means the its a read-only pointer to the string, meaning we can't modify the string itself. BUT this read only pointer itself is mutable - it can be made to point somewhere else.
    eat_word(&mut rest);   // rest is now " big world"

    fn eat_word<'a>(input: &mut &'a str) -> &'a str {
        let end = input.find(' ').unwrap_or(input.len());
        let word = &input[..end];
        *input = &input[end..];   // repoint the caller's &str further along
        word
    }
    
	// _____________________________________________________________________________________________
	// Syntax summary
	let mut s = "hi".to_string();

	let r1 = &s;        // read reference
	let r2 = r1;        // read reference (copied)
	let r3 = &*r1;      // read reference (reborrowed), no difference to copied
	let r4: &String = r1;        // read reference (implicitly reborrowed), no difference to copied
	let r5 = &r1;      // && reference of a reference
	
	let m1 = &mut s;    // &mut
	let m2 = m1;        // &mut moved, m1 dead
	let m3 = &mut *m2;  // &mut (reborrowed) as &mut
	let m4: &mut String = m2;        // &mut (reborrowed implicitly) as &mut
	let m5 = &*m2;          // &mut (reborrowed) as &
	let m6: &String = m2;            // &mut (reborrowed implicitly) as &
	let m7 = &m2;      // &&mut referene of a reference    

	// _____________________________________________________________________________________________
	// LOOPS and CONSUMING

	// Need to rewrite this part
   	// The for loop desugars into IntoIterator::into_iter() function and just like any other function it can kill variables. Death by MOVING variable into the function.
	// for x in thing {}		IntoIterator::into_iter(thing)			Kills 'thing' variable, like any other function.		Death by moving for thing
	// for x in &thing {}		IntoIterator::iter(&thing)				'thing' survives. Can read 'thing' and its elements. 	Read party for thing and its elements
	// for x in &mut thing {}	IntoIterator::iter_mut(&mut thing)		'thing' survives. Can modify 'thing' and its elements. 	Write retreat for thing and its elements

	
	/* 
	Vec<T>
        fn iter(&self) -> Iter {}                    
        fn iter_mut(&mut self) -> IterMut {}         
        fn into_iter(self) -> IntoIter {}       // iterates over T, consumes Vec / v 
        
	&Vec<T>
	    fn into_iter(self) -> Iter {}           // iterates over &T, copies reference &Vec, &v (vector still alive, reference &v still alive)
	
	&mut Vec<T>
	    fn into_iter(self) -> IterMut {}        // iterates over &mut T, consumes reference &mut Vec, &mut v (vector still alive, reference &mut v dead)

	The for loop calls into_iter depending on if it has v, &v, or &mut v.
	v       -> v consumed
	&v      -> reference &v copied,     original alive
	&mut v  -> reference &mut v moved,  original alive


	Moral of the story:
	    for i in v {}       is equivalent to calling    fn into_iter(self) {}  where self is Vec<>       so v is consumed
	    for i in &v {}      is equivalent to calling    fn into_iter(self) {}  where self is &Vec<>      so &v reference (which is created on the same line) is copied
	    for i in &mut v {}  is equivalent to calling    fn into_iter(self) {}  where self is &mut Vec<>  so &mut reference (which is created on the same line) is consumed
	
		- Why looping through the same &mut v reference twice doesn't work?
	        - when you do for i in &mut v {...} you create a reference on the same line, before its consumed
			- if somebody gave you let m = &mut v, and you call for i in m {...} that reference is consumed, so you can't do for i in m {...} again.
			- If you want to for-iterate through a let m = &mut v somebody gave you - you can't do &mut v (&mut &mut v), because into_iter() is not implemneted for &mut &mut Vec<T>.
			  That also is not what you want as it would be reference of a reference. Instead you need to subborrow with &mut *v (create another borrow of the original, which would be consumed on the same line):
			  for i in &mut *v {...}

		- How does for loop pick implementation?
		  The loop picks implementation based on what it is given: v, &v (reference), &mut v (mut reference), not what is actually written inline on the loop line.
		  So the last two lines below are equivalent (&Vec::into_iter() is selected). Only difference is whether the reference is created beforehand or inline.
            let mut v: Vec<i32> = Vec::new();
            let r = &v;
            for i in &v {}
            for i in r {}
	*/
	// _____________________________________________________________________________________________
    // How does &mut *v work? Why does it not conflict with "cannot move out of `` which is behind a mutable reference"? Is &mut *v reborrowing? I thought reborrowing is creating a reference of a reference like &mut &mut v.

    // _____________________________________________________________________________________________
    // PUTTING IT ALL TOGETHER - ITERATING OVER A v = &mut Vec twice
    // 1. for loop chooses Vec::into_iter(self) | &Vec::into_iter(self) | &mut Vec::into_iter(self) implementation based on what type is passed to it - v | &v | &mut v.
    // 2. &mut references are move type, and & references are copy type.
    // 3. for loop kills &mut references because it desugars to &mut Vec::into_iter(self) and moves the &mut v reference inside.
    // 4. so if we already hold a &mut reference it will be killed by the loop, we need to create another one for it (reborrow) - &mut *v.
    
    fn i_take_ref(v: &mut Vec<i32>) {
        for i in &mut *v {}     // for loop kills &mut reference, so we need to create another one for it on the same line - &mut *v (reborrowing) 
        for i in v {}
    }

    // _____________________________________________________________________________________________
      
}



