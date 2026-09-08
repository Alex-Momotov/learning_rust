#![allow(unused)]

/* 
General rules:
- ⭐ When to use each construct
	if let              Use if let for a genuine one-branch side effect.
	let else            Use let else for predictions at the top of a function. So that the rest of the function can continue with all those predictions passed and all the variables extracted.
	match               Use match when the variants are all real cases, and you need exhaustiveness check.
	while let           Loop until Option/Result runs dry
	loop + break value  Exit condition in the middle or you need a value out
	labled loops        Escape more than one level

- Refutable vs Irrefutable patterns - Fancy sounding concept, but it simplifies to the following:
  - All patterns consist of: 1. Destructuring (taking it out of the box), can't fail 2. Binding (can't fail) 3. Testing (can fail).
  - Constructs that support Testing patterns must have a second alternative branch for when the pattern test fails. Because otherwise the variables it binds would be uninitialised.
  - let, for, fn params                     Don't have alternative branch       Therefore don't support Testing patterns
  - match, if let, while let, let else      Have an alternative branch          Therefore support Testing patterns

*/

fn main() {
   	// _____________________________________________________________________________________________
	// IF LET
	// If let is a match with one arm you care about. The trade-off is that you give up exhaustiveness checking.
	// On pattern match - unpacks the pattern into the block and runs the block.
	// On pattern failure - skips the block and continues.

	let opt = Some(1);

	if let Some(v) = opt {
        println!("Got {v}!");
    }
    
    // If let {} else if let {} else
	if let Some(p) = opt {
        
    } else if let None = opt {

    } else {

    }

	// Its an expression - it can return a value. That means all branches must return the same type
	// But a combinator (like .unwrap_or()) is ususally better. Clippy will complain
    let a = if let Some(v) = opt { v } else { 0 };  // Works
    let a = opt.unwrap_or(0);                            // But this is better

    // If let shines we care only about one side of an enum
    if let Err(e) = flush_to_disk() {
        print!("error: {}", e);
    }
    
    // Extract value from an enum   (common use case)
	enum State { Stopped(i32), Running(i32) }
	let state = State::Running(10);
	
	if let State::Running(v) = state {
        println!("{:?}", v);
	}

	// _____________________________________________________________________________________________
	// LET ELSE
	// The inverse of if let - bind on success, diverge on failure. It MUST DIVERGE - meaning it must return, break, continue, or panic!.
	// On pattern match - unpacks the pattern into the outside, and skips the block.
	// On pattern failure - runs the block as the new exit path of the scope - therefore it must return the same type as the current scope.

	let Some(c) = load_config() else {
	    return;
	};
	c;  // <- c is now in scope for the rest of the function

	// Use let else for predictions at the top of a function. So that the rest of the function can continue with all those predictions passed and all the variables extracted.
	fn handle(req: Request) -> Result<String, Response> {
        let Some(user) = req.user else { return Err(Response::Unauthorized); };
        let Some(id)   = req.id else { return Err(Response::BadRequest); };
        let Ok(id)     = id.parse::<u64>() else { return Err(Response::BadRequest); };
        // ... the entire body at one indent level
        Ok(id.to_string())
    }

   	// _____________________________________________________________________________________________
	// LET CHAINING
	// The LET IF and WHILE LET support LET CHAINING. Let chains are just several patterns or boolean
	// conditions joined by &&. All the patterns and conditions must be true for the whole pattern to match.
	
    if let Some(x) = opt && x > 3 && let Ok(y) = Result::<i32, &str>::Err("hi") {
        println!("hi");
    }

    while let Some(x) = opt && x > 3 && let Ok(y) = Result::<i32, &str>::Err("hi") {
        println!("hi");
    }
   
	// _____________________________________________________________________________________________
	// MATCH
	// Match compares a value against patterns, top to bottom, and runs and returns the first arm that fits.
	// 1. It's an expression, meaning it returns the result the arm, and all arms must return the same type.
	// 2. It's exhaustive, meaning it forces you (at compile time) to handle all variants of an enum. Or have a catchall for unhandled ones.
	// 3. It destructures matched pattern into the arm. ⚠️ And this destructuring can move the variable.

	let opt = Some(1);
	let a = {
	    match opt {
			Some(v) => v,     // Arm's body can be an expression (comma is required) or a block (comma is optional)
			None         => { 0 },
		}
	};

    // It's an expression, so it returns the value from the arm that runs. That also means all arms must return the same type. 
    let desc = match 300 {
        200 => "ok",
        404 => "missing",
        _   => "other",
    };

    // Match arm bindings have same borrow level as matched variable
    // ⚠️ Think of match's arm bindings as function call sites (remember functions can kill move variables, and copy by value the copy variables) - they will borrow/move/copy your variables. 
    // Note: since '_' doesn't make bindings, it won't move/copy anything.
    // match x       -> x.m binding becomes x.m         move/copy
    // match &x      -> x.m binding becomes &x.m        read
    // match &mut x  -> x.m binding becomes &mut x.m    mutate
    let mut x = Kitty { m: String::from("Kitty"), c: 123 };
    
    match &x { Kitty { m, c} => {} }              // x.m is &x.m       x.c is &x.c      read
    match &mut x { Kitty { m, c} => {} }  // x.m is &mut x.m   x.c is &mut x.c  borrow
    match x { Kitty { m, c} => {} }                 // x.m is moved      x.c is copied    move/copy
    match x { _ => {} }

    // ⚠️ Implication - bindings for copy types copy the value. So you can't modify the underlying value in the match arm - you'd be modifying a copy. 
    // Correct way to modify copy variables when pattern matching - match on &mut x and modify with *x.c += 1.
    let mut x = Kitty { m: String::from("Kitty"), c: 123 };
    match x { Kitty { mut c, ..} => { c += 1} }         // ⚠️ You're updating a copy
    match &mut x { Kitty { c, ..} => { *c += 1} }  // ✅ Updating correctly

    // Be careful with _. It catches everything so when you add a new enum there will be no error on match statements.
    // Prefer listing all enum variants rather than having _ whenever possible.

	// _____________________________________________________________________________________________
	// FUNCTION PARAMETERS
	// Destructuring can happen in function parameters 

	// Function that takes a tuple
    fn take_tuple(t: &(i32, i32)) {
        let (x, y) = t;
        println!("{}, {}", x, y);
    }

    // Same thing but deconstruction happens in function signature
    fn take_tuple2(&(x, y): &(i32, i32)) {
        println!("{}, {}", x, y);
    }
    
	// _____________________________________________________________________________________________
	// PATTERN MATCHING

	// What problem does pattern matching solve? Needing to: 
	//  1. test a variable's identity - enum, value      (what instanceof normally does in other languages)
	//  2. extract the data                              (what .get() chains normally do in other languages) 
	//  3. exhaustively check all identity variants      (what other languages don't even have, introducing silent bugs)
	// In other languages this looks like a giant nested if-else tree with lots of instanceof() and .get() chains and null checks.
	// Verbose, ceremoneous, hard to read, and error prone. Rust replaces all of that with pattern matching.
	
	// A pattern is a tree of nested nodes, where each node does one of three things: 
	//   Destructure Opens a container                (a, b)          Tuple           
	//                                                P { a, b }      Struct          
	// 				                                  &x          	  Reference       
	//   Bind        Binds value to a variable        Some(v)         Enum variants
	// 				                                  some_val        (all lowercase - matches everything)
	//					                   			  x @ pattern     (test and bind at the same time - if test_pattern matches, bind the value to x)
	//                                                _               (match any value, discard); 
	//                                                ..              (match any number of values, discard)
	//   Test        Compares against a known value   1, "s", true    Literals        
	//                                                1..10           Ranges          
	//                                                None, Some(v)   Enum variants   

	match 1 {
	    // Literals and ranges
	    0            => {} ,
		1 | 2 | 3    => {} ,
		4..=6        => {} ,
		7..10        => {} ,

		// Catchalls
		lowercase_var  => {},  // all lowercase variable    -> matches everything can use as placeholder
		_                   => {},  // _                         -> matches everything, can't use as placeholder
	}

	// Destructuring - structs
    struct Point { x: i32, y: i32, }
    let point = Point { x: 1, y: 2 };
    match point {
        Point { x, y}   => {},    // bind all
        Point { x, .. }      => {},    // bind some '..' means ignore the rest
        Point { x: 0, y: 0 }      => {},    // literal inside a struct pattern
        Point { x: 0, y }    => {},    // y is bound
    };

    // Destructuring - enums
    enum Message {
        Quit, 
        Move {x: i32, y: i32 },
        Write(String),
        ChangeColour(i32, i32, i32),
    }
    let msg = Message::Write(String::from("hello"));

    match msg {
        Message::Quit => {},
        Message::Move { x, y } => {},
        Message::Write(text) => {},
        Message::ChangeColour(r, g, b) => {},
    }

    // Destructuring — tuples
    let t = (1, 2, 3);
    match t {
        (x, y, z)  => {},  // bind x, y z
        (x, .., z)      => {},  // bind x, z only
        _                         => {},
    }

    // Destructuring - nested
    let nested: Result<Option<State>, String> = Ok(Some(State::Running(1)));
    match nested {
        Ok(Some(State::Running(n)))  => {},
        Ok(_)                             => {},
        Err(_)                            => todo!(),
    }
    
    // Destructuring - References
    let reference = &4;
    match reference {
        &val => {},  // &x dereferences each &i32 item, binding x: i32
    }

    // Test two separate things at the same time
    let mut setting = Some(1);
    let new_setting = Some(2);

    if let (Some(s), Some(s_n)) = (&mut setting, new_setting) {
        *s = s_n;
    }

    // GUARDS
	// - A guard is a conditional logic after a pattern to further specify if the branch should match.
	//   It exists because while handling logic in one arm (after =>) you might want to jump to another arm - depending on:
	//   a comparison to an outside runtime variable; a comparison between two bindings; a method call.
	// - The tradeoff is that the compiler can't know if you handled all cases in your guard, so you have to specify a no-guard fallback.

	match point {
        Point {x, y} if x == y  => {},
        Point {x, ..} if x % 2 == 0  => {},
        _                                 => {},
    };

    // _____________________________________________________________________________________________
    
}

fn flush_to_disk() -> Result<(), String> {
    return Ok(());
}

fn load_config() -> Option<String> {
    return Some("config".to_string());
}

struct Request {
    user: Option<String>,
    id: Option<String>
}

enum Response {
    Unauthorized,
    BadRequest
}

enum Cat { 
    Kitty { m: String, c: i32 } 
}
use Cat::*;