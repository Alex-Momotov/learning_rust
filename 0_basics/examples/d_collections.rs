#![allow(unused)]
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::Reverse;

pub fn main() {
	// _____________________________________________________________________________________________
	// ARRAY
	
    // Same type - Every element of array must be of the same type
    // Fixed length - once declared, arrays cannot change size.

	// Array type and size are part of the type - [i32; 5] is a different type to [i32; 6]. That means:
	// - Array size must be known at compile time. That means you cannot for example instantiate an array of random size at runtime.
	//   let arr = [0; rand_i];       // ❌ ERROR - length cannot be known at compile time
	// - You can't write a function that accepts an array of arbitrary size - which type would it be? There's no syntax for it.
	//   This is part of the reason why &[T] exists - You can specify &[T] in the function signature and it can take an array 
	//   of any size. This is because length is not part of [T] type - but rather part of its value - the length is runtime informaiton carried in the fat pointer's len field. 
	//   It's a fat pointer (prt, len) into the bytes behind an array or vector regardless of their size.
    
    let a = [1, 2, 3, 4, 5];        // literal
    let a: [i32; 3] = [1, 2, 3];    // [type; length] notation - to specify type of elements explicitly
    let a = [0; 5];                 // [value, length] notation - pre-fill an array with same value, in this case [0, 0, 0, 0, 0]

    // access elements
    a[0];

	// _____________________________________________________________________________________________
	// TUPLE

    // Can be different types - Tuple is a generic way of grouping together a number of values of different types. 
    // Fixed length - once declared, tuples cannot change size.
    let a = ();     // empty tuple, aka unit. Used as return sometimes
    let a: (i32, bool, char) = (1, true, 'a');
    let a = (1, true, 'a');
    let (x, y, z) = (1, true, 'a');     // Destructuring - unpack a tuple into variables
    
    let a = (1, 2, 3);
    a.0;    // access tuple by index
    a.1;
    a.2;

    let mut t = (1, 2, true);   // mutable tuples - can change value of elements, but not type in each position and also not length. 
    t = (1, 1, false);

    // Can be used to handily return multiple things from a function, like in Python
    fn sample_collections() -> (Vec<String>, HashMap<String, i32>) {
        let words: Vec<String> = vec!["one".to_string(), "two".to_string(), "three".to_string()];
        let counts: HashMap<String, i32> = HashMap::new();
        return (words, counts);
    }
    
    // Destructuring - functions, loops, variables, hashmaps
    let (words, counts) = sample_collections();   // destructuring from function that returns a tuple
    let (x, y, z) = (1, true, 'a');                                 // destructuring from a tuple
    for (word, count) in counts {println!("{} {}", word, count);}       // destructuring from a hashmap
    
    
	// _____________________________________________________________________________________________
	// VECTOR

	// A dynamic array. Growable, heap-allocated, all elements same type. Automatically grows (doubles) when capacity is exceeded.
	// - &v[1..3] is a SLICE (&[T]) - a borrowed view into a range, same idea as &str is to String.
	// - Functions take slices: fn f(v: &[i32]) accepts &vec and &array both.
	// - Iterate with &v (borrow) - 'for x in v' without & MOVES the vec into the loop, killing it.

	let mut v: Vec<i32> = Vec::new();               // empty        (needs type - nothing to infer from)
	let mut v: Vec<i32> = Vec::with_capacity(100);  // empty with given capacity
	let mut v: Vec<i32> = (0..5).collect();         // from a range [0, 1, 2, 3, 4]
	let mut v = vec![1, 2, 3];                      // literal
	let mut v = vec![0; 5];                         // pre-filled   [value; length]
	
	v.push(4);              // add at the end
	v.insert(0, 99);        // add at index      insert(idx, element)    PANICS if idx doesnt exist!     (shifts the rest - O(n))
	v.extend([7, 8]);       // add many
	
	v.pop();        // remove + return last   -> Option<i32> (None if empty)
	v.remove(0);    // remove + return at index (shifts - O(n))     PANICS if idx doesn't exist
	
	v[0];           // Access by idx          PANICS
	&v[0..1];       // Access by range        PANICS    Note, [T] can't exist on its own, so & is needed 
	v.get(0);       // Access by idx (safe)   Option
	v.get(1..3);    // Access by range (safe) Option
	v.contains(&2); // Contains               note the & - it wants a reference

	v.len();        // length
	v.is_empty();   // is empty

	v.sort();                                       // sort in place
	v.sort_by(|a, b| b.cmp(a));                     // sort by lambda value - useful for sorting complex values by some key
	v.sort_by(|a, b| a.partial_cmp(b).unwrap());    // floats: plain .sort() doesn't compile (NaN has no order)
	v.reverse();                                    // reverse in place
	
	// Vec<String> gotchas 
	let names = vec![String::from("ann"), String::from("bob")];
	let e = names.contains(&String::from("ann"));   // works but allocates a String just to compare
	let e = names.iter().any(|s| s == "ann");       // better - no allocation

	// _____________________________________________________________________________________________
	// HASHMAP

	let mut map = HashMap::from([("a", 1), ("b", 2)]);                // literal      (array of tuples; no vec!-style macro exists)
	let mut map: HashMap<&str, i32> = HashMap::new();               // empty
	let mut map: HashMap<&str, i32> = HashMap::with_capacity(100);  // with capacity

    map.len();               // length

    map.insert("tokio", 20); // insert   Overrides if key already exists
    map.remove("bob");       // remove   Returns Option - Some if value was present, None if it was absent

    map.contains_key("tokio");          // Python's 'k in d'
    
	map["tokio"];            // get item            PANICS if key not found     (note, you can't &mut borrow it - your options are V or &V for copy types and &V or V.clone() for move types)     
	map.get("tokio");        // get item            Option. THE way to read.
	map.get_mut("ann");      // get item mutably    Option
	if let Some(v) = map.get_mut("ann") { *v += 1; }  // Update item if it exists                              The idiomatic way. (if let = match with one arm)
	*map.entry("a").or_insert(0) += 1;                          // Update item if it exists, insert first if doesnt.     The idiomatic counter pattern
	
	for (k, v) in &map {}         // Iterate key, value pairs.    Arbitrary order
	for k in map.keys() {}              // Iterate keys
	for k in map.values() {}             // Iterate values
	let keys: Vec<&str> = map.keys().copied().collect();    // collect keys
	let vals: Vec<&i32> = map.values().collect();           // collect values

	// _____________________________________________________________________________________________
	// SET
	let mut set: HashSet<i32> = HashSet::new();                 // empty
	let mut set: HashSet<i32> = HashSet::with_capacity(100);    // with capacity
	let mut set = HashSet::from([1, 2, 3]);       // literal
	let mut set: HashSet<i32> = (1..100).collect();             // from a range

	set.insert(5);
	set.contains(&5);
	set.remove(&5);
	let s: HashSet<i32> = vec![1, 2, 2, 3].into_iter().collect();   // dedup a vec

	// Set algebra ------------------------------
	let a: HashSet<i32> = HashSet::from([1, 2, 3]);
	let b: HashSet<i32> = HashSet::from([2, 3, 4]);
	let both: HashSet<&i32> = a.intersection(&b).collect();    // also .union() .difference()
	let e = a.is_subset(&b);
	for x in a.intersection(&b) {}      // or just iterate, no collect needed

    // _____________________________________________________________________________________________
    // STACK = VECTOR
    let mut stack = vec![1, 2];
    stack.push(3);      // push last                O(1)
    stack.pop();        // pop last     -> Option   O(1)
    
    // _____________________________________________________________________________________________
    // QUEUE = VecDeque
    let mut q: VecDeque<i32> = VecDeque::new();                     // empty
    let mut q: VecDeque<i32> = VecDeque::with_capacity(100);        // with capacity
    let mut q: VecDeque<i32> = vec![1, 2, 3].into_iter().collect(); // from vector
    
    q.push_back(1);     // Push last (enqueue)
    q.pop_front();      // Pop first (dequeue)      Option<i32>
    
	// _____________________________________________________________________________________________
	// CONVERTING Vector <-> HashMap <-> HashSet
	// Mental model: Every collection can dump itself into a stream of items (into_iter()), and every collection can be built
    // from a stream (collect()). So conversion is always: source → iterator → collect into target.

    // Note for the below: .copied() for copy types, .cloned() for move types, but .cloned() works for both.
    // For any conversion there are 4 ways to do it:
    //                              New collection      Original collection     Use when
    // .into_iter().collect()       V elements          dies                    Default. Don't need original any more. No new allocations.
    // .iter().collect()            &V elements         lives                   When new collection is a temporary "read index" and the original needs to outlive it.
    // .iter_mut().collect()        &mut V elements     lives                   When new collection is a temporary "read/write helper" and the original needs to outlive it.
    // .iter().cloned().collect()   clone/copy          lives                   Need both new and original, and need them independent. New allocation for each element.

    // Exceptions to the table:
    // - HashMap and HashSet keys cannot be mutated, so: 1. no iter_mut() on them; 2. HashMap iter_mut() pairs are &K, &mut V;
    // - HashMap iter() returns (&K, &V), not &(K, V), so to make a copy you need to: map.clone().into_iter() or deref manually 
    //   map.iter().map(|(k, v)| (*k, *v)).collect()
    // - HashMap has separate iterators for keys and values:
    //      iter()      iter_mut()      into_iter()
    //      keys()      ❌              into_keys()
    //      values()    values_mut()    into_values()
    
    // -----------------------------
    // VECTOR   
    // Vector -> Set
	let mut v = vec![1, 2, 3];
	let set: HashSet<&i32> = v.iter().collect();            // & borrow 
	let set: HashSet<&mut i32> = v.iter_mut().collect();    // &mut borrow  
	let set: HashSet<i32> = v.iter().copied().collect();    // clone/copy   Note: it's .cloned() for move types
	let set: HashSet<i32> = v.into_iter().collect();        // move
   
	// Vector -> HashMap
	let mut v = vec![(1, 1), (1, 2), (1, 3)];
	// let map: HashMap<&i32, i32> = v.iter().collect();            // & borrow 
	// let map: HashMap<&mut str, i32> = v.iter_mut().collect();    // &mut borrow  
	let map: HashMap<i32, i32> = v.iter().copied().collect();    // clone/copy   Note: it's .cloned() for move types
	let map: HashMap<i32, i32> = v.into_iter().collect();        // move

	// -----------------------------
	// SET      
	// HashSet -> Vector
	// Note: no iter_mut() because Set elements are not modifyable - after modifying they'd sit in the wrong bucket
	let mut set = HashSet::from([1, 2, 3]);
	let v: Vec<&i32> = set.iter().collect();
	let v: Vec<i32> = set.iter().copied().collect();
	let v: Vec<i32> = set.into_iter().collect();

	// -----------------------------
	// HASHMAP  
	// HashMap -> Vector        (example with k,v pairs - iter() variants)
	let mut map = HashMap::from([(1, 1), (2, 2)]);
	let v: Vec<(&i32, &i32)> = map.iter().collect();
	let v: Vec<(&i32, &mut i32)> = map.iter_mut().collect();
	let v: Vec<(i32, i32)> = map.clone().into_iter().collect();     // Note, you can't do iter().cloned() because iter() gives (&K, &V), not &(K, V)
	let v: Vec<(i32, i32)> = map.iter().map(|(k, v)| (*k, *v)).collect();     // Same thing
	// let v: Vec<(i32, i32)> = map.into_iter().collect();
	
    // HashMap -> Set           (example with keys - keys() variants)
    let mut map = HashMap::from([(1, 1), (2, 2)]);
    let set: HashSet<(&i32)> = map.keys().collect();
    let set: HashSet<(i32)> = map.keys().cloned().collect();
    let set: HashSet<(i32)> = map.into_keys().collect();


}
