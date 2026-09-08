/*
Quick Mental Model
- Object - 	a "thing on the heap" that has many references to it. Having references to the thing is part of the definition of an 
			object, so therefore Rust doesn't use the Object term.
- Value (Rust's equivalent) - the "thing on the heap" itself, not a reference to a thing. Think of it as a unique-ownership handle 
		to a resource. When you pass it around you either hand it over (move, you no longer have it), lend it (& / &mut borrow - 
		you get it back), or photocopy it (Copy/.clone()). 
		Both copy types and move types are "values". "This is a value of type String, this is a value of type i32."
- & - shared read access
- &mut - exclusive write access

----------------------
Context:
Every language must answer: when is heap memory freed?
- C - you call free() yourself" — get it wrong and you have use-after-free and double-free bugs.
- Python, Java - "a garbage collector figures it out at runtime" — safe, but you pay with a runtime, GC pauses, and memory overhead.
- Rust - the new idea - the compiler proves at compile time where every piece of heap memory should be freed and inserts the free for you. 
         No GC, no manual calls, no runtime cost. The code that would be a memory bug doesn't compile at all. 

Stack vs Heap
- Stack - In Rust every variable lives inside the function's stack frame by default. When function returns, it's stack frame is popped and everything in it vanishes - that part of memory management is free and automatic. 
- Heap - Is opt in: Box::new(value) moves the value to the heap and the stack variable holds only a pointer. Now copying the variable copies just the pointer (cheap) - but it raises teh question the ownership model answers: two pointers, one heap allocatiion - who's responsible for freeing it?

Types:
- Copy types - Integers, floats, bools, chars, tuples, arrays - live on the stack and are copied on assignment. There's no heap data to own, and nothing to enforce.
               Tuples and arrays are only Copy types if all their elements are Copy types. (i32, bool) and [f64; 5] — Copy. (i32, String) and [String; 3] — move.
			   ⭐ Allocation to a different variable copies the value, and both variables are alive and usable.
			 - Stack-only values (integers, floats, bools, chars) are copied on assignment, not moved. let b = a; println!("{b}");   is fine for i32. 
			   There's no heap data to own, and nothing to enforce. 

- Owning types - Anything owning heap data: String, Vec, HashMap, Box. 
			   ⭐ Allocation to a different variable moves ownership of the heap data. The previous variable is dead and unusable (compile error).
			   ⭐ Every heap allocation is owned by exactly one variable. 
			   ⭐ When the owner goes out of scope (its stack frame is popped), Rust frees the heap data.
			      This single rule is the entire memory manager. But this only works if "exactly one owner" is actually enforced.
				  This is the Rust's answer to the question of: who's responsible for freeing the heap memory if there are two pointers to the same data? -> There can never be two pointers to the same data.
			   ⭐ Assignment or passing to a function MOVES ownership - the old variable is dead, using it = compile error.
				  Doing let b = a means now b owns the heap data and a is now dead - using it is a compile error. 
				  Function calls are similar. Doing f(s) moves s into the function, and it's gone from the caller. 
				  In other languages doing b = a means copying the reference to the same object.
			   ⭐ That means a function can "kill" variables fn I_kill_variables(some_var: Box<i32>) {}    This is why borrowing exists. 
			   ⭐ .clone() = explicit deep copy when you really want two independent values.
				  Clone: if you genuinely need both heap variables usable - .clone() makes a second independent heap allocation (deep copy of the data).
				  Rust makes you write it explicitly because it's an expensive operation. 

Java vs Rust ⭐⭐⭐
- Java: Primitive types get copied by value. Reference types get copied by reference.
- Rust: Primitive/copy types get copied by value. Reference/move types get copied by reference, but the old reference dies. This is because in order to deterministically prove that the heap data should be freed when a variable goes out of scope there must exist only one reference to that data at any given time.

- Java: primitives copy by value; objects copy by reference (aliases — both names stay live, mutations visible through both).
- Rust: Copy types copy by value; owning types move — the handle's bits are copied and the old name dies. Exactly one owner may exist at any time, so the compiler can prove where heap data must be freed: at the end of the owner's scope.

----------------------

If passing a String to a function kills your variable, how do you ever call a function twice?
You lend instead of give: &s creates a reference — a non-owning pointer.
The function borrows the data, uses it, and when the reference dies... nothing happens. No free, because the reference never owned anything; 
The owner back in the caller is untouched and stays valid.
Sharing exists in Rust, but only in this leashed form.
The act is called borrowing, and it's what you'll do in 95% of function signatures.

Aliasing and Mutation
- Pointer Safety Principle: data must never be aliased AND mutated at the same time.
  ⭐ Any number of readers (&T) XOR exactly one writer (&mut T) may exist at any given time.


Every variable has up to three permissions: R - read, W - write (only if declared mut), O - own (the permission to give the thing away (MOVE) or let it drop).
Borrowing temporarily strips permissions from the source.
- &x (shared borrow): x loses W and O — frozen read-only while borrowed. Many &x can coexist; readers don't hurt each other.
- &mut x (mutable borrow): x loses R, W, and O — completely locked; the &mut is the only path to the data
  while it lives. That exclusivity is what makes mutation safe: a writer with no aliases can't invalidate anyone.
  
  - Every borrow-checker error is some version of "you tried to use a permission that's currently lent out"
  and the error messages literally speak this language ("cannot borrow v as mutable because it is also borrowed as immutable").
  
- ⭐ Permissions flow back when the borrow ends. At the borrow's LAST USE, not end of scope.
- ⭐ Borrows end at last use, not end of scope. A reference's lifetime runs from creation to the last line that
  uses it — after that, the owner unlocks, even mid-function.

- ⭐ Data must outlive its references.
  	- Can't move/drop the owner while borrowed
	  	let v = vec![1, 2, 3];
		let r = &v;          // v loses W and O
		// drop(v);          // error: cannot move out of `v` because it is borrowed  (drop needs O)
		// let w = v;        // error: same - a move needs O
		println!("{r:?}");   // r's last use - v gets O back below this line
		drop(v);             // fine now
	- Can't return a reference to a function's local. The local dies with the frame, the reference would dangle. 
	  fn myfunc() -> &Box<i32> {}      <- won't compile

- reference = non-owning pointer. &x borrows x instead of taking ownership -> owner stays alive.
   When a reference dies nothing is freed - it never owned the data.

⭐ Essentially variables transition between states of existance where each state consists of one variable and one or more borrows each with a set of permissions.:
	Mutable variable   let mut x = Box::new(5);
	- x (RWO)  ->  x (R), &x (R), &x (R), ...   ->  x (RWO)  when all &x die (last usage)
	- x (RWO)  ->  x (), &mut x (RW)  			->  x (RWO)  when &mut x dies (last usage)
	Immutable variable let x = Box::new(5);
	- x (RO)   ->  x (R), &x (R), &x (R), ...  ->  x (RO)  when all &x die (last usage)

⭐ Here's a story of one happy variable:
	- The variable gets born
		let mut a = Box::new(5);
	- "Read Party": Then it multiplies itself into several read only things
		let r1 = &a;
		let r2 = &a;
	  	a (R), r1 (R), r2 (R)
	  Then after the last usage of r1 and r2, a gets its permissions back and returns to normal
	- "Write Retreat": Then it lends its RW powers to a single other variable/reference
		let b = &mut a;
	    a (), b (RW)
	  b does some importarnt reading and writing work and finishes.
	  Then after the last usage of b, a gets its permissions back and returns to normal.
	- Then it dies in one of the following ways.
		- It goes out of scope at the end of the function OR gets deleted with drop(a) OR gets shadowed by another variable (let a = Box(5);) (heap data is freed)
		- It is moved to another variable 
		  let c = a;  	// a dies, c (RWO)
		  func(a)		// a dies, func variable gets ownership 
  The middle steps can happen as many times as you want, in any order as long as they are between when it is born (first step) and when it dies (last step).

Questions:
- What does dying mean? Not going out of scope but last usage I assume?
- In the permissions model what does O (Own) mean? practically speaking what does it allow? Dying and moving? What does that mean? I suppose moving means being assigned to a different var like (let b = a)? But then what does dying mean?

*/ 

#![allow(unused)]


fn main() {
	// _____________________________________________________________________________________________
	// QUICK MENTAL MODEL
	// A variable can have as many read parties (shared read access) and write retreats (exclusive write access) as you want, as long as they don't overlap. 
	
	let mut a = String::from("hello");

	// read party ------------------------------
	let b = &a;         
	let c = &b;
	let d = &c;	// read references can reference each other
	
	// write retreat ------------------------------
	let tmp = &mut a;
	tmp.push_str(" world");   
	
	// read party ------------------------------
	let e = &a;
	let f = &a;

	// write retreat ------------------------------
	let bar = &mut a; 
	bar.push_str("!");

	print!("{a}");

	// _____________________________________________________________________________________________
	// SUB-BORROWING
	// "The story of one happy variable" is recursive, meaning &mut and & variables have O,W,R permissions 
	// themselves, and can hold their own read parties and write retreats. 

	let mut a = String::from("foo");	// a is born

	// write retreat ------------------------------
	let mut b = &mut a;	// b is born
	
	// sub-read party ------------------------------
	let br1 = &b;				// sub-borrows of b -> b can't write because it lost W, and O
	let br2 = &b;				
	println!("{br1} {br2}");	
	
	// sub-write retreat ------------------------------
	let mut bw = &mut b;
	bw.push_str("hi");

	b.push_str(" bar");
	let t = b;			// b dies. It would die normally anyway after last usage
	
	// read party ------------------------------
	let c = &a;
	let d = &a;
	println!("{c} {d}");	// a dies 

	// _____________________________________________________________________________________________
	// OWNERSHIP AND SLEF METHODS
	// ⭐ Think of self functions as functions that take the variable - self (kills), &mut self (write), &self (read)
	let mut v = vec![1, 2, 3];
	v.push(4);	// Think of it as	fn some_func(v: &mut Vec)	Since we are creating a &mut borrow, no other borrows can exist during this operation

	// Value modifying methods such as v.push() (Vector) or s.push_str() (String) are just functions that take &mut self
	// and this is calling those modifying methods is equivalent to trying to create &mut reference (not allowed when already borrowed).
	let mut v = vec![1, 2, 3, 4, 5];
	let r = &v;

	// v.push(6);				❌ Not allowed - equivalent to the below because it desugards to fn push(&mut self, value: T) 
	// modifying_func(&mut v);	❌ Not allowed - for usual reasons

	println!("{:?}", r);

	// _____________________________________________________________________________________________
	// THE MOVE/BORROW/CLONE TRILEMMA
    // You have two variables A and B, and variable B "wants something" from A. You really have three options.
    // Basically choose two: (no new allocations, both variables are independent, both variables live)

    // OPTION 1: MOVE - no new allocations, new variable is independent, original dies
    let a = "foo".to_string();
    let b = a;
    
    // OPTION 2: BORROW - no new allocations, variables are "tied-up", both live
    let a = "foo".to_string();
    let b = &a;

    // OPTION 3: CLONE - new allocation, variables are independent, both live
    let a = "foo".to_string();
    let b = a.clone();

    // When to use each:
    // OPTION 1 (move)   → DEFAULT. A was a stepping stone to B; you're done with A.
    //                     Moves are how data flows FORWARD through a program.
    // OPTION 2 (borrow) → B only needs to LOOK (or briefly edit via &mut), and A
    //                     outlives the looking. All borrow-checker errors are this
    //                     option's fine print: A frozen while B lives; B ⊂ A's lifetime.
    // OPTION 3 (clone)  → B must be independent AND A must survive: B crosses a
    //                     boundary A won't (returned, stored, sent to a thread).
    //                     A clone is buying independence with an allocation — fine
    //                     when deliberate, a smell when it's just silencing E0382.
    // Rule of thumb: move by default; borrow to look; clone to escape.

    // MOVE     When A is no longer needed
    // BORROW   When you only need to look or mutate and A outlives the looking
    // CLONE    When A needs to live and B needs to cross a boundary that A won't: B is returned, stored, or sent to a thread
    
    // Note: Copy types (i32, bool, char…) exit the trilemma: `let b = a` copies — all
    // three properties at once, free. The trilemma binds only for resource owners.

    // Prompt:
    // I need an intuivive principle for understanding the three options using an analogy of a physical object:
    // For example you have DVD disk. You can:
    // 1. Put it into a new box, the old box is empty/dead
    // 2. Lend it to a friend, and tell them they can only read it, or tell them they can etch new data onto it, before returning it to you
    // 3. Copy the entire disk onto a new disk. An expensive operation since you need to buy a new one

	// _____________________________________________________________________________________________
	// POINTERS
	// What are &mut and &? &mut and & are pointers to the thing.
    // Are they pointers themselves move or copy types? &mut is move and & is copy
    
    let mut s = String::from("");    // String       the String itself
    let r1 = &s;                    // &String      pointer to String. Pointer itself is Copy type
    let r2 = r1;                    // ✅ copy — r1 still usable
    let r3 = r2;                    // ✅ copy of a copy — r1, r2 are still usable
    
    let m1 = &mut s;            // &mut String  pointer to String. Pointer itself is Move type
    let m2 = m1;                // move - m1 is now dead.
    // m1;                                   // ❌ error - use of moved value

    let mut s2 = String::from("");
    
    // ----------------------
    // What does it mean for a pointer itself to be mut?
    let mut s = String::from("");   // The String itself is mutable.
    let m1 = &mut s;           // The pointer to a String is immutable - it can never point to something else
    let r1 = &s;
    let mut m2 = &mut s;       // The pointer itself it mutable - we can make it point to a different String
    let mut r2 = &s;

    // m1 = &mut s2;    // ❌ error - can't make it point to a different String
    // r1 = &s2;        // ❌
    m2 = &mut s;        // ✅ mutable pointer can be made to point somewhere else
    r2 = &s2;           // ✅

    // So what does 'mut a' mean?
    // if its a value - the value itself is mutable.
    // if its a pointer - the pointer itself is mutable - can be made to point to something else.

    // So what does mut a: &mut b; mean?
    // It means it's a &mut pointer to another variable, and the pointer itself is mutable (can be made to point to something else)
    
	// _____________________________________________________________________________________________
	// ⚠️ MODIFYING A COPY WHILE THINKING ITS ORIGINAL
	// Two things must be true:
	// 1. The place/binding you're modifying is T and not &mut T.
	// 2. The type you're modifying is Copy (or the copy/cloning happened in a non-obvious way inside a function call).

	// Place 1: Pattern matching - modifying the binding
	enum Colour { White {r: i32} }
	let mut c = Colour::White { r: 10 };
	match c {
        Colour::White { mut r } => r += 1,    // ❌ Binding r is i32 (T and not &mut T) AND you're modifying a Copy type (i32)
	}
	match &mut c {
        Colour::White { r } => *r += 1,  // ✅ Binding r is &mut i32 (&mut T)
	}

	// Place 2: For loops
	let mut v = vec![1, 2, 3];
	for mut i in v {         // ❌ Binding i is i32 (T and not &mut T) AND you're modifying a Copy type (i32)
	    i += 1;
	}
	let mut v = vec![1, 2, 3];
	for i in &mut v {   // ✅ Binding i is &mut i32 (&mut T)
	    *i += 1;
	}

	// Place 3: function parameters
	fn take_copy(mut a: i32) {  // ❌ Binding is i32 (T and not &mut T) AND you're modifying a Copy type (i32)
	    a += 1
	}
	fn take_ref(a: &mut i32) {  // ✅ Binding is &mut i32 (&mut T)
	    *a += 1
	}

	// Place 4: Something that came out of an index
	let mut v = vec![1, 2, 3];
	let mut x = v[0];         // ❌ Binding x is i32 (T and not &mut T) AND you're modifying a Copy type (i32)
	x += 1;
	let mut v = vec![1, 2, 3];
	let x = &mut v[0];   // ✅ Binding is &mut i32 (&mut T)
	*x += 1;

	// Place 5: Something that came out of a function or instance method
	let a = SomeStruct { x: 10 };
	let mut b = a.some_method();
	b += 1;         // ❌ Binding b is i32 (T and not &mut T) AND you're modifying a Copy type (i32)
	let a = SomeStruct { x: 10 };
	let b = &mut a.some_method();
	*b += 1;        // ✅ Binding is &mut i32 (&mut T)

	

	// Tip: a hint that this is happening is that you declared the new binding itself as 'mut x: T' instead of 'x: &mut T'.
	// Most of those cases can be solved by taking the binding as 'x: &mut T' instead of 'mut x: T'.
	
	// _____________________________________________________________________________________________
	// QUICK NOTES

	// ⭐ Borrowing a copy type
	// Why are you allowed to borrow copy types? e.g. &i32
	// Because a reference isn't "the way to avoid moving" — it's a pointer to A PLACE.
	// Copying gives you a snapshot of the value; borrowing gives you access to the location itself.
	// Those are different tools for differetn situations 

    let mut x = 5;
    let r = &mut x;
    *r += 1;              // I want to CHANGE the original — a copy wouldn't do

	fn modify_num(x: &mut i32) {    // no access to x in this function but we can still modify the original
	    *x = *x * *x;
	}
	modify_num(&mut x);

	
	
	
	// ---------------------------
	let a = Box::from(5);
	let b = &a;
	let c = &b;	// this is fine. & borrows can borrow each other

	// Variable references cannot outlive the variable 
	fn create_string() -> String {
		let s = String::from("hi");
		// return &s;	-> ERROR because &s (reference) would outlive the data (s) which is not allowed. 
		return s;	//  -> Fine because ownership will move from s to the caller assigned variable
	}
	let s2 = create_string();	// Ownership moves from s inside the function to s2 outside the function.
	let s2_ref = &s2;			// Fine because it references s2 which is alive.


	
	// _____________________________________________________________________________________________
	// Stack vs heap

	// Stack variables / Copy types
	// - values live directly in the function's stack frame by default (unlike Python where everything is an object)
    // - stack values vanish for free when the function returns
	let a = 5;                  // Stack - because i32 is a copy type
	let b = [0; 1_000_000];   	// Stack - because all elements of array are copy types
	let c = (1f64, 2i32, 'a');	// Stack - because all elements of tuple are copy types

	// Heap variables / Move types
	let a = Box::new(5);   		// Heap - because Box is an owning type
	let b = vec![1, 2, 3];      // Heap - because Vec is an owning type
	let c = String::from("Hi"); // Heap - because String is an owning type

	// ----------------------------------------------------
	// Stack variables / Copy types are copied by value on assignment. Not moved, no heap data to own
	let x = 5;
	let y = x;
	println!("{x} {y}");    // both alive - x was just copied

	// ----------------------------------------------------
	// Move types
	// Death by moving - a dies, assignment transfers ownership of heap data to b
	let a = Box::new(5);
	let b = a;              // heap data moves a -> b. a is dead now.
	// println!("{a}");     // compile error: "value used here after move"

	// Death by moving - into a function parameter - a dies, the function parameter becomes the new owner
	let a = Box::from(5);
	i_kill_variables(a);    // a moves into the function, gone from here
	// println!("{a}");     // uncomment - compile error "value borrowed here after move"

	// Clone - explicit deep copy, original stays alive. Explicit because it's the expensive option.
	let a = Box::new(5);
	let b = a.clone();
	println!("{a} {b}");  // both fine

	// ----------------------------------------------------
	// & - lend instead of give (solves "function call kills my variable" - I_kill_variables())
	// Both the function usage and the function signature must use & to indicate borrowing. 
	let a = Box::new(5);
	let r1 = &a;                // a (R), r1 (R)
	let r2 = &a;                // a (R), r1 (R), r2 (R)
	println!("r1: {}, r2: {}", r1, r2);

	let len = i_dont_kill_variables(&a);     // lends a
	println!("{a} {len}");      // a still alive and usable

	// * dereferences, but the dot operator does it for you - you rarely write * or &
	let b = Box::new(-1);
	let a1 = i32::abs(*b);      // explicit deref
	let a2 = b.abs();           // implicit - same thing
	let s = String::from("hello");
	let l = s.len();            // implicit in the other direction: str::len(&s)

	// shared refs - many simultaneous readers are fine
	let v = vec![1, 2, 3];
	let r1 = &v;
	let r2 = &v;
	println!("{} {}", r1[0], r2[0]);

	// while borrowed, the owner is frozen (lost W+O)
	let mut v = vec![1, 2, 3];
	let first = &v[0];
	// v.push(4);           // uncomment - compile error: push could reallocate -> first would dangle
	println!("{first}");    // last use of first - v unlocks below this line
	v.push(4);              // fine here

	// &mut - the exclusive writer
	let mut s = String::from("hello");
	let m = &mut s;
	// println!("{s}");     // uncomment - compile error: s fully locked while m is live
	m.push_str(" world");
	println!("{m}");        // last use of m - s unlocks
	println!("{s}");        // fine now

	// references must not outlive their data:
	// fn dangle() -> &String { &String::from("oops") }  // compile error: missing lifetime specifier
	let b = String::from("Hello, world!");

}

fn i_kill_variables(some_var: Box<i32>) {
    println!("I killed the variable with value: {}", some_var);
}   // some_var (the owner) goes out of scope here -> heap data freed

fn i_dont_kill_variables(a: &Box<i32>) -> bool {
	return **a > 10;
}   // the reference s dies here - nothing freed, this function never owned the data

fn modifying_func(v: &mut Vec<i32>) {
	v.push(1);
}

struct SomeStruct { x: i32 }
impl SomeStruct { fn some_method(&self) -> i32 { self.x } }