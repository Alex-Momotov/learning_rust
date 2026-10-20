#![allow(unused)] // supress unused variable warnings - for learning notes
use std::{cell, num::ParseIntError};

// comment

/*
Multi-line 
comment
*/


const MY_CONSTANT: u32 = 60 * 3;    // Constant - must be: uppercase; declare type; equal to a literal or expression known at compile time (not runtime); 
                                    // can be: in global scope (outside functions) or inside a function

fn main() {
    // _____________________________________________________________________________________________
    // Printing
    let a = 5;
    println!("{a}");    // Variables
    
    let b = 5;
    println!("{}", b + 1);  // Expressions - must use {} and pass the expression as an argument to println

    let a = [1, 2, 3];
    println!("{a:?}");    // Collections - must use {:?}
    println!("{:?}", a);
    
    // _____________________________________________________________________________________________
    // Variables - cannot be in global scope (outside functions)
    let a = 5;              // Immutable variable
    
    let mut b = 6;          // Mutable variable
    b = b + 1;
    
    const C: i32 = 100;     // Constant
    
    // _____________________________________________________________________________________________
    // Shadowing
    let x = 5;
    let x = x + 1;  // new variable 6

    // Inner shadowing
    {
        let x = x * 2;  // new variable 12
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");     // return back to original scope - value back to 6

    // Shadowing - can change type
    let spaces = "   ";             // string
    let spaces = spaces.len();     // number

    // _____________________________________________________________________________________________
    // Types
    // Scalar types - integers, floats, booleans, characters
    // Compound types - tuples and arrays
    // Inferrence - Rust is statically typed, meaning it knows types at compile time. But it can also infer types by literal values or by how we use them.
    
    // ------------------------------
    // Integers 
    // i - signed, u - unsigned, isize and usize - depend on the architecture of the computer the program is running on: 64 = i64 or 32 = i32.
    // Default is i32 if unspecified
    
    let a: i8 = 127;
    let b: i16 = 32_767;
    let c: i32 = 2_147_483_647;                                         // ~2.1 billion
    let d: i64 = 9_223_372_036_854_775_807;                             // ~9.2 quintillion
    let e: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    let f: isize = 9_223_372_036_854_775_807;                           // = i64 max on a 64-bit machine (pointer-sized)
    
    let a: u8 = 255;
    let b: u16 = 65_535;
    let c: u32 = 4_294_967_295;                                         // ~4.3 billion
    let d: u64 = 18_446_744_073_709_551_615;
    let e: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let f: usize = 18_446_744_073_709_551_615;                          // = u64 max on a 64-bit machine (pointer-sized)

    // You can also specify type with a suffix - works on any numeric literal
    let g = 3u8;
    let h = 2.5f32;
    
    // Min and max values
    let b = i8::MAX;    // Max value
    let b = i8::MIN;    // Min value

    // Overflow story
    // In a debug build rust panics on overflow.
    // In a release build rust wraps around on overflow.

    // ------------------------------
    // Floats 
    // Default - f64
    let a: f32 = 3.0;
    let b: f64 = 3.0;

    // ------------------------------
    // Booleans 
    let a: bool = true;
    let b: bool = false;

    // ------------------------------
    // Char
    let a: char = 'a';  // must use single quotes

    // ⭐ The ideomatic i32 and usize
    // i32 - The idiomatic default for "just a number" - quantities, arithmetic, domain values (ages, counts you compute with)
    // usize - platform agnostic memory addresses - anything that indices or measures a buffer.
    //         It's 32 bits on a 32-bit system and 64 bits on a 64-bit system, so you can use a single type to for memory address
    //         arithmetic without wasting bits and still being able to address the entire memory space.
    //         It's pointer-sized: big enough to address any position in memory on the current platform.
    //         It's the type of lengths, indices, and capacities - v.len() returns usize, v[i] requires i to be usize, and others. 


    // _____________________________________________________________________________________________
    // OPERATORS
    
    // Arithmetical
    5 + 10;     // addition
    5 - 10;     // subtraction
    5 * 30;     // multiplication
    5 / 3;      // integer division = results in 1
    5.1 / 3.1;  // float division
    10 % 5;     // remainder
    5i32.pow(2);    // raise integer to integer power
    5f32.powi(2);   // raise float to integer power (efficient)
    5f32.powf(2.0); // raise float to float power (less efficient)

    // Compound assignment
    let mut n = 10;
    n += 1; 
    n -= 1; 
    n *= 2; 
    n /= 2; 

    // Comparison
    // Returns bool. NOTE: both sides must be the SAME type. `1 == 1.0` won't compile
    1 == 2;  
    1 != 2;
    1 < 2;   
    1 <= 2;  
    1 > 2;  
    1 >= 2;

    // Logical
    true && false;   // AND, short-circuits
    true || false;   // OR,  short-circuits
    !true;           // NOT

    // _____________________________________________________________________________________________
    // CASTING
    
    // ::from / .into    For widening (SAFE) - from a smaller type to a larger type (e.g. i32 -> i64)
    //                   Those methods simply don't exist for narrowing (unsafe) conversions. 
    //                   Use whenever widening, because it's safe. If it compiles it's safe.
    let a = i64::from(5i32);
    
    // as           UNSAFE - Never fails at compile or runtime, but can silently lose data on narrowing (wrap or truncate)
    //              Use only when lossyness is acceptable, or when you know the value is in range.
    5i32 as u8;     
    5.6f32 as i32;  // = 5   discards fractional part (rounding down to zero)

    // try_from / try_into  SAFE for handling potentially narrowing conversions. Returns Result, forces you to handle it.
    //                      Use when lossyness is unacceptable, or when you don't know if the value is in range.
    u8::try_from(1000i32);   // returns Result - Err or Ok

    // When to use which: 
    //   from/into          Whenever widening, because it's safe. If it compiles it's safe.
    //   as                 When lossyness is acceptable, or when you know the value is in range
    //   try_from/try_into  Use when lossyness is unacceptable, or when you don't know if the value is in range.

    // Strings need PARSING, not casting — a string isn't a reinterpretable number:
    let n: i32 = "42".parse().unwrap();      // parse infers target from the annotation
    let m = "42".parse::<i32>().unwrap();    // ...or pin it with turbofish ::<>
    let s = 42.to_string();                  // number -> String, the reverse
    
    // _____________________________________________________________________________________________
    // INSTANCEOF Story
    // There isn't an instanceof equivalent in Rust.
    // The key shift is: instead of checking types at runtime, you encode the POSSIBILITIES into 
    // the type up front, and the compiler + match force you to handle each one. 

    enum Shape { Circle(f64), Square(f64) }
    let shape = Shape::Circle(1.0);

    match shape {
        Shape::Circle(r) => {},   // this is your "instanceof Circle"
        Shape::Square(s) => {},
    }

    // _____________________________________________________________________________________________

}

