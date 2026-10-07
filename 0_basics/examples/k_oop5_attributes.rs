#![allow(unused)]

use std::collections::HashMap;


fn main() {
    /* _____________________________________________________________________________________________
    ATTRIBUTES
    
    An attribute is written as #[...] above a struct, enum, function, field, or module.
    Its a label telling compiler or other tools to 'do something' at compile time - generate code, remove code, silence warning. 
    Its a similar mechanism to Python's decorators and Java's annotations, with the only difference that Rust's attributes act entirely at compile time. 


    Why attributes exist?
    Because we need a way to say things about the code that aren't part of the code itself.
      - "Generate the boring Clone impl for me."
      - "Don't warn me about unused variables in this file."
      - "Lay this struct out in memory exactly like C does."
      - etc ...
    The language could have added a keyword for each of these, however it would make the language complex, so instead it has one uniform syntax for all of these.
    In addition, libraries can add their own attributes without changing the language. 


    The syntax
    Syntax               Example                   Description
    #[name]              #[test]                   Plain
    #[name(arg1, arg2)]  #[derive(Debug, Clone)]   With args
    #[name = value]      #[doc = "hi"]             With key=value args
    #![name]             #![allow(unused)]         Inner form: applies to the thing it's inside. Plain #[...] applies to the item after it. #![...] applies to the item around it.    


    Three families of attributes:
    1. 
    
    _____________________________________________________________________________________________
    MACROS   (BRIEFLY)
    Attributes are implemented using macros, and you can write your own. 
    A macro is code that writes code. It runs at compile time, takes some of your source code as input and produces more source code, which is then compiled normally.
    Two ways to use macros: attributes (like #[derive(Debug)]) and macro calls like println!, vec!, format! - The ! marks a macro call.

    Two kinds of macros:
        - Declarative macros - They match patterns (foo!PATTERN) and expand that into code. Example: vec![1, 2, 3].
                               vec![]    -> expands into ->   Vec::new()
        - Procedural macros - These are real Rust functions that receive your code and return new code. Must be defined in its own crate. Example: #[derive(Serialize)]
    */





    
    // _____________________________________________________________________________________________
    // DERIVE
    // - #[derive(...)] is an attribute that tells the compiler to auto-generate a trait impl block for your type.
    //   e.g. #[derive(Debug)] auto-generates impl std::fmt::Debug for Point {...} 
    // - Derive is recursive, meaning every field's type must itself implement the trait you're deriving. e.g. #[derive(Clone)] on a struct only works if all its fields are Clone.
    // - When to write impl by hand - whenever the auto-generated logic isn't what you want.

    //     Debug      -> enables printing with {:?} and {:#?}.
    //     Clone      -> enables .clone()
    //     Copy       -> makes '=' do bitwise copy of your type instead of moving. Only works if every field is Copy
    //     PartialEq  -> ==
    //     Default    -> Dot::default(), all fields zero/empty
    //     Hash       -> usable as a HashMap key

    #[derive(Debug, Clone, Hash, Default, Eq, PartialEq)]
    struct Cattt { name: String, hungry: bool }

    let cat = Cattt { name: String::from("Witchy"), hungry: true };
    let mut hash_map: HashMap<Cattt, i32> = HashMap::new();

    println!("{:?}", cat);          // can print
    hash_map.insert(cat, 42);       // can insert into hash map
    let cat = Cattt::default();  // can create a default cat

    // _____________________________________________________________________________________________
    
}


