#![allow(unused)]
use std::{fmt::Display, rc::Rc, sync::Arc};
use uuid::Version::Mac;


fn main() {

    // _____________________________________________________________________________________________
    /* 
    POINTERS    (in general)
    Normal variable - Everything in memory is reached via an address (hardware fact) incl if a variable is just a simple i32 number. 
                    let x: i32 = 5 puts four bytes somewhere, and to read the value CPU needs read it from a memory location. That doesn't mean it's a pointer.
        
    Pointer - A value that IS ONLY the memory address (thin pointer), or it CONTAINS the memory address with other metadata fields (fat pointer).
            Just like a normal variable a pointer value is accessed by the memory address. The difference is that the CONTENT of the pointer value is the memory address of something else.
    
                        ADDRESS             CONTENTS
    Normal variable     0x00000000 ->       42 (data)
    
    Pointer             0x00000000 ->       0x00000001 (another address)
                        0x00000001 ->       42 (data)

                        
    POINTERS IN RUST
    
    Normal Variables (not pointers)
    i32, u64, f64, bool, char, isize, usize     — scalars
    [i32; N], (i32, i32) - arrays, tuples       - elements just sit there inline
    structs, enums                              - tag and payload inline
    str, [T], dyn Trait                         - the data itself
    
    Pointers
    &, &mut         - borrow; the address of someone else's value
    Box<T>          - your traditional pointer; just the address of something else
    Rc<T>, Arc<T>   - your traditional pointers; with sharing / threading semantics on top
    &dyn Trait      - address + vtable pointer/address?
    &str, &[T]      - fat pointers; address + length
    Vec<T>          - address + length + capacity (growable)
    String          - address + length + capacity (growable)

    */
    
    struct ThinPointer {
        memory_address: usize
    }
    
    struct FatPointer {
        memory_address: usize,
        length: usize,      // metadata fields
        capacity: usize,
    }

    // _____________________________________________________________________________________________
    // TRAITS
    // Think of a trait as a contract/interface/a promised set of behaviours/a named set of capabilities. Decoupled from any type that has them.
    // Use cases: 
    //    1. Abstraction - I don't care what Database it is, as long as it can .store() elements for me, I can use it.
    //                     Implement this trait and suddenly this library knows how to work with your type (e.g. serialise).
    //    2. Attachable behaviour - Attach new behaviours for author's other type, or a type from a common lib.
    //                              Attach new behaviours to a type you don't own (for convenience).
    //                              Attach new behaviours as recognised parts of ecosystem (that others or you already support)
    //    3. Generic bounds - (T: Trait) A promise that a generic param is capable of X, Y, Z behaviours that become available to the generic code. 
    //                        Generic params are otherwise a bit useless without those capabilities.
    
    // A trait is a named set of behaviours that a type can promise to implement - like an interface.
    // The only difference between traditional interfaces is that you can implement a trait separately (write it anywhere) from the main type definition (struct + impl).
    
    // Defining
    // 1. The body is a list of signatures without the body. Read this as "any type that is Animal must be able to tell you its name and make a sound."
    // 2. Default methods - they have a body and implementers can use it for free or override them
    //    Required vs default is your API's minimum-vs-convenience split. When designing a trait, ruthlessly minimise required methods — every one is a cost paid by every implementer, forever.
    //    So a trait becomes: implement those 1-2 things, get those 20 things for free. The trait defines the algorithm and the implementer fills in the holes
    //    That's how iterator work: you implement next() and std gives you map, filter, collect, etc.
    // 2. Self - Inside a trait Self means "whatever teh concrete type ends up implementing this". Used for returning the implementer's own type with '-> Self'.
    // 3. self, &self, &mut self - they mean the implementer's instance itself and define the api contract
    trait Animal {
        const LEGS: i32;            // Associated constant - Every implementer must add the const value
        const HEARTS: i32 = 1;      // Default associated constant 
        
        fn new() -> Self;           // Associated function 
        fn name(&self) -> String;   // Method
        fn sound(&self);            
        fn non_required(&self) {    // Default method - Has body. Can call required methods - Because implementers are guaranteed to provide them
            self.sound();
        }                           
    }                               
    
    // Implementing a trait
    // 1. You must implement every required method. 
    // 2. You can't add extra methods or constants (things not declared by trait).
    impl Animal for Cat {
        const LEGS: i32 = 4;
        fn new() -> Self { Cat { alive: true, hungry: false } }
        fn name(&self) -> String { "Kitty".to_string() }
        fn sound(&self) { println!("meow"); }
    }

    // Calling trait methods
    // After implementing a trait for a type you can automatically start calling that trait's methods on the type's instances.
    let cat: Cat = Animal::new();            // trait-qualified
    let cat = Cat::new();               // same, from type name
    let cat = <Cat as Animal>::new();   // fully qualified
    cat.sound();                             // method syntax

    // You only need the fully qualified version when a type implements multiple traits that have colliding methods, so you need to disambiguate
    struct Human;
    trait Pilot { fn think(&self); }
    trait Scientist { fn think(&self); }
    
    impl Human { fn think(&self) { } }
    impl Pilot for Human { fn think(&self) { } }
    impl Scientist for Human { fn think(&self) { } }

    let human = Human {};
    human.think();                      // when unspecified, the inherent impl takes priority
    <Human as Pilot>::think(&human);    // when ambiguous you must use the fully qualified version

    // Take as param or return
    fn i_take_trait(a: impl Animal) {}
    fn i_return_trait() -> impl Animal { Cat::new() }

    // Marker traits
    // A trait with an empty body promises nothing behavioural — it just labels the type. It's a promise you're making as the implementing author.
    // Copy, Send, Sync, Eq, and Sized are all marker traits.
    trait Serialisable {}
    impl Serialisable for Cat {}
    
    // Supertraits
    // Supertrait - a trait that depends on another trait already having been implemneted (like a pre-requisite). You can only do X behaviour if you can already do Y and Z behaviours.
    // trait Run: Walk means "you can only run if you can already walk" - and in exchange Run's default bodies may use Walk's methods.
    trait Walk { 
        fn walk(&self); 
    }
    trait Run: Walk {       // to depend on more that one trait the syntax is 'trait Run: Walk + Crawl {}'
        fn run(&self) {
            self.walk()
        } 
    }

    // _____________________________________________________________________________________________
    // THE ORPHAN RULE
    // You may implement a trait for a type only if the trait local to your crate, or the type is local to your crate (or both), but not if both are foreign.
    // Why it exists: If two unrelated crates/dependencies were both allowed to implement a (Trait, Type) pair they dont own, and your code depends on both - this create a conflict - which implementation should compiler pick?
    // What it achieves: At most one crate in the dependency graph is ever allowed to implement the (Trait, Type) pair. No conflicts.

    // Implementing YOUR trait on a FOREIGN type
    // ...

    // YOUR type implementing a FOREIGN trait
    // ...

    
    // The workaround for when you need "foreign trait on foreign type" - Wrap the foreign type in a local tuple struct, then the type is yours. 
    struct Wrapper(Vec<String>);          // now local
    impl std::fmt::Display for Wrapper {  // ✅ foreign trait, but on a local type — allowed
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    // Cost - you loose direct access to the underlying type's methods, you you need Deref or explicit .0 acces

    // Note: if your type appears as a generic parameter in the trait implementation signature then it's okay to implement a foreign trait on a foreign type:
    //       impl foreign::Trait<MyType> for foreign::OtherType can work.


    // You can implement YOUR trait for a FOREIGN type, or have YOUR type implement a FOREIGN trait, but never a FOREIGN trait on FOREIGN type - see the Orphan Rule.
    // ⭐ This is huge: That means you can add behaviour to types you don't own - something other languages don't allow. That's why Rust never needed a "utils" class: you add behaviour to existing types instead.
    //    E.g. in Java you can't add an interface implementation to a class you don't own without subclassing it (Java simply has no mechanism for it because 'implements' is part of the class definition.)
    // Benefits: 
    //   1. the whole ecosystem can layer new abstractions over existing code without forking it
    //   2. without the original author anticipating your need
    //   3. and without losing method-call syntax or replacing every occurance with the new interface - it just works. 
    
    // 1. Implement your trait for a foreign type
    trait Summarisable { 
        fn summary(&self) -> usize; 
    }
    impl Summarisable for String {
        fn summary(&self) -> usize {
            self.chars().count()
        }
    }
    let s = String::from("hi");
    s.summary();
    
    // 2. Implement foreign trait for your type
    struct Counter(i32);
    impl PartialEq for Counter {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    let a = Counter(1);
    let b = Counter(1);
    a == b;

    // 3. Implement your trait for your type
    struct Dog;
    trait Greets { fn greet(&self); }
    impl Greets for Dog { fn greet(&self) { println!("woof"); } }
    let dog = Dog;
    dog.greet();
    
    // Trait must be in scope
    // ⭐ You must ensure the trait is in scope and do use::.. if needed. Otherwise you don't have access to the trait's behaviours.

    // _____________________________________________________________________________________________
    // AUTO TRAITS
    // An auto trait is a trait the compiler implements for you automatically, on any type whose parts all implement it.
    // A struct is Send if all its fields are Send. And so on for other auto traits. 
    struct Meow { data: String };   // String is Send and Sync, so the entire struct is also Send and Sync
    let m: &dyn Send = &Meow { data: "hi".to_string() };
    
    trait CatSound {}
    impl CatSound for Meow {}
    let s: Box<dyn CatSound + Send + Sync> = Box::new(Meow { data: "meow".to_string() });

    // 1. Auto-traits are market traits - they don't have methods, instead they assert a property (e.g. "safe to move to another thread", etc).
    // 2. There are only a handful of them: Send, Sync, Unpin, UnwindSafe, RefUnwindSafe
    // 3. Automatic traits Send and Sync mean every type gets correct thread-safety marking for free without authors thinking about it,
    //    and a single non-thread-safe field poisons the whole type exactly as it should
    
    // _____________________________________________________________________________________________
    // GENERICS
    // How to think of generic code:
    // Structure (with some parts generic, some concrete "utils") and behaviours that can operate on that structure 
    // using parametrised types (vectors, collections, containers) and methods from generic bounds.

    // GENERIC STRUCT
    struct Wrap<T> {     // Wrap is not a type, Wrap<i16> and Wrap<i32> are types.
        inner: T,
    }
    let a = Wrap::<i16>{ inner: 10 };   // explicit - Wrap<i16>
    let a = Wrap { inner: 10 };         // inferred - Wrap<i32>
    
    struct Pair<A, B> { left: A, right: B }   // Two generic params
    struct Grid<T> { cells: Vec<Vec<T>> }     // Nested generic params

    
    // GENERIC ENUM
    enum Jellyfish<T> {
        Purple(T),
        Blue(T),
    }
    let p = Jellyfish::<i32>::Purple(10);   // explicit
    let p = Jellyfish::Purple(10);          // inferred


    // GENERIC IMPL
    // An impl can only be generic if its implementing a generic type or a generic trait for a type.
    // ✅ impl          + struct               struct Cat {}       impl Cat {}
    // ✅ impl          + trait                struct Cat {}       impl Audible for Cat {}
    // ✅ generic impl  + generic trait        struct 
    // ✅ generic impl  + generic struct       struct Cat<T> {}    impl<T> Cat<T> {}
    
    // Why two Ts (after impl and after the type)? - Firt introduces it, second uses it.
    // impl<T>   Wrap<T>
    //      ^         ^
    //      |         └── USING it: "We're using the introduced T."
    //      └── DECLARING it: "T is a parameter of this impl block. We're just introducing it."
    impl<T> Wrap<T> {
        fn new(inner: T) -> Self {
            Self { inner }
        }
        fn get(&self) -> &T {
            &self.inner
        }
        fn zip<U>(self, other: U) -> Wrap<(T, U)> {  // Methods can also carry their own generic params unrelated to struct's
            Wrap { inner: (self.inner, other) }
        }
    }


    // TURBOFISH
    function::<T>();            // generic function
    MyType::functionn::<T>();   // generic associated function on a type
    instance.method::<T>();     // generic method on an instance
    MyType::<T>::function();    // generic type's associated function


    // BOUNDS
    // A bound is a constraint on a generic param - the param must implement a trait, and in return the generic code can call the trait's methods.
    // There are 3 ways to write it.
    
    trait Audible { fn make_sound(&self); }
    trait Tangible { fn make_touch(&self); }

    // 1. T: Trait - means T must implement Trait
    fn f<T: Audible>(a: T) {}
    fn f2<T: Audible + Tangible>(a: T) {}   // multiple bounds

    // 2. Where clause - for anything non-trivial
    fn f3<T>(a: T) where T: Audible {}
    fn f4<T>(a: T) where T: Audible + Tangible {}   // multiple bounds

    // 3. impl Trait - anonymous generic param (when you don't need to specify T anywhere else)
    fn f5(a: impl Audible) {}
    fn f6(a: impl Audible + Tangible) {}    // multiple bounds


    // IMPL FOR A BOUND
    // Think of this as "conditional capabilities". When the generic type T is capable of Display behaviour then we add a convenience method pretty_print().

    // Recall "how to think of generic code". An impl for a generic bound simply adds behaviours for a given subset of T (that satisfy the bound).
    // "This is my generic code. If the generic happens to be of type A or have capabilities X, Y, Z then my code gainst those convenience methods."
    // "This is my generic matrix - <Vec<Vec<T>>. If T happens to be an i32 then my matrix has a convenience method to add all the numbers together."

    // A common pattern is to have multiple impl blocks, each implementing methods for a different bound or combination of bounds.
    // As a result, depending on what bounds a type satisfies it has access to a different set of behaviours (impls for that bound).
    struct Matrix<T> {
        data: Vec<Vec<T>>
    }
    
    impl<T> Matrix<T> {
        fn new(rows: usize, cols: usize, new_elem: impl Fn() -> T) -> Matrix<T> {
            let mut x: Vec<Vec<T>> = Vec::with_capacity(cols);
            for _ in 0..rows {
                let r = (0..cols).map(|_| new_elem()).collect();
                x.push(r);
            }
            Matrix { data: x }
        }
    
        fn insert(&mut self, idx_x: usize, idx_y: usize, e: T) {
            self.data[idx_x][idx_y] = e;
        }
    }

    // methods that will only be available on Matrix<i32> - think of them as convenience
    impl Matrix<i32> {
        fn sum(&self) -> i32 {
            let mut s = 0i32;
            for i in &self.data {
                for j in i {
                    s += *j;
                }
            }
            s
        }
    }

    // Methods that will only be available when Matrix<T: Display>
    impl<T: Display> Matrix<T> {
        fn pretty_print(&self) {
            for i in &self.data {
                for j in i {
                    print!("{} ", j);
                }
                println!();
            }
        }
    }

    let mut m: Matrix<i32> = Matrix::new(10, 10, || 0);
    m.pretty_print();   // Only possible because i32 implements Display
    m.sum();            // Only possible because matrix type is i32


    // IMPL TRAIT FOR A BOUND
    // Think of this as conditionally propagating the capability of generic type T to our generic Matrix - if T is able to Display then this makes Matrix itself be able to Display.

    // Just like before 'impl<T: Display>' means this implementation adds behaviour only for T: Display subset of types.
    // The only difference is that we're implementing a trait for this subset of types.
    impl<T: Display> Display for Matrix<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for i in &self.data {
                for j in i {
                    write!(f, "{} ", j)?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }

    // MONOMORPHISATION
    // When using generics, the compiler writes out a separate copy of the code for each concrete type you actually use with T substituted.
    // This is called monomorphisation and is the real cost of generics. As a result:
    // 1. Zero runtime cost - each copy is as fast as the hand written non-generic one would be, because it doesn't do any lookups or indirections.
    // 2. Binary size and compile time - increase. Calling a generic function for 30 types compiles it 30 times. This is a real contributor to Rust's compile times.

    
    // LIMITATIONS OF GENERICS
    // What monomorphisation cannot do - is to mix multiple types inside one invocation - which compiled copy would handle that? That's why 'dyn' exists.
    trait Database { fn turn_on(&self); }
    struct Postgres;
    struct MySQL;
    impl Database for Postgres { fn turn_on(&self) {} }
    impl Database for MySQL { fn turn_on(&self) {} }

    // 1. Collections can't hold a mix of trait implementations
    // let v : Vec<impl Database> = Vec::new();     // ❌ not possible

    // 2. Single function cannot return different implementations
    fn i_return_bd_impl() -> impl Database {
        // if true { return Postgres {}; }       // ❌ not possible
        return MySQL;
    }

    // _____________________________________________________________________________________________
    // STATIC VS DYNAMIC DISPATCH
    // ⭐ When you see &dyn Trait (or any version of it), think: sized pointers to concrete implementations; interleavable together; indirection is the cost.
    // ⭐ When you see impl Trait, think: monomorphisation, duplicate concrete implementations, NOT interleavable together, no indirection/pointer cost; slightly bigger binaries / compile times.
    
    // Why do generics have the above limitation? Because each type takes a diff amount of bytes in memory, and so for example collection wouldn't know how much memory to reserve for each item if the are all different types.
    // How does dyn Trait solve this? dyn Trait is a pointer to a concrete type, and since all pointers are of the same size in memory you can store them in a collection etc.
    // You essentially buy memory size determinism with the price of indirection.

    // _____________________________________________________________________________________________
    // DYN TRAIT (Dynamic dispatch)
    // dyn Trait <- means "pointer to some type that implements Trait. Concrete type is only known at runtime".
    // You can never have one as a bare value, because its 'unsized', '!Sized', meaning the compiler doesn't know its size at compile time.
    // You can only have it behind a pointer, because the pointer is of a fixed size. 

    // let concrete: dyn Database = Postgres;               // ❌ not possible
    let pointer: &dyn Database = &Postgres;                 // ✅ &dyn           - read pointer
    let pointer: &mut dyn Database = &mut Postgres;         // ✅ &mut dyn       - mutable pointer
    let pointer: Box<dyn Database> = Box::from(Postgres);   // ✅ Box<dyn ...>   - box pointer
    let pointer: Rc<dyn Database> = Rc::from(Postgres);     // ✅ Rc<dyn ...>    - reference counted pointer
    let pointer: Arc<dyn Database> = Arc::from(Postgres);   // ✅ Arc<dyn ...>   - atomic reference counted pointer

    // Automatic coercion
    // &value -> coerces to &dyn Trait automatically when value implements Trait

    // Overcoming generics limitation 1: Heterogeneous collections
    let v: Vec<&dyn Database> = vec![&Postgres, &MySQL];
    let v: Vec<&mut dyn Database> = vec![&mut Postgres, &mut MySQL];
    let v: Vec<Box<dyn Database>> = vec![Box::new(Postgres), Box::new(MySQL)];

    // Overcoming generics limitation 2: Returning different trait implementations per call
    fn i_return_bd_impl2(flag: bool) -> Box<dyn Database> {
        if flag { 
            return Box::new(Postgres); 
        } else {
            return Box::new(MySQL);     // ✅ now possible
        }
    }

    // Dyn compatibility
    // It order to be dyn compatible a trait must follow the below rules. Rules can be summarised as "everything has to be against self only."
    // 1. No generic methods - because they'd be monomorphised - the vtable would neeed one slot per possible T.
    // 2. No functions without 'self' param (methods only) - you need 'self' to resolve the type
    // 3. No Self in return position - the caller can't have a value of unknown size back
    //    Returning Box<Self>                   is fine.
    //    Returning Self where Self: Sized;     is fine also, because we exclude it from vtable entirely basically
    // 4. No associated constants

    // dyn A + B + C
    // Just like for generics, the syntax means "the thing must implement all traits specified - A, B, C"
    // For dynamic dispatch: First trait has to a normal trait, and the rest (everything after +) have to be auto-traits or lifetimes. 
    let a: Box<dyn Database + Send + Sync> = Box::new(Postgres);    // works because Send and Sync are auto-traits

    
    // IMPL TRAIT (Static dispatch)
    // impl Trait <- means "one specific concrete type that the compiler fully knows", you're just aren't writing its name. 
    // Monomorphized, fully Sized, no pointers, no indirection, duplicated concrete implementation per type.

    // _____________________________________________________________________________________________
    // ENUM INSTEAD OF DYN
    // Before reaching for dynamic dispatch, check if what you want is a closed set which can be implemented with enum
    // Heterogeneous collections and returning diff implemetattions from same function call are possible because 
    // the collection allocates the size of the biggest enum variant to each capacity slot in memory, so in the worst case a little bit of memory is wasted.

    // in a real enum dispatch each enum variant would take db connection as param
    enum AnyDatabase {  
        Postgres,
        MySQL
    }

    // Overcoming generics limitation 1: Heterogeneous collections
    let v: Vec<AnyDatabase> = vec![AnyDatabase::Postgres, AnyDatabase::MySQL];

    // Overcoming generics limitation 2: Returning different trait implementations per call
    fn i_return_bd_impl3(flag: bool) -> AnyDatabase {
        if flag { 
            return AnyDatabase::Postgres; 
        } else {
            return AnyDatabase::MySQL;     // ✅ now possible
        }
    }

    // Cost:
    // 1. Every element is as big as the size of the largest variant
    // 2. Adding a new subtype requires editing the enum to add new variant and editing all the match statements

    // Benefit:
    // Heterogeneous collections and funcs that return diff types per call without cost of indirection

    // _____________________________________________________________________________________________
    /* 
    
    SUMMARY - STATIC VS DYNAMIC DISPATCH
    
                                          │ impl Trait (static dispatch) │ dyn Trait (dynamic dispatch)  │
    How many concrete types at this spot? │ Exactly one                  │ Potentially many              │
    Does the compiler know which?         │ Yes                          │ No                            │
    Different types per branch?           │ No                           │ Yes                           │
    Sized?                                │ Yes — stored inline          │ No — needs a pointer (&, Box) │
    Cost                                  │ Monomorphization (code size) │ Pointer chase + no inlining   │
    
                                 │   <T: Trait> (static)   │     &dyn Trait (dynamic)     
    Resolved                     │ compile time            │ runtime                      
    Code generated               │ one copy per type       │ one copy total               
    Call cost                    │ direct, inlinable       │ one indirection, no inlining 
    Binary size                  │ grows per instantiation │ constant                     
    Compile time                 │ slower                  │ faster                       
    Heterogeneous collections    │ ❌ impossible           │ ✅ the point                 
    Trait must be dyn-compatible │ no                      │ yes                          

    HOW TO PICK
    Hot path; one implementer per call site, known at compile time.     Static dispatch, <T: Trait>
    Hot path; closed set you own; exhaustiveness checking;              Enum
    Open set; heterogeneous collections you don't control;              Dynamic dispatch, dyn Trait

    TIPS
    - Cost of indirection matters in tight inner loops on hot paths. No need to optimise prematurely.
    
    SYNTAX SUMMARY
    <T>             Static dispatch, generic param
    <T: A>          Static dispatch, generic param with bound
    impl A          Static dispatch, anonymous trait
    dyn A           Dynamic dispatch
    
    */

    // _____________________________________________________________________________________________
    /* 
    REFERENCE: HOW DYN TRAIT POINTERS ARE IMPLEMENTED AND VTABLE
    How dyn Trait pointer looks like in memory:
    dyn Trait pointer ──data pointer──▶ the actual Cat struct
                     └─vtable pointer─▶ the vtable ──[3]──▶ speak() machine code
                     
    1. fetch the data (struct) by following the data pointer; 
    2. follow the vtable pointer and in the vtable follow the method's pointer to fetch the function's code; 
    3. feed the data as parameter to the function
   
    */
    struct DynTraitPointer {
        data_memory_address: usize,
        v_table_memory_address: usize,
    }

    struct VTableForTypeTraitPair {     // 1st hop: from v_table_memory_address to this table
        type_size: usize,
        pointer_to_drop_code: usize,
        // ...
        pointer_to_method_a: usize,     // 2nd hop: from this table to method a
        pointer_to_method_b: usize,
        pointer_to_method_c: usize,
        // ...
    }


    // _____________________________________________________________________________________________
    // REFERENCE: A + B + C syntax
    // Means the thing must implement all the traits specified. 

    // Static dispatch:
    // For static dispatch: A, B, C can be normal traits
    fn i_take_thing<T: Database + Send + Sync>(a: T) {}         // Static dispatch, generic bounds syntax
    fn i_take_thing2(a: impl Database + Send + Sync) {}         // Static dispatch, anonymous generic syntax

    // Dynamic dispatch
    // For dynamic dispatch: First trait has to a normal trait, and the rest (everything after +) have to be auto-traits or lifetimes. 
    fn i_take_thing3(a: Box<dyn Database + Send + Sync>) {}     // Dynamic dispatch. Only works because Send and Sync are auto-traits
    

    // _____________________________________________________________________________________________
}



struct Cat { alive: bool, hungry: bool }

struct MyType <T> { data: T }
impl<T> MyType<T> { fn method<U>(&self) { } fn function() {} }
impl MyType<i32> { fn functionn<T>() {} }
type T = i32;   // gives the call sites below a real type named `T` to point at
fn function<T>() {}
#[allow(non_upper_case_globals)]
const instance: MyType<i32> = MyType { data: 42 };







/* 
oop.rs -> is about syntax of oop constructs.
oop2.rs (this file) -> about patterns of writing oop code

OOP principles:
- Single responsibility
- Encapsulation
- Abstraction
- Minimal Coupling


[] You typical class (struct + impl) with invariants as private fields, and the public / private mehtods - how to design this?
   Book chapters: 5.1, 6.1, 17.1, 17.2, 17.3
[] Composition, not inheritance
[] The getter story
[] Polymorphism - when to use static vs dynamic dispatch
   https://www.youtube.com/watch?v=m_phdVlkr6U
[] Data modelling

*/


