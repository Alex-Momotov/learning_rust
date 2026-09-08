/* 
OWNED vs BORROWED-VIEW DUALITY

⭐ Summary2: &str is just a more powerful way of borrowing String, and should be used in 99% of the cases. 
			More powerful because it can also read literals and string slices. 
			There aren't really "two string types" — there's one string (the bytes) and two ways to hold it: owning it (String) or 
			borrowing it (&str) (string just happens to have a more powerful way of borrowing it than other types).

⭐ Summary: The view is a separate type because it must describe bytes that no container owns (literals + sub-slices).
		    The consequence: because a view is origin-agnostic, one function taking &str (or &[T]) 
			accepts all sources at once — literal, sub-slice, owned buffer — through a single signature.
			That's why the idiom is ⭐ "take the view, give the owner": fn f(s: &str) -> String.
			⭐ Own with the fat struct, borrow with the fat pointer.

Terminology:
- pointer - a address in memory which points at a specific memory byte. A "thin" pointer.
- fat pointer - a memory address PLUS something else -> e.g. memory address + length + capacity.
  Slice references (&str, &[T]) -> (address, length). The length is there to know how many bytes/elements the view spans.
  It has to carry this because a view into "part of a buffer" needs to know where it starts AND how far it goes. 
  ⭐ So &str being a "fat pointer" means: it's an address plus a length, packed together.

The two types:
For any kind of data that lives in a contiguous chunk of memory, Rust provides two types: 
- An owner that holds and frees the buffer.
	Lives on the heap, carries (ptr, len, capacity), can grow.
- A view that borrows a look at some bytes without owning them.
	Points into memory owned by someone else, carries just (ptr, len), cannot grow.

  ┌──────────────────────────────────┬──────────────────────────────────┐
  │ Owns the buffer (heap, growable) │ Views bytes (fat pointer, fixed) │
  │ String                           │ &str                             │
  │ Vec<T>                           │ &[T]                             │
  │ PathBuf                          │ &Path                            │
  │ OsString                         │ &OsStr                           │
  └──────────────────────────────────┴──────────────────────────────────┘

⭐ Why the view has to be its own type? 
Because a view must be able to point at bytes that no owning container backs. Two things have no owner:

  1. Literals — "foo", [1, 2, 3] — baked into the binary; no owner exists.
  2. Sub-ranges — &s[1..4] — a slice of a bigger buffer; no container owns just that piece.

A view type (&str, &[T]) can describe both (literals and sub-ranges) because it's just "a pointer and a length into bytes, wherever they live."
An owner-reference (&String, &Vec<T>) can't — it demands a full owner-struct that, for a literal or a sub-slice, isn't there.

⭐ General principle: an owned growable heap buffer is a fundamentally different thing from a view of a contiguous 
region, and views must be able to exist over data that no container owns — literals baked in the binary, and sub-ranges of a bigger buffer.

⭐ Reasons why view type has to exist (for strings, vectors, etc):
- Literals like "foo", [1, 2, 3] have no heap-allocated owner to point at, so can't be referred to as &String because there is no String owner.
  &str solves this: it's "a view of some UTF-8 bytes, wherever they live." The bytes can live in the binary (a
  literal), on the heap (inside a String), anywhere. &str doesn't require an owning container to exist — that's the whole point of it.
- Sub-ranges like substrings &s[1..4] have no heap-allocated owner to point at either
  &str solves this because its just a pointer to an existing buffer.
- Unification - one type accepts all sources
  Because &str means "view of bytes regardless of origin", a single function singature accepts every kind of string
  This flexibility is why the idiom is "take &str".
	fn print(s: &str) { ... }
	print("literal");             // bytes in the binary
	print(&owned_string);         // bytes on the heap (String coerces to &str)
	print(&owned_string[2..5]);   // a substring — bytes mid-buffer

--------------------------------------------------------------------------------------------------
⭐ &str isn't a third type — it's a more powerful borrow form, replacing &String.
More powerful because it supports literals, slices, and owned types, and removes the extra hop -> so why not use it everywhere.
Idiomatic Rust doesn't use &String at all. So in practice you have exactly two:
  String — the owner
  &str — the borrow
You'd think the borrow of string should be &String and that works, but this type of borrow is a DEFICIENT BORROW - 
because: double indirection, can't name literals or sub-slices. So therefore Rust provides a better borrow form &str
for you to use as THE way to borrow. &str doesn't sit on top of &String — it takes its place. You're swapping one borrow form for a better one.

  There are TWO types of strings:
	String  The "main" type of string - owned, growable, heap-allocated string. It owns its text and is responsible for freeing it.

	&str    A borrowed view into string text that lives somewhere else. It owns nothing, it's a "window" onto bytes owned by someone else
			(a String or a string literal baked into the binary). String is to &str as owner is to borrower. That's it — everything else follows from it.
	
			("string slice" - a fat pointer: address + length).
	
			Why string literals like "hello" are &str type?
			Compiled binary contains both: machine instructions AND hardcoded data such as string literals "hello".
			When you run the binary, the OS loads the binary into memory - ALL of it including the data (string literals).
			So while the program runs "hello" permanently sits in the process's memory. &str points at that string in memory.
			Who owns the string literal? Nobody, because through the lifetime of the process string literal is never allocated 
			and never freed. It's just permanently there in memory.

			Since it cannot be unloaded from memory or modified it can't be the normal String type.
			
			// &str type. String literals are ALREADY in memory (loaded from binary at startup), and stay there until the program exits. 
			// Nothing allocates them, nothing frees them → doesn't need an owner, because there's no freeing decision to make.

			Ownership in Rust exists to answer one question: "when does this memory get freed?" A String needs an owner
			because its heap buffer must be freed at some point, and Rust tracks who's responsible. A string literal
			sidesteps the whole question — it lives as long as the program itself, so there's simply no freeing event to
			assign to anyone.

			So the resolution to your puzzle: a &str is "a view into data someone else owns" — and for a literal, the
			"someone else" is the program's static memory itself (the loaded binary), which holds that data for the
			entire run. 

			Rust has a name for "lives for the entire duration of the program": the 'static lifetime. That's why the
  			true type of a string literal is:
				let s: &'static str = "Hello";
				//        ^^^^^^^ "this borrow is valid for the whole program run"

	- string literals are &str (baked into the binary)
	- &String auto-converts to &str, so FUNCTIONS TAKE &str: fn f(s: &str) accepts both
- Strings are UTF-8 bytes:
	- .len() = BYTES, not characters ("é" is 2 bytes)
	- s[0] doesn't compile! use .chars().nth(i) for chars, &s[a..b] for byte-range slices
- methods never mutate in place unless they take &mut self (push_str) - to_uppercase etc. return new Strings
*/

#![allow(unused)]
pub fn main() {
	// ------------------------------
	// String vs &str
	let s1 = String::from("hi"); 	// String type. heap buffer is ALLOCATED when this runs, and FREED when `owned` goes out of scope.
									// → needs an owner, because someone must decide when to free it.

	let s2 = "hi"; 		// String literals get loaded from binary at startup and permanently live in memory of the program as long as the program lives. 
						// They cannot be freed from memory or mutated and so they cannot have an owner variable.
						// So &str is just a borrowed view into that string in memory.

	let s3: &str = &s1;	// &str

	// Unification factor - &str accepts all types of string - literal, slice, owned. This flexibility is why the idiom is "take &str".
	// ⭐ There aren't really "two string types" — there's one string (the bytes) and two ways to hold it: owning it (String) or 
	// borrowing it (&str) (string just happens to have a more powerful way of borrowing it than other types).
	// ⭐ The idiom: "take &str, give String"
	//			Use for										Cost to create from the other
	//  String  fields, returned data, building strings		allocates + copies
	//  &str    function params, read-only views			free (just a view)

	//   - Function parameters → take &str. It's the most flexible: a caller can pass a &str or a String (the latter
	//   auto-borrows to &str). Taking String needlessly forces callers to give up ownership.
	//   - Struct fields / return values / when you need to own or modify → use String. If the data must outlive the
	//   current scope, or you need to build/grow it, you need ownership.
	// 
  	struct User { name: String }        // struct owns its data → String
  	fn make_name() -> String { return String::from("hi"); }    // returning owned data → String
	fn make_literal() -> &'static str {return "hi";}	// This is fine, because "hi" is baked into the binary, so the &str reference doesn't outlive the data

	let owned_string = String::from("hello");
	fn print(s: &str) { println!("{s}"); }
	print("literal");             // bytes in the binary
	print(&owned_string);         // bytes on the heap (String coerces to &str)
	print(&owned_string[2..5]);   // a substring — bytes mid-buffer

	// Deref coercion (automatic conversion to &str)
	// Note the asymmetry: &str → String costs an allocation (you're making an owned copy), while String → &str is free (you're just handing out a window).
	let owned: String  	= String::from("foo");
	let borrowed: &str	= "foo";

	// String -> &str 		(three ways)
	let borrowed1: &str  = &owned;			// &str annotation
	let borrowed2        = owned.as_str();	// .as_str()
	let borrowed3        = &owned[..];		// slice

	// &str -> String
	let owned1 = String::from(borrowed);	// String::from()
	let owned2 = borrowed.to_string();		// to_string()
	let owned3 = borrowed.to_owned();		// to_owned()

	// ------------------------------
	// Create 
	let s = String::from("hello");
	let s = "hello".to_string();        // same thing
	let s = String::new();              // empty
	let lit: &str = "hello";            // literal - a &str, not a String

	// ------------------------------
	// Grow / mutate (needs mut) 
	let mut s = String::from("hello");
	s.push_str(" world");               // append &str
	s.push('!');                        // append single char

	// ------------------------------
	// Concatenate 
	let hello = String::from("hello");
	let world = String::from("world");
	let hw = hello + " " + &world;      // + moves the LEFT operand (hello is dead now), borrows the rest
	let hw = format!("{} {}", "hello", world);      // format! moves nothing - prefer it
	let n = 3;
	let s = format!("{n} pigs");        // inline vars like f-strings
	let s = format!("{:.2}", 3.14159);  // -> "3.14" (format specs live after :)
	let s = format!("{:?}", vec![1, 2]);// {:?} = debug print for non-string things

	// ------------------------------
	// Length / emptiness 
	let n = "héllo".len();              // -> 6 BYTES (é is 2), not 5!
	let n = "héllo".chars().count();    // -> 5 characters
	let e = "".is_empty();

	// ------------------------------
	// Index / slice 
	let s = "hello world";
	// s[0]                             // doesn't compile - which byte of a multi-byte char would it mean?
	let c = s.chars().nth(0).unwrap();  // first char (Option - .unwrap() = "or crash", ch 6)
	let sub = &s[0..5];                 // byte-range slice -> "hello" (&str view, no copy)
	// &"héllo"[0..2]                   // PANICS at runtime - cuts é in half. slice on char boundaries only.

	// ------------------------------
	// Case 
	let s = "Hello World!".to_lowercase();   // returns new String, original untouched
	let s = "Hello World!".to_uppercase();

	// ------------------------------
	// Search 
	let s = "she and he and they";
	let e = s.contains("and");
	let e = s.starts_with("she");       // also .ends_with()
	let i = s.find("and");              // -> Option<usize>, BYTE index of first match (None if absent - no -1s here)
	let n = s.matches("and").count();   // count occurrences

	// ------------------------------
	// Replace (returns new String) 
	let s = "she and he".replace("and", "+");
	let s = "a b c".replace(' ', "");   // remove chars

	// ------------------------------
	// Trim 
	let s = "  padded  ".trim();                    // also .trim_start() / .trim_end()
	let s = "#!comment!#".trim_matches(['#', '!']); // trim specific chars

	// ------------------------------
	// Split / join 
	let parts: Vec<&str> = "a, b, c".split(", ").collect();     // -> ["a", "b", "c"]
	let joined = ["History", "Math"].join("__");                // -> "History__Math"
	let words: Vec<&str> = "  a  b ".split_whitespace().collect(); // splits + ignores extra spaces
	let lines: Vec<&str> = "l1\nl2".lines().collect();

	// ------------------------------
	// Iterate 
	for c in "hello".chars() {}         // by character
	for b in "hello".bytes() {}         // by raw byte (u8)

	// ------------------------------
	// Compare 
	let e = String::from("a") == String::from("a");     // -> true, == compares contents

	// _____________________________________________________________________________________________
	// Why s[0] doesn't compile
	// Because strings are utf-8 vectors (meaning each character can oppupy between 1 and 4 bytes) and so there's no O(1) operation to get Nth caracter, because in order to get an Nth character you need to walk the entire string which is O(n) operation.
	// The s[0] syntax would look like a O(1) operation, and so Rust refuses to have it as that would be misleading, and instead makes you explicitly choose what you want - byte O(1) but meaningless (because it can return half a character) or characters O(N).
	
	// ⚠️  .len() returns bytes, not characters — the single most common surprise. And .chars().count() has to walk the whole string to answer.
	let s = "héllo";   	// h, é, l, l, o  — 5 "characters", but é is 2 bytes
	s.len();            // 6   ← number of BYTES (not characters!)
	s.chars().count();  // 5   ← number of Unicode scalar values (`char`s). 	O(N) operation

	// Rust exposes a truth (variable-width encoding) that other languages hide, and makes you name which thing you actually mean.
	s.bytes();			// Iterator over bytes (one char can be up to 4 bytes)
	s.chars();			// Iterator over chars (one char is always 4 bytes - after conversion)
	s.chars().nth(2);   // Option<char> — note it's an iterator walk, O(n)
						// option is there because the string might not have the Nth character

	// Slicing works — but by BYTES, and it can panic
	// You can take substrings with a range, but the indices are byte offsets, not character offsets
	let s = "héllo";
	let ok  = &s[0..1]; 	// "h"  — byte 0..1 lands on a clean char boundary ✅
	let bad = &s[0..2]; 	// 💥 PANIC at runtime: byte index 2 is not a char boundary
							//    (byte 1 is the FIRST half of é; slicing there splits it)
	// Rust's rule: a str must always be valid UTF-8, so it refuses to hand you a slice that would cut a
	// character in half. Slicing at a bad boundary panics rather than producing a corrupt string.

	// A Rust string is UTF-8 bytes, not an array of characters — so s[0] is forbidden (a byte would corrupt, a 
	// char would be secretly O(n)), .len() counts bytes, "characters" come from .chars(), and slicing is by byte
	// offset and panics if it splits a character.

}

