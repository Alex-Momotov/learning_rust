#![allow(unused)]
use std::{collections::HashMap, thread::sleep, time::Duration};


/* 
General rules:
- An expression must supply a value on every path out of it. Applies to if, else if, else and loop with break.
- Only 'loop' can return a value with a 'break' - 'while' and 'for' can't (they always evaluate to () ). Why?
  Because 'loop' has exactly one exit path - 'break' (so return type can be known), but 'for' and 'while' 
  have another exit path - the loop ending, in which case the return type would be () which would be incompatible 
  with the 'break' return type.
*/

fn main() {

    // _____________________________________________________________________________________________
    // IF
	// No truthiness - no automatic convertion of other types to bool like 1 -> true, 0 -> false.
	if false {
		print!("false");
	} else if true {
		print!("true");
	} else {
		print!("else");
	}

	// IF IS AN EXPRESSION
	// If is an expression that returns:
	//    - the last value in each branch without semicolons. 
	//    - or an empty tuple () otherwise
	let a = if true { 1 } else { 2 };   // a is i32
	let a = if true { print!("hi") };    // a is ()     Omitting else is allowed here because no i32 vs () type confusion
	
	// That Means you can use it as ternary operator, provided that:
	// 1. All branches must return the same type
	// 2. Else is required, because otherwise the unspecified branch returns () - not compatible with the other type
	let a = if true { 1 } else { 2 };

	// _____________________________________________________________________________________________
	// BLOCKS
	// Blocks are expressions that return:
	//    - the last value without semicolons
	//    - or an empty tuple () otherwise
	let a = {1};   // a = 1
	let a = {1;};   // a = ()

	// Useful for scoping temporary computation.
	let a = {
	    // some complex math
		5
	};

	// LABLED BLOCKS
	// Gives you an early exit from a block
	struct User { active: bool, banned: bool }
    let user = User { active: true, banned: false };
	
    let status = 'check: {
        if !user.active   { break 'check "inactive"; }
        if user.banned    { break 'check "banned"; }
        "ok"
    };

	// _____________________________________________________________________________________________
	// LOOP 
	// The idiomatic infinite loop - loops until break or return.
	loop {
		print!("This will loop forever");
		break;
	}

	// LOOP IS AN EXPRESSION
	// 'loop' is an expression that returns: 
	//      - A value using 'break' - Think of break as 'return' - return can simply return or it can return a value. 
	//        Just like with 'if', all branches of 'break' must return the same type.
	//      - () if break; without a value
	//      - ! which is a never type - it unifies with everything.
	let a = loop { break 1; };     // returns i32
	let a = loop { break; };        // returns ()
	let a = loop { print!("hi") };   // returns !
	
	// Useful for retry-until succcess and backoff loops; event loops; accept() loops in servers; blocking on channels;
   	let conn = loop {
        match try_connect() {
            Ok(c)  => break c,
            Err(e) => { print!("{e}"); sleep(Duration::from_secs(1)); }
        }
    };

    // Can be used as a do-while loop, by placing the test is at the bottom
    loop {
        print!("some action");
        if 1 < 2 { break; }
    }

    // 'loop' can break out of labled loops while returning a value at the same time
    let result = 'outer: loop {
        let mut count = 0;
        loop {
            count += 1;
            if count == 5 {
                break 'outer count;   // breaks the OUTER loop, returning a value
            }
        }
    };

	// _____________________________________________________________________________________________
	// WHILE
	// It always evaluates to (), break can't return values. 
	while 1 < 5 {
        print!("hi");
	}

	// Typical counter loop
	let mut cnt = 0;
	while cnt < 10 {
		println!("cnt is {cnt}");
		cnt += 1;
	};

	// WHILE LET
	// Loop while a pattern keeps matching. This exists because a large number of APIs signal exhaustion with Option or Result. 
    let mut stack = vec![1, 2, 3];
	while let Some(i) = stack.pop() {
        print!("{i}");
	}

	// _____________________________________________________________________________________________
	// FOR
	// Always evaluates to (), 'break' can't return values.
	// _ means a throwaway placeholder.

	// OVER A RANGE         the standard way to "loop n times"
	for _ in 1..4 {}         // 1, 2, 3    - end exclusive
	for _ in 1..=4 {}        // 1, 2, 3, 4 - end inclusive
	for _ in (1..4).rev() {} // 3, 2, 1    - reversed

	// OVER A COLLECTION
	let mut v = [1, 2, 3];
	for x in &v {println!("{x}");}         // loops are functions that take v or &v or &mut v. If passing without & they kill the variable like any other function.
	for x in &mut v { *x += 1; }       // mutate elements in place

	// ENUMERATE
	let v: Vec<i32> = vec![1, 2, 3];
	for (idx, num) in v.into_iter().enumerate() { println!("{} {}", idx, num); }

	// NOTE, THIS PANICS
	let v: Vec<i32> = vec![];
	for i in 0..v.len() - 1 { }   // PANICS on an empty vec: 0usize - 1 underflows
	
	// OVER MAP OR TUPLES       destructure neatly
	let v = vec![("foo", 1), ("bar", 2), ("buz", 3)];
	for (word, count) in v { println!("{}, {}", word, count); }

	let map: HashMap<&str, i32> = HashMap::from([("foo", 1), ("bar", 2), ("buz", 3)]);
	for (k, v) in map { println!("{} {}", k, v);}

	// Tip to avoid deref (*i) everywhere in the loop:
	for &i in &vec![1, 2, 3] {  // option 1 (idiomatic): &i means i is the actual i32 in the loop
	    print!("{}", i);             // no need for *
	}
	for i in &vec![1, 2, 3] {  // option 2: plain i means i in the loop is &i32 and needs deref * everywehere which is more awkward
	    print!("{}", *i);
	}

	
	// OWNERSHIP AND FOR LOOPS 
	// ⭐ Think of loops as functions that take the collection:
	// for x in v {}		-> 		fn into_iter(v: Vec)  { v.iter(|x| ...) }		Kills variable like any other function
	// for x in &v {}		-> 		fn into_iter(v: &Vec) { v.iter(|x| ...) }		Variable survives. Function can read collection and its elements
	// for x in &mut v {}	-> 		fn into_iter(v: &Vec) { v.iter(|x| ...) }		Variable survives. Function can mutate collection and its elements
	let mut v = vec![1, 2, 3];
	for x in &v {	// think of it as 	fn some_func(v: &Vec<i32>) {}		Without & the function will kill v variable
		println!("{x}");
	}

	// The for loop desugars into IntoIterator::into_iter() function and just like any other function it can kill variables. Death by MOVING variable into the function.
	// for x in thing {}		IntoIterator::into_iter(thing)			Kills 'thing' variable, like any other function.		Death by moving for thing
	// for x in &thing {}		IntoIterator::iter(&thing)				'thing' survives. Can read 'thing' and its elements. 	Read party for thing and its elements
	// for x in &mut thing {}	IntoIterator::iter_mut(&mut thing)		'thing' survives. Can modify 'thing' and its elements. 	Write retreat for thing and its elements

	// A loop is just a function that consumes a collection (in v, &v, &mut v way) and hands you (element, &element, &mut element) one at a time for you to act on.
	// Remember from OWNERSHIP AND COLLECTIONS that borrowing a single element locks the entire collection and all elements in the same way (& or &mut).
	//   let mut v = vec![String::from("a"), String::from("b")];
	//   for s in v       { }  	death by moving 	s: String       OWR		s is a fully owned String and will die unless you move it somewhere
	//   for s in &mut v  { }  	write retreat   	s: &mut String  WR		s can be modified in place 
	//   for s in &v      { }  	read party      	s: &String      R		s can be read only

	let mut v = vec!["a", "b"];
	for s in v {}
	// println!("{:?}", v)	// ❌ ERROR - borrow of moves values

	let mut v = vec!["a", "b"];
	for s in &v {}
	println!("{:?}", v);	// ✅ Fine

	let mut v = vec!["a", "b"];
	for s in &mut v {*s = "c";}	// ✅ can even modify elements
	println!("{:?}", v);		// ✅ Fine
	
	// Copy types (array) are an exception because they are copied into the function, and survive 
	let mut v = ["a", "b"];
	for s in v {}
	println!("{:?}", v);	// ✅ Fine

	// _____________________________________________________________________________________________
    // LABLED LOOPS
    // Break or continue out of outer loops - works with loop, for, while.
    'outer: loop {
        'mid: for i in 0..100 {
            'inner: while 1 < 10 {
               	break 'outer;
               	continue 'mid;	// continue with the next iteration of the mid loop
            }
        }
    }

    // _____________________________________________________________________________________________
    
}

fn try_connect() -> Result<i32, String> {
    return Ok(1);
}
