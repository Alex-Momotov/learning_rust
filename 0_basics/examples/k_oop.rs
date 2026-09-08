#![allow(unused)]

use std::{collections::HashMap, fs::File, io::Write};

/* 
General OOP notes

- Struct vs enum
  A struct's shape is known at compile time. An enum's shape is known at run time, and match is the only way to ask which one it is.
  The struct vs enum field access rules follow from this: instance.field works on struct because the compiler knows the field is always there.
  The same can't work on enum because "is there instance.field?" is a question with a runtime answer - so you must ask it in a form that handles every answer.

- Why struct fields and impl functions are private by default to outsiders?
  1. Encapsulation - private fields mean - "you're not supposed to construct this".
     You instantiate it from a builder, or a function return, and then from then on you use it's methods, but not the fields directly.
  2. Invariants - keeping a field private allows a struct to ensure it doesn't get modified in an illegal way that would break the struct's invariants.
     E.g. String's invariant is that the utf8 is legal chars, so you can't directly modify it's underlying Vec<u8>.

- The getter story
  Rust has no getter-for-everything culture. Instead: 
  Invariant fields are private, (rather than everything is private by default) + methods.

*/


fn main() {
    // _____________________________________________________________________________________________
    // STRUCT
    // Tuple structs - Think of it as a tuple with a name, that is a different type from another tuple with same params.
    // Unit structs - Sometimes you want a type to exist without any data in it - the type itself is the payload. It's only job is to be a distinct thing. 
    //                1. Group related impl functions under a struct so you can do TypeName::function1() TypeName::function2()
    //                2. Implement a trait but don't have any data to store -> empty struct with an impl block implementing a trait.
    //                3. Named stub as part of data modelling (nested structs and enums) - with intention to add fields later
    //                4. Simple error type - E.g. ParseError - for when there's nothing more to say about the error

    // Three kinds of struct
    struct Point { x: i32, y: i32 }     // named-field
    struct Colour(i32, i32, i32);       // tuple struct — fields accessed by number .0, .1
    struct State;                       // unit struct — no fields at all

    // Instantiating
    let p = Point { x: 1, y: 2 };
    let c = Colour(1, 2, 3);
    let s = State;

    let y = 1;
    let p = Point { x: 1, y};      // field init shorthand - 'y' is already in scope, so it can be passed instead of 'y: y'

    // Accessing fields
    p.x;    // named fields
    p.y;    
    c.1;    // tuple fields
    c.2;
    
    // Updating
    let mut p = Point { x: 1, y: 1 };
    let mut p = Point { x: 1, ..p};  // Recreate struct - .. means take the rest from another struct. Moves owned types from another struct
    p.x = 2;                                // Update single filed
    
    // Fields - Mutability
    // Either all of struct's fields are mutable or none are. There's no mut on individual fields. Just like with collections.
    let mut p = Point { x: 1, y: 1 };
    p.x += 1;

    // Fields - Borrowing
    // Unlike collections, you can borrow and move individual struct fields independently, and without locking the entire struct
    let mut u = Point {x: 1, y : 1};
    let w = &mut u.x;  // borrow d.x
    print!("{}", u.y);           // d.y is usable
    print!("{}", u.x);           // still using d.x
    
    // Fields - Moving out a single field
    // After moving out a single struct field, the struct becomes "partially moved". Consequences
    // 1. Operations that require the entire struct will fail - e.g. passing struct as parameter to a function
    // 2. Direct field access still works
    let u = User { name: "John".to_string(), email: "abc".to_string() };
    let name = u.name;  // move out name
    u.email;                    // ✅ email is still usable
    // u.name;                  // ❌ u.name is unusable (moved out)
    // u;                       // ❌ struct variable itself is unusable (partially moved)

    // Fields - Ownership
    // Default to owned fields. Borrow-holding structs = short-lived views (parsers, iterators).
    struct Owns   { s: String }             // the struct OWNS; the string drops when the struct drops
    struct Borrows2<'a> { a: &'a i32}       // the struct BORROWS; struct can't live longer than the reference; lifetime param is mandatory.

    
    // STRUCT PRIVACY 
    // Struct fields are private by default, even if struct itself is public.
    // You can't directly construct a struct with private fields if you don't have module level access. The only way is to call a function/impl block within the module that does.
    pub struct User2 {         // public struct
        pub id: i32,           // public field
        name: String,          // private field
    }
    impl User2 {
        pub fn new(name: String) -> Self { Self { name, id: 0 } }  // the only door in
        pub fn name(&self) -> &str { &self.name }                  // read-only view
    }

    let u = User2::new("alex".into());
    // let bad = user::User { name: ..., id: 0 };  // ❌ can't construct — no access to `name`
    // u.name;                                     // ❌ can't read (also can't match on it)
    u.name();                                      // ✅ through a method
    u.id;                                          // ✅ public field

    
    // _____________________________________________________________________________________________
    // ENUM
    // A construct that can only be exactly one variant - never more than one, and never zero.
    // Enum declaration lists variants. Each variant may carry data and may be different shape from other enums.

    // Three kinds of variant
    enum Status {
        Stopped,                        // unit variant - no data
        Finished(i32),                  // tuple variant - positional fields
        Running { t: i32, p: i32 },     // struct variant - named fields
    }

    // Instantiating
    let a = Status::Stopped;
    let b = Status::Finished(10);
    let c = Status::Running{ t: 10, p: 5 };

    // Accessing fields
    // You can't access fields like instance.0 or instance.a because variants have different shapes and compiler doesn't know which variant it is at runtime.
    // So, instead, pattern matching is the only way to access fields.
    match a {
        Stopped                   => {},
        Finished(n)          => {},
        Running {t, p } => {},
    }

    if let Finished(n) = a {
        println!("Taks is done!")
    }    

    // Updating fields
    // Need to match on &mut x, not x. Matching on x directly will only update COPY of the fields
    // Remember from the ownership notes: If the place/binding you're modifying is T and not &mut T AND the type is Copy, then you're modifying the copy and not the original
    let mut d = Direction::Up { i: 10 };
    match d {
        Direction::Up { mut i } => { i += 1 }       // ❌ Binding i is i32 (T and not &mut T) AND you're modifying a Copy type (i32)
    }
    match &mut d {
        Direction::Up { i } =>  { *i += 1 }    // ✅ Binding i is &mut i32 (&mut T)
    }

    // Use shortcut
    use Status::*;                  // Can now use variants without Status:: - less noisy.
    let a = Finished(10);   

    // DERIVE ON ENUMS
    // Derived PartialOrd/Ord follow declaration order. Earlier variants are less than later ones.
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    enum Severity { Info, Warning, Error, Critical }
    Severity::Info > Severity::Critical;                // true

    // _____________________________________________________________________________________________
    // MAKE INVALID STATES UNREPRESENTABLE
    // 1. CORRELATED FIELDS
    //    If any two fields are CORRELATED - "correlated" means knowing the value of field A changes what values are valid for field B.
    //    Then the meanigful combinations of those fields should be Enum variants.
    //    1. Count the number of possible-states - the product of each field's domain (bool x bool x Option<T> = 2x2x(n+1))
    //    2. Count the number of meaningful-states.
    //    3. If possible-states > meaningful-states = some fields are correlated and the gap is the invalid states space.
    //       Replace the correlated field group with an enum whose variant count equals the valid state count.

    // ❌ A cat can't be both dead and hungry at the same time.
    // Possible-states = 4      alive+hungry, alive+satiated, dead+hungry, dead+satiated
    // Meaningful-states = 3    alive+hungry, alive+satiated, dead
    struct Cat {
        alive: bool,
        hungry: bool,   
    }

    // ✅ Only the three meaninfgul states are representable
    enum Catt {
        Alive { hungry: bool },
        Dead,
    }

    // _____________________________________________________________________________________________
    // IMPL
    // Impl block defines behaviour on the data - struct or enum.
    // 1. Name of impl block must match an existing struct/enum in the scope.
    // 2. Self - Inside an impl, Self = the type. Used for creating instances or returning the type.
    // 3. self - The instance itself - parameter keyword for methods - &self (reads), &mut self (mutates), self (consumes).
    // Associated functions - Don't take self. Often used as constructors.   Called TypeName::function_name()
    // Methods              - Take self. Used as instance methods.           Called instance.method_name()

    impl Point {
        // ------------------------
        // ASSOCIATED FUNCTIONS - Commonly used as constructors. No self param, called as Point::new().
        fn new() -> Self {      // "new" is a convention only, not a keyword
            Self {x: 0, y: 0}   // Inside an impl, Self = the type. Used for creating instances of sel
        }

        fn from_xy(x: i32, y: i32) -> Self {    // separate functions instead of overloading
            Self {x, y}
        }

        // ------------------------
        // METHODS - First param is self. Called on the instance as p.distance().
        fn distance(&self) -> i32 {     // &self - reads self only. Instance is alive afterwards
            (self.x - self.y).abs()
        }

        fn move_x(&mut self, t: i32) {  // &mut self - mutates self. Instance is alive afterwards
            self.x += t;
        }

        fn into_scalar(self) -> i32 {   // self - moves self. Remember functions can kills variables by moving (kills self in this casse),
            self.x + self.y
        }
    }

    // Multiple impl blocks are allowed
    // Why you'd want it? 
    //   1. Organisation - separate large impl blocks into multiple. 
    //   2. Express "these three methods need bound X, these two don't". 
    //   3. Separate your own methods from trait implementations - impl Foo {} vs impl Display for Foo {} - they are necessarily separate blocks anyway
    impl Point {
        fn is_equidistant(&self) -> bool {
            self.x == self.y
        }
    }

    // Associated fn   TypeName::function
    let mut p = Point::new();           
    let mut p = Point::from_xy(0, 0);   

    // Methods          instance.method
    p.distance();       // struct is & borrowed during the operation
    p.move_x(10);       // struct is &mut borrowed during the operation
    p.into_scalar();    // struct is moved here
    // p;               // ❌ ERROR - 'p' is dead, it's been moved

    // IMPL AND PRIVACY
    // impl functions are private by default. You can't specify 'pub' on 'impl' itself - instead you specify 'pub' on each function individually.
    // ⭐ This goes well together with struct's private-by-default fields to achieve encapsulation nicely: Write your struct + impl, then expose only the public api with pub impl functions.
    impl User {
        pub fn hi() { println!("user says hi") }    // public function
        fn hello() { println!("user says hello") }  // private function
    }
    
    // IMPL ON ENUMS
    // impl on enums works exactly like it does on struct, with the only different is that you tend to pattern match self everywhere, since you don't know the shape and fields in advance.
    // Self can be used in place of enum name, as you'd expect - Self::Running is identical to Status::Running.
    impl Status {
        fn running_time(&self) -> i32 {
            match self {
                Finished(n) => { return *n },
                Running { t, p } => { return *p },
                _ => { return -1 },
            }
        }
    }

    // IMPLS AUTOMATICALLY GET APPLIED
    // ⭐ You can't import a type (struct/enum) and not import it's trait. Importing a struct from a crate auto-brings all it's behaviours (impls) with it.
    // Importing an impl is not even a thing. Since you can only add impl to types in the same crate - you can think of impl blocks as "fused" to the type they are impl-ing and always go with them everywhere.

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

fn take_user(u: User) {}
struct User { name: String, email: String }

enum Direction { Up { i: i32 } }
struct Duck;
