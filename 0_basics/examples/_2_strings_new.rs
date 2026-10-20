#![allow(unused)]

fn main() {

    // Deref coercion
    // &String -> &str coercion happens automatically. It only goes from reference (&String, not String) to a reference (&str), and only in that direction.
    let s_owned = "hi".to_string();
    let s: &str = &s_owned;

    // _____________________________________________________________________________________________
    let mut s = "hello".to_string();


	// Create 
	"hello";                    // literal, in static memory (&str, not String)
	String::new();              // empty
	String::from("hello");      // from literal
	"hello".to_string();        // from literal

	// Length / Is empty
	"héllo".chars().count();    // Length in characters. ⚠️ Note: .len() gives length in bytes which may be longer than char length if the UTF-8 is not ASCII
	"".is_empty();

	// Concatenate
	s.push_str(" world");       // append &str or String
	s.push_str("hi".to_string());
	s.push('!');                // append single char

	// Uppercase / lowercase    (returns new String, original untouched)
	"Hello World!".to_uppercase();
	"Hello World!".to_lowercase();   
 




	
}













