#![allow(unused)]

use std::collections::HashSet;
fn main() {

    /* 
    _____________________________________________________________________________________________
    ITERATORS
    - Chain shape:   SOURCE  →  ADAPTERS (lazy)  →  CONSUMER (drives it)
    - The three ways in:
            call      │ yields │   collection after   │
        v.iter()      │ &T     │ still yours          │ for x in &v → iter(), 
        v.iter_mut()  │ &mut T │ still yours, mutated │ for x in &mut v → iter_mut(), 
        v.into_iter() │ T      │ consumed             │ for x in v → into_iter()
    - Every pipeline must end in a consumer.
    - Iterators are lazy - No work happens until a consumer (.collect(), take(), count(), for_each(), find()) runs.
    - When to use: Use them as the default for any traversal, transformation, filtering, or aggregation over a sequence. 
    - You can't mutate the collection's length while iterating it (the borrow checker forbids it).
    */
    
    let mut v = vec![1, 2, 3, 4, 5];

    // Enumerate loop
	for (i, x) in v.iter().enumerate() {}   // with index
    
   	// Aggregate
	let n: i32 = v.iter().sum();
	let m = v.iter().max();             // Option<&i32>, also .min()
	let e = v.iter().any(|x| *x > 2);   // Python's any(...)
	let e = v.iter().all(|x| *x > 0);   // Python's all(...)

	// _____________________________________________________________________________________________

	/* 
	_____________________________________________________________________________________________
	Transform
	map, 
	filter, 
	filter_map (filter+map in one, keep the Somes), 
	flat_map, 
	flatten, 
	copied/cloned (turn &T into T, fixes half your type errors), 
    rev, 
    zip, 
    enumerate, 
    chain, 
    take, 
    skip, 
    take_while, 
    skip_while, 
    step_by,
    peekable, 
    scan (fold that yields each step), 
    inspect (debug print mid-chain).

    _____________________________________________________________________________________________
    Consume → one value: 
    sum, 
    product, 
    count, 
    min/max, 
    min_by_key/max_by_key, 
    fold (seed + accumulator), 
    reduce (fold using first item as seed, returns Option), 
    last, 
    nth, 
    find, 
    find_map, 
    position, 
    any, 
    all, 
    for_each.

    _____________________________________________________________________________________________
    Consume → a collection: 
    collect (into Vec, String, HashMap, HashSet, BTreeMap…), 
    partition (split by predicate into two), 
    unzip (pairs → two collections).
    _____________________________________________________________________________________________

    5. collect() needs to know the target type — annotate or turbofish:

        let v: Vec<i32>       = it.collect();
        let s: String         = it.collect();      // from chars / &str
        let m: HashMap<K, V>  = it.collect();      // from (K, V) pairs
        let set: HashSet<_>   = it.collect();
        let n = it.sum::<i32>(); 

        
    6. The killer collect: Result<Vec<T>, E> from Vec<Result<T, E>>
        — stops at the FIRST error and returns it.

        let nums: Result<Vec<i32>, _> = strs.iter().map(|s| s.parse::<i32>()).collect();
        let nums = nums?;                          // works with Option too

    7. retain = filter in place
    v.retain(|x| *x > 0);            // filter in place
    */

	
	



}