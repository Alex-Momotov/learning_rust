#![allow(unused)]

use std::{collections::{HashMap, HashSet}, default, fmt::Display, fs::File, hash::Hash, ops::{Deref, DerefMut}, path::Path};
use std::alloc::{alloc, dealloc, Layout};

fn main() {
    /* _____________________________________________________________________________________________
    MACROS   (BRIEFLY)
    Attributes are implemented using macros, and you can write your own. 
    A macro is code that writes code. It runs at compile time, takes some of your source code as input and produces more source code, which is then compiled normally.
    Two ways to use macros: attributes (like #[derive(Debug)]) and macro calls like println!, vec!, format! - The ! marks a macro call.

    Two kinds of macros:
        - Declarative macros - They match patterns (foo!PATTERN) and expand that into code. Example: vec![1, 2, 3].
                               vec![]    -> expands into ->   Vec::new()
        - Procedural macros - These are real Rust functions that receive your code and return new code. Must be defined in its own crate. Example: #[derive(Serialize)]
       
    _____________________________________________________________________________________________ 
    ATTRIBUTES
    An attribute is written as #[...] above a struct, enum, function, field, or module.
    Its a label telling compiler or other tools to 'do something' at compile time - generate code, remove code, silence warning. 
    Its a similar mechanism to Python's decorators and Java's annotations, with the only difference that Rust's attributes act entirely at compile time. 


    WHY ATTRIBUTES EXIST
    Because we need a way to say things about the code that aren't part of the code itself.
      - "Generate the boring Clone impl for me."
      - "Don't warn me about unused variables in this file."
      - "Lay this struct out in memory exactly like C does."
      - etc ...
    The language could have added a keyword for each of these, however it would make the language complex, so instead it has one uniform syntax for all of these.
    In addition, libraries can add their own attributes without changing the language. 


    THE SYNTAX
    Syntax               Example                   Description
    #[name]              #[test]                   Plain
    #[name(arg1, arg2)]  #[derive(Debug, Clone)]   With args
    #[name = value]      #[doc = "hi"]             With key=value args
    #![name]             #![allow(unused)]         Inner form: applies to the thing it's inside. Plain #[...] applies to the item after it. #![...] applies to the item around it.    

    _____________________________________________________________________________________________ 
    THREE FAMILIES OF ATTRIBUTES
    1. Built-in instructions to compiler. Most common:
        #[derive(...)]            generate trait impls (below)
        
        #[allow(unused)]          allow unused variables
        #[must_use]               warn if a caller ignores the return value
        #[deprecated]             warn anyone who uses this
        
        #[cfg(test)]              only compile this when running tests
        #[test]                   this function is a test

        Doc comments are attributes 
        /// adds two numbers        ->  desugars to  ->      #[doc = " Adds two numbers"]

    2. Derive. #[derive(Clone)] means "write the obvious impl Clone for me". The compiler generates a real impl block, exactly as if you'd typed it.
       Derive is a code generator for trait impls - and it's typically "do this thing to every field".

        #[derive(Clone)]
        struct Point { x: i32, y: i32 }
        
        ↓ the compiler generates roughly this, invisibly:
        
        impl Clone for Point {
            fn clone(&self) -> Self {
                Point { x: self.x.clone(), y: self.y.clone() }
            }
        }

        When to derive vs write impl by hand
        When "do the same thing to every field" is the right behaviour (compare every field, clone every field, print every field) -> derive it.
        When it isn't -> write the impl yourself.

        Derive is recursive
        Every field's type must itself implement the trait you're deriving. e.g. #[derive(Clone)] on a struct only works if all its fields are Clone.
        If one of the fields isn't Clone then you'd get the compiler error.
        

    3. Attributes from libraries. 
       Crates can define their own attributes such as Clap, or Tokio.

        #[derive(Parser, Debug)]    <- Clap cli
        struct FlatCli { ... }

    * Bonus: Doc comments are attributes too. 
      '/// Adds two number'     -> desugars to ->   #[doc = " Adds two numbers"]

    _____________________________________________________________________________________________ 
    When to use attributes
    - get a standard behaviour for free: #[derive(...)], which is by far the most common
    - control warnings: #[allow(...)], #[deny(...)]
    - include or exclude code: #[cfg(...)], #[test]
    - plug into a library: #[tokio::main], #[derive(Serialize)]     */

    
    // _____________________________________________________________________________________________
    // DERIVE
    // - #[derive(...)] is an attribute that tells the compiler to auto-generate a trait impl block for your type.
    // - When to write impl by hand - whenever the auto-generated logic isn't what you want.

    //     PartialEq / Eq   -> ==
    //     PartialOrd / Ord
    //     Clone            -> enables .clone()
    //     Copy             -> makes '=' do bitwise copy of your type instead of moving. Only works if every field is Copy
    //     Default          -> Dot::default(), all fields zero/empty
    //     Hash             -> usable as a HashMap key

    // --------------------------------------------------------------------------------------------------------------------------
    // Trait             Derivable    What does it do                                  When to hand-write?
    // --------------------------------------------------------------------------------------------------------------------------
    // Debug            │ ✅        │ developer printout via {:?} / {:#?}             │ you want a custom developerview (rare)
    // Display          │ ❌        │ user-facing printout via {}, gives .to_string() │ always, if you want one

    // PartialEq / Eq   │ ✅        │ == and != (Eq: also promises a == a)            │ equality isn't field-by-field
    // PartialOrd / Ord │ ✅        │ < > <= >= and sorting (Ord: total order)        │ ordering isn't declaration-order lexicographic
    // Hash             │ ✅        │ turns a value into a hash, needed for map keys  │ you hand-wrote PartialEq

    // Default          │ ✅        │ T::default() gives a starting value             │ the default isn't zero/empty
    // From / TryFrom   │ ❌        │ converts another type into this one (or fails)  │ always
    // Clone            │ ✅        │ explicit, possibly expensive copy via .clone()  │ cloning needs to do somethingspecial
    // Copy             │ ✅        │ = copies the bits instead of moving             │ never — it's a marker, derive or don't

    // Drop             │ ❌        │ code that runs when the value goes away         │ you own a non-Rust resource
    // Deref            │ ❌        │ *x and auto-forwarding of method calls to inner │ you're writing a smart pointer
    
    // _____________________________________________________________________________________________
    // DEBUG, DISPLAY
    // Derive Debug on almost everything, implement Display only for types a user will see, such as error messages, CLI output or logs. 

    // DEBUG
    // Derivable. Makes it printable with {:?} or {:#?} (pretty). Intended audience is programmers.
    #[derive(Debug)]
    struct Cat { name: String, age: u32 }

    let cat = Cat { name: "Kitty".to_string(), age: 5 };
    println!("{:?}", cat);

    // DISPLAY
    // Can't derive. Makes it printable with {}. Also gives you to_string() for free. Intended audience is users.
    impl Display for Cat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} ::: {}", self.name, self.age)
        }
    }
    
    println!("{}", cat);
    cat.to_string();

    // _____________________________________________________________________________________________
    // PARTIALEQ, EQ
    // Derive both unless the type contains floats, or a == a is not always true.

    // PARTIALEQ
    // Derivable. Gives a type .eq(), == and !=. Compares field by field.
    #[derive(PartialEq)]            // floats inside: PartialEq only
    struct Point { x: i32, y: i32 }

    let p1 = Point {x: 10, y: 15 };
    let p2 = Point {x: 10, y: 15 };
    p1.eq(&p2);

    println!("{:?}", p1 == p2);
    println!("{:?}", p1 != p2);
    
    // EQ
    // Derivable. A marker trait (has no methods) - a promise that every value equals itself (a == a is always true).
    // In floats NaN (not a number) is not equal to anything, including itself (F64::NAN != F64::NAN), which is why floats don't implement Eq.
    // That also means any type that contains floats can't derive Eq.
    #[derive(PartialEq, Eq)]        // the usual pair
    struct UserId(u32);

    // _____________________________________________________________________________________________
    // HASH
    // Derivable. A derived hash feeds every field into the hasher. A type needs both Hash and Eq to be used as a map key or set member.

    // The contract: f a == b, then hash(a) == hash(b). A lookup uses hash key to find the bucket, then uses == inside that bucket. 
    // If two equal values hashed differently they would go to different buckets and the lookup would miss. 
    // ! The rule: Derive both PartialEq and Hash, OR handwrite both. But never derive one and hand write the other, because their logic must be the same.
    // Deriving Hash next to a custom PartialEq is the classic silent bug.

    // Floats don't implement Hash, because they have NaN, which is why f64 can't be a map key.
    
    #[derive(PartialEq, Eq, Hash)]
    struct User(String, i32);

    let u = User("John".to_string(), 25);
    let mut m = HashSet::new();
    
    m.insert(u);    // can be inserted into hashset or hashmap
    
    // _____________________________________________________________________________________________
    // PARTIALORD, ORD
    // Derive both unless the type contains floats, or "any two values can always be compared" is not true.

    
    // PARTIALORD
    // Derivable, requires PartialEq. Gives a type partial_cmp(), <, >, <=, >=. 
    // Derived ordering - fields are compared one by one in declaration order and we look at the next field only on a tie (==). For enums its variant declaration order, then by field.
    
    // By itself partial_cmp() only returns Option<Ordering>, implying that not all instances of the type can be compared. 
    // E.g. float's NaN can't be compared. This also means methods such as Vec::sort() on the collection of items are unavailable until ORD is implemented also.
    #[derive(PartialEq, PartialOrd, Eq, Ord)]
    struct SemVer { major: i32, minor: i32, patch: i32 }

    let s1 = SemVer { major: 1, minor: 1, patch: 1 };
    let s2 = SemVer { major: 1, minor: 2, patch: 1 };

    println!("{:?}", s1 > s2);
    println!("{:?}", s1 <= s2);
    s1.partial_cmp(&s2);


    // ORD
    // Derivable. Requires Eq, PartialOrd. Marker trait - a promise that all value of the type can be compared. 
    // Gives a type cmp() (returns Ordering, not Option<Ordering>) which enables methods such as Vec::sort(), max(), min().
    
    #[derive(PartialEq, PartialOrd, Eq, Ord)]
    struct Version { major: i32, minor: i32, patch: i32 }

    let v1 = Version { major: 1, minor: 1, patch: 1 };
    let v2 = Version { major: 1, minor: 2, patch: 1 };
    
    let mut vec: Vec<Version> = vec![v1, v2];
    vec.sort();

    /* _____________________________________________________________________________________________
    CLONE, COPY
    If the type implements nothing (no Clone, nor Copy) - it gets moved on assignment (same as Clone types) and doesn't get .clone() - sometimes it makes sense to not implement Clone if a type represents a unique resource such as a file.
    A type can implement either: Clone trait, or Clone + Copy traits. This affects what happens when you: 1. Assign the type with '='. 2. Clone the type with .clone().
                Clone (can have heap data*)                                                 Clone + Copy (stack data only*)
    =           variable moved; stack copied, heap untouched (cheap);                       variable alive; stack copied (cheap)
    .clone()    variable alive; stack copied, heap copied (expensive if has heap data)      variable alive; stack copied (cheap), same as '='                       

    When to use each:
    - Derive Clone on most data types.
    - Add Copy for simple, small plain types consisting of all Copy values. Be careful - if you add Copy and then remove it, all usages that rely on assignment doing copy it break.    */
    

    // CLONE
    // Clone - means you can clone it with .clone() - stack / Copy fields are copied on the stack (cheap), and heap / dynamic fields (like String) are copied on the heap (expensive heap allocation).
    // So, unless the type is also Copy or you know that all fields are Copy, assume that .clone() is an expensive operation.
    // Assignment (=)   variable moved; stack copied, heap untouched (cheap); 
    // .clone()         variable alive; stack copied, heap copied (expensive if has heap data) 
    #[derive(Clone)]
    struct Expensive { 
        x: i32,         // Copy field - will be copied during .clone()
        name: String,   // Clone field - meaning it can potentially hold heap data - potentially expensive allocation during .clone()
    }
    let e = Expensive { x: 10, name: "hi".to_string() };
    let e2 = e;                 // moved
    let e_clone = e2.clone();   // expensive .clone()

    
    // COPY
    // Copy - marker trait, a promise that this value in its entirety can live on the stack (because each field is Copy / can live on the stack), 
    // meaning that .clone() is cheap (stack copy only), and also makes assignment (=) copy, not move.
    #[derive(Clone, Copy)]
    struct Cheap { 
        x: i32,     // Both fields are Copy - therefore this type can be Copy (a promise that all fields are), therefore .clone() is cheap, and assignment (=) copies, not moves
        y: i32 
    }
    let c = Cheap { x: 10, y: 10 };
    let c2 = c;                 // copied (c still alive)
    let c_clone = c2.clone();   // cloned (cheap - copied on stack)

    // _____________________________________________________________________________________________
    // DEFAULT
    // Default gives a type one function Type::Default(), with no parameters, that creates a value with every field set to it's default value: 0, false, "".
    // For an enum, you mark the default variant with #[default] attribute - that variant must be unit variant (no fields).
    #[derive(Debug, Default)]
    struct Config { 
        host: String, 
        port: u16, 
        retries: u8 
    }
    let config = Config::default();

    
    #[derive(Default)]
    enum Severity {
        #[default] 
        INFO,
        WARN,
        CRITICAL
    }
    let sev = Severity::default();      // INFO


    // Hand write default if you want other default values.
    struct Something {
        x: i32,
        y: i32
    }
    impl Default for Something {
        fn default() -> Self {
            Self { x: 30, y: 30 }
        }
    }
    let s = Something::default();

    // _____________________________________________________________________________________________
    // FROM / INTO
    // From - implemented by hand to add associated function Type::from(another_type) that converts another type to this one.
    // Implementing From automatically adds into() for another_type in reverse direction. You never implement Into yourself. So the rule is: implement From, use either.
    struct Feet(i32);
    struct Meters(i32);

    impl From<Feet> for Meters {        // We only implement From, but we get both Type::from(another_type) and another_type.into()
        fn from(value: Feet) -> Self {
            Meters(value.0 * 2)
        }
    }
    
    let meters1 = Meters::from(Feet(10));
    let meters2: Meters = Feet(10).into();

    
    // Practical example 1 - a function that accepts impl Into param, as a way of accepting anything that can be converted
    fn greet(name: impl Into<String>) {
        let msg = name.into();
        println!("{}", msg);
    }

    
    // Practical example 2 - custom application error
    // ? uses From to convert one error into another. So one From impl lets ? convert that error type into yours everywhere.
    // impl From<X> for MyError is therefore the highest-leverage impl in a Rust codebase: one 3-line block removes boilerplate from every call site that can produce X.
    enum AppError {
        ConfigError(std::io::Error),
        ProcessingError
    }
    impl From<std::io::Error> for AppError {
        fn from(value: std::io::Error) -> Self {
            AppError::ConfigError(value)
        }
    }

    fn read_file() -> Result<File, AppError> {
        let file = File::open("data.txt")?;     // ? uses From to convert one error into another
        Ok(file)
    }

    // _____________________________________________________________________________________________
    // TRY_FROM / TRY_INTO
    // Same idea as From / Into, but for conversions that can fail. So the return type of Type::from(another_type) is Result<Self, error_type> where error_type is an associated type.
    struct Pounds(i32);
    struct Kg(i32);

    impl TryFrom<Pounds> for Kg {
        type Error = String;
        fn try_from(value: Pounds) -> Result<Self, Self::Error> {
            if value.0 > 0 { 
                Ok(Kg(value.0))
            } else { 
                Err(format!("{} is less than zero", value.0)) 
            }
        }
    }

    let kg = Kg::try_from(Pounds(10)).unwrap();
    let kg: Kg = Pounds(10).try_into().unwrap();

    // Practical example 1 - the standard library's number conversions follow the same From / TryFrom split. Widening, which always fits is From. Narrowing, which might fail, is TryFrom.
    let a: u32 = u32::from(5u8);                // Widening (will always succeed) -> from
    let b: u8 = u8::try_from(3u32).unwrap();  // Narrowing (can fail) -> try_from

    // _____________________________________________________________________________________________
    // DROP
    // Drop is code that runs automatically when a value goes away - either because it goes out of scope or because its owner is dropped with drop()
    // - Order: 
    //    - variables are dropped in reverse order of declaration: last in, first out (stack).
    //    - A struct's own drop body runs first, then its fields are dropped in declaration order.
    // - A type can't be both Drop and Copy. If a value can be silently duplicated, there's no single owner whose cleanup should run exactly once.
    // You rarely implement drop yourself - only for something that Rust doesn't know how to clean up: e.g. a handle from C, or a hand written buffer with unsafe code.

    struct Hi { x: i32 }
    impl Drop for Hi {
        fn drop(&mut self) { println!("Hi {} was dropped!", self.x); }
    }
    
    let h = Hi { x: 10 };
    drop(h);                    // prints "Hi 10 was dropped!"

    
    // A common case for implementing Drop is when you write custom unsafe type e.g. a memory buffer.
    // If you allocate raw memory yourself, Rust doesn't know that memory exists, so you need to free it yourself with Drop. 
    // This is how String, Vec are built internally - they allocate raw memory and free it in their Drop.
    struct Buffer { 
        ptr: *mut u8, 
        layout: Layout 
    }
  
    impl Buffer {
        fn new(size: usize) -> Self {
            let layout = Layout::array::<u8>(size).unwrap();
            let ptr = unsafe { alloc(layout) };   // raw allocation: Rust won't track it
            Buffer { ptr, layout }
        }
    }
  
    impl Drop for Buffer {
        fn drop(&mut self) {
            unsafe { dealloc(self.ptr, self.layout) };   // without this, the memory leaks
        }
    }
    
    // _____________________________________________________________________________________________
    // DEREF

    // Deref lets a type behave like a reference to something inside it - implying that the type is either a reference to or a wrapper around something more meaningful. 
    // Dereferencing - means "follow the arrow". What you're holding points at something, and * (or dereferencing) takes you to the thing. Implementing Deref is your type saying "I am an arrow to a Target"

    // Implementing Deref adds .deref() method that returns &Target; and implementing DerefMut adds .deref_mut() method that returns &mut Target.
    // Manually (or otherwise) defererencing then picks .deref() or .deref_mut() depending on whether the operation reads or only mutates.

    // 1. *x works on the type
    // 2. Method calls are forwarded - if a method isn't found on your type, the compiler retries the lookup on the type it derefs to. 
    //    This is also how auto-dereferencing works on pointers (&, &mut, Box, etc). Those pointers implement Deref.
    // 3. Deref coercion - wherever &Target or &mut Target is expected you can pass &Type or &mut Type and it will convert automatically (compiler inserts deref for us).

    struct MyType {
        x: i32,
        s: String
    }
    impl Deref for MyType {     // Deref enables: 1. manual dereferencing * for read operations; 2. Methods via auto-dereferencing that require read only access; 3. Deref coercion &Type -> &Target
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.s
        }
    }
    impl DerefMut for MyType {  // DerefMut enables: 1. manual dereferencing * for write operations; 2. Methods via auto-dereferencing that require write access; 3. Deref coercion &mut Type -> &mut Target
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.s
        }
    }

    let mut my_type = MyType { x: 10, s: "hi".to_string() };
    let read_ref = my_type.deref();         // Same as doing *my_type just for reading - e.g. let foo = *my_type
    let mut_ref = my_type.deref_mut();  // Same as doing *my_type for mutable access - e.g. *my_type += 1
    
    // 1. Dereferencing manually gives you the inner target. What can you do with it?
    let m2 = (*my_type).clone();  // Moving / copying out (if Target is Copy - you can copy it out; if Target is Move - you can't move it out from behind a reference, but you can clone it)
    *my_type = "new".to_string();         // Modifying / re-assigning to something else
    *my_type += "hi";                     

    // 2. Calling inner methods (auto-dereferencing)
    my_type.len();            // MyType has no len(), so len() is found and called after auto-dereferencing to Target
    my_type.push_str("hi");   // Mutable operation - requires DerefMut implemented

    // 3. Deref coercion
    fn something(s: &String) {}
    fn something2(s: &mut String) {}
    something(&my_type);          // compiler inserts &*m automatically
    something2(&mut my_type);     // compiler inserts &mut *m automatically
    

    // _____________________________________________________________________________________________


}


