#![allow(unused)]
use std::collections::HashMap;
use uuid::Uuid;

fn main() {

   	// _____________________________________________________________________________________________
	// RANGE

	// Range is an actual type, it's used in slicing, iteration, and pattern matching. 
	// The ../..= syntax is just literal syntax for constructing values of range type. Range expressions:
	//  1..5;      Range           — 1, 2, 3, 4        (half-open: start included, end excluded)
    //  1..=5;     RangeInclusive  — 1, 2, 3, 4, 5     (..= includes the end)
    //  1..;       RangeFrom       — 1 to "the end" (or infinity, as an iterator)
    //  ..5;       RangeTo         — start to 4
    //  ..=5;      RangeToInclusive— start to 5
    //  ..;        RangeFull       — everything

    // (10..1).rev()        reversed
    // (10..1).step_by(2)   step by 

    // You can bind it to and use it as a normal variable
    let r = 1..;
    let v = vec![1, 2, 3];
    let slice = &v[r];      // Returns &[T] type - note [T] can't exist on it's own, so hence & is needed.

    // ________________________________
    // Use case 1: slicing / indexing
    &v[1..2];

    // ________________________________
    // Use case 2: Iteration

    // iterate N times
    for i in 0..100 {       // Note, no need for parentheses
        println!("{}", i);
    }

    // classic index loop
    for i in 0..v.len() {
        println!("{}", v[i]);
    }

    // similar to list comprehension
    let squares: Vec<i32> = (1..=10).map(|x: i32| x.pow(2)).collect();  
    let uuids: Vec<Uuid> = (0..100).map(|_| Uuid::new_v4()).collect();  // use _ for 'throwaway variable', otherwise compile warning
    let random_numbers: Vec<i32> = (0..1000).map(|_| rand::random_range(1..10_000)).collect();
    
    // ________________________________
    // Use case 3: Match patterns
    let age = rand::random_range(1..100);
    match age {
        0..12   => "child",
        12..17  => "teen",
        _       => "adult",
    };


    
    
	// _____________________________________________________________________________________________
	// SLICE
	// [T] means "some continuous run of Ts, of a length not known at compile time, living SOMEWHERE - binary, 
	// stack, heap, mid-buffer, doesn't matter". It is the pure content. str (without &) means the same thing.
      
	// And because the compiler doesn't know its size, you can never hold a [T] or a str directly in a variable.
	//   Every local variable lives in a stack frame, and the compiler must know each variable's size to lay the
	//   frame out. So:
      
	// let x: [i32] = ...;   // ❌ doesn't compile — "doesn't have a size known at compile-time"
	// let s: str   = ...;   // ❌ same error
      
	// ⭐ &[T] is a fat pointer (ptr, len) into the bytes of continuous run of Ts living SOMEWHERE - binary, 
	// heap, stack, mid-buffer, doesn't matter. There are three layers:
	// 	 str / [T] 			The bytes themselves, wherever they live (unsized, can't hold in variable directly)
	//   &str / &[T] 		The fat-pointer view of those bytes (what you actually pass around)
	//   String / Vec<T> 	An owner that allocated those bytes on the heap and must free them
	// In daily code you only ever write the bottom two, which is why the two-type model works — but [T] is the
	// answer to "a view of what, exactly?"
      
	// ⭐ Reasons for &[T] to exist:
	// 1. Sub-ranges — identical to strings: &v[1..4] is a piece of a bigger buffer that no container owns. A
	//    &Vec<T> demands a full owner-struct that isn't there; only a fat pointer (ptr, len) can describe it.
	// 2. Multiple owner types — this replaces the "literals" reason, and it's stronger. Strings have one owner
	//    (String); sequences have two: Vec<T> (heap) and [T; N] (stack array). &Vec<i32> can't view an array, and
	//    &[i32; 3] can't view a Vec or accept any other length — the view must be origin-agnostic to serve them all.
	//    (Literals still count too: &[1, 2, 3] is baked into the binary like "foo" — but it's a sub-case of this.)
	// 3. Unification — the consequence of 1+2: one signature fn f(xs: &[T]) accepts array, Vec, sub-slice, and
	//    literal at once. &Vec<T> is the same deficient borrow as &String: double indirection, and it rejects most of
	//    those sources.
	// So: same theory, but for strings the view exists mainly because of ownerless bytes (literals), while for
	// slices it exists mainly because there's more than one kind of owner.
      
	let mut v = vec![1, 2, 3];
	let mut a = [1, 2, 3];
      
	let mut b1: &[i32] = &mut v;
	let mut b2: &[i32] = &mut a;
	
	
	// Functions of &[T] vs functions of Vec:
	// Almost a total overlap - functions available on the slice [T] are automatically available on the vector Vec<T> becuase 
	// Vec<T> automatically derefs to [T]. So the real question is what does Vec<T> add on top? -> only the owner powers:
	//   - Change length: push, pop, insert, remove, clear, truncate, extend, retain, drain, resize
	//   - Manage capacity: reserve, shrink_to_fit, capacity, with_capacity
	//   - Give away ownership: consuming into_iter() (yields T not &T), into_boxed_slice
	// The mutating-but-not-growing ones (sort, reverse, swap, fill, iter_mut) sit on [T] too — they need &mut [T],
	// but they only rearrange elements in place, never change the length, so a view suffices.
	
	// Rule of thumb in your language: if it touches the buffer's size or its ownership, it's a Vec method; if it
	// only reads or rearranges what's already there, it's a slice method — and this is exactly why fn f(xs: &[T])
	// costs you almost nothing: you're giving up only the grow/shrink/consume surface.
      
	// When to use &[T] vs &mut [T] vs &Vec<T> in function signatures:
	// 	  &[T] 			viewing 
	// 	  &mut [T] 		viewing, sorting, rearranging
	//	  &mut Vec<T>	vector specific operations that can changing length, manage capacity, or give away ownership: e.g. push, pop, capacity, ...
      
	
	// ⭐ What's the role of [T] in day to day use?
	// 1. Mostly function signatures - function that takes: 1. array of any arbitrary length; 2. vector; 3. subslice of array of vector;
	//	  Here you see &[T] or &mut [T] directly in the function signature. 
	// In other use cases of [T] the [T] type is inferred and hence not directly written in the code:
	fn i_take_anything(slice: &[i32]) {}
	fn i_take_anything2(slice: &mut [i32]) {}
	i_take_anything(&[1, 2, 3]);			// array
	i_take_anything(&[1, 2, 3][0..2]);		// array slice
	i_take_anything(&vec![1, 2, 3]);		// vector
	i_take_anything(&vec![1, 2, 3][0..2]);	// vector slice
      
	// 2. Sub-range views - any &v[1..5] local is a &[T], you just don't write the type.
	let slice = &v[2..];      // type is &[i32], inferred
      
	// 3. This is also possible - a variable holding array or vector depending on some condition
	let s: &[i32] = if true {&[1, 2, 3]} else {&vec![1, 2, 3]};
      
	// _____________________________________________________________________________________________
	// MUTABILITY AND COLLECTIONS
	// ⭐ Elements of a collection inherit its mutability permissions
   
	// MUTABLE collection -> the collection itself and all it's elements are MUTABLE
	let mut v = vec![1, 2, 3];
	v[0] = 10;        // ✅ modify any element
	v.push(4);        // ✅ modify the collection itself
   
	// IMMUTABLE collection -> the collection itself and all it's elements are IMMUTABLE
	let v = vec![1, 2, 3];   // no `mut`
	// v[0] = 10;            // ❌ CANT modify any element
	// v.push(4);            // ❌ CANT modify the collection itself
   
	// _____________________________________________________________________________________________
	// OWNERSHIP AND COLLECTIONS - ELEMENT BORROW
	// ⭐ Can't move element out of a vec (if it's move type)   -> clone, remove, or use &names[0]
	let mut v = vec![Box::from(1), Box::from(2), Box::from(3)];
	// let s = v[0];    // ❌ can't move element out of a vec

	// This is also a reason why HashMap get() returns a & reference of a value, regardless of what value type it is, even if it's a copy value.
	// 1. You can't move a value out of a collection -> it has to be cloned, removed or referenced (& or &mut)
	// 2. HashMap get() signature is generic    pub fn get(&self, k: &Q) -> Option<&V>
	// 3. So because HashMap values can be of move type, the return type must be a reference (& or &mut)
	// 4. Since you can't have generic code that's different depending on type (copy vs move) 
	//    the copy values are returned as references as well

	// Its fine to do it for copy types as they are copied
	let mut v = vec![1, 2, 3, 4, 5];
	let n = v[0];       // ✅ this is fine
	
	// ⭐ Borrowing a single element makes the whole collection considered as borrowed in the same way (& or &mut)
	let mut v = vec![1, 2, 3, 4, 5];
	let a = &mut v[0];
	// let b = &mut v[1];	// ❌ CANT borrow because collection is already considered as borrowed via 'a'
	// let c = &v[1];		// ❌ CANT borrow because collection is already considered as borrowed via 'a'
	println!("{a}");
	
	// _____________________________________________________________________________________________
	// OWNERSHIP AND COLLECTIONS - COLLECTION BORROW
	// &mut collection borrow CAN modify the collection and its elements
	let mut v = vec![1, 2, 3];	
	let w = &mut v;
	w[0] = 10;	// ✅ modify any element	
	w.push(5);	// ✅ modify the collection itself
	
	// & borrow collection CANNOT modify the collection or its elements
	let mut v = vec![1, 2, 3];	
	let w = &v;
	// w[0] = 10;	// ❌ CANT modify any element
	// w.push(5);	// ❌ CANT modify the collection itself

	// _____________________________________________________________________________________________
	// OWNERSHIP AND COLLECTIONS - TWO COLLECTIONS CAN BE "tied up"
	// Two collections can be "tied up" if one contains references from the other, meaning
	// you treat the whole collection and it's elements as a & or &mut borrow of the other collection
	// Collection B can't outlive collection A; as long as b is alive you can't mutate or drop A;
	// Use it when both live in the same scope
	let mut a = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];
	let mut b: Vec<&str> = Vec::new();

	b.push(a[0].as_str());  // Now the collections are "tied-up"
	b.push(a[1].as_str());

	// a.push("hi".to_string());    // ❌ CANT modify the collection, as it has a read party
	// a[0].push_str("hi");         // ❌ CANT modify elements, as the collection has a read party
	
	println!("{:?}", b);

	// Real example
	// words is now "tied up" to counts, so long as counts lives (until last usage)
	// Think of it as: words is the owner, counts is an index over it
    let words: Vec<String> = vec![String::from("one"), String::from("two"), String::from("three")];
    let mut counts: HashMap<&str, i32> = HashMap::new();

    for word in &words {
        *counts.entry(word).or_insert(0) += 1;
    }

    // _____________________________________________________________________________________________
    // OWNERSHIP AND COLLECTIONS - ONE COLLECTION INDEXING ANOTHER - THE MOVE/BORROW/CLONE TRILEMMA
    // When you want to have one collection "index" another collection (hold elements from it) you really have three options.
    // Basically choose two: (no new allocations, two collections are independent, both collections live)
    
    // OPTION 1 - no new allocations, new collection is independent, original collection dies
    // Efficient because no new allocations are made per element
    let words: Vec<String> = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let mut counts: HashMap<String, i32> = HashMap::new();
    for word in words {                         // C
        *counts.entry(word).or_insert(0) += 1;          // entry takes V so must pass collection as is C
    }

    // OPTION 2 - no new allocations, two collections tied up, both live
    // Note, the collections will become "tied-up", but often it's acceptable.
    // Think of it as: words is the owner, counts is an index over it
    let words: Vec<String> = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let mut counts: HashMap<&str, i32> = HashMap::new();
    for word in &words {               // &C
        *counts.entry(word).or_insert(0) += 1;  // entry takes &V so must pass collection as &C, no clone
    }
    
    // OPTION 3 - new allocation per element, two collections independent, both live
    // Clone the key before entry() - collections are independent
    let words: Vec<String> = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let mut counts: HashMap<String, i32> = HashMap::new();
    for word in &words {
        *counts.entry(word.clone()).or_insert(0) += 1;
    }

    // When to use each:
    // OPTION 1 (move in)      → DEFAULT. Source was scaffolding you don't need again.
    //                           Most "build a Vec, then aggregate it" pipelines end here.
    // OPTION 2 (borrow keys)  → both needed in the same scope, map dies with the source
    //                           (count then print/compare against original). Borrow
    //                           checker enforces: source frozen while map lives, map
    //                           can't outlive source (can't be returned without it).
    // OPTION 3 (clone keys)   → map must OUTLIVE the source: returned from a fn that
    //                           owns the source locally, stored in a struct, sent to
    //                           a thread. You're paying allocations to buy independence.
    // Rule of thumb: reach for 1; drop to 2 when the source must survive;
    // pay for 3 only when the map leaves the source's scope.

    // _____________________________________________________________________________________________


    
}

