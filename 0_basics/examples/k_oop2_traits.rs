#![allow(unused)]
use std::{fmt::Display, rc::Rc, sync::Arc};
use uuid::Version::Mac;


fn main() {

    // _____________________________________________________________________________________________
    // TRAITS
    
    // Think of a trait as a contract/interface/a promised set of behaviours/a named set of capabilities. Decoupled from any type that has them.
    // A trait is a named set of behaviours that a type can promise to implement - like an interface.
    // The only difference between traditional interfaces is that you can implement a trait separately (write it anywhere) from the main type definition (struct + impl).

    // Use cases
    //    1. Abstraction - I don't care what Database it is, as long as it can .store() elements for me, I can use it.
    //                     Implement this trait and suddenly this library knows how to work with your type (e.g. serialise).
    //    2. Attachable behaviour - Attach new behaviours for author's other type, or a type from a common lib.
    //                              Attach new behaviours to a type you don't own (for convenience).
    //                              Attach new behaviours as recognised parts of ecosystem (that others or you already support)
    //    3. Generic bounds - (T: Trait) A promise that a generic param is capable of X, Y, Z behaviours that become available to the generic code. 
    //                        Generic params are otherwise a bit useless without those capabilities.
    
    
    // Defining
    // 1. The body is a list of signatures without the body. Read this as "any type that is Animal must be able to tell you its name and make a sound."
    // 2. Default methods - they have a body and implementers can use it for free or override them
    //    Required vs default is your API's minimum-vs-convenience split. When designing a trait, ruthlessly minimise required methods — every one is a cost paid by every implementer, forever.
    //    So a trait becomes: implement those 1-2 things, get those 20 things for free. The trait defines the algorithm and the implementer fills in the holes
    //    That's how iterator work: you implement next() and std gives you map, filter, collect, etc.
    // 2. Self - Inside a trait Self means "whatever teh concrete type ends up implementing this". Used for returning the implementer's own type with '-> Self'.
    // 3. self, &self, &mut self - they mean the implementer's instance itself and define the api contract
    // 4. Associated type - is declared 'type x;' inside a trait and allows the trait to refer to it as a generic param using 'Self::x' everywhere, and for the implementer to choose what type it is.
    trait Animal {
        type X;                     // Associated type
        const LEGS: i32;            // Associated constant - Every implementer must add the const value
        const HEARTS: i32 = 1;      // Default associated constant 
        
        fn new() -> Self;           // Associated function 
        fn name(&self) -> String;   // Method
        fn age(&self) -> Self::X;   // associated type is referred to as Self::X
        fn sound(&self);            
        fn non_required(&self) {    // Default method - Has body. Can call required methods - Because implementers are guaranteed to provide them
            self.sound();
        }
    }                               

    
    // Implementing
    // 1. You must implement every required method. 
    // 2. You can't add extra methods or constants (things not declared by trait).
    impl Animal for Cat {
        type X = i32;           // Implementer chooses the associated type once. This pins it for this type and trait forever
        const LEGS: i32 = 4;

        fn new() -> Self { Cat { alive: true, hungry: false } }
        fn name(&self) -> String { "Kitty".to_string() }
        fn age(&self) -> i32 { 5 }                       // Note: once we choose the associated type as i32 in 'type X = i32' we refer to it as i32 throughput the rest of implementation
        fn sound(&self) { println!("meow"); }
    }

    
    // Using
    // After implementing a trait for a type you can automatically start calling that trait's methods on the type's instances.
    let cat: Cat = Animal::new();            // trait-qualified
    let cat = Cat::new();               // same, from type name
    let cat = <Cat as Animal>::new();   // fully qualified
    cat.sound();                             // method syntax
    cat.age();                               // associated type returned from a method. Associated type is inferred here automatically, no need to specify it 


    // _____________________________________________________________________________________________
    // Fully qualified syntax
    // You only need the fully qualified syntax when a type implements multiple traits that have colliding methods, so you need to disambiguate
    struct Human;
    trait Pilot { fn think(&self); }
    trait Scientist { fn think(&self); }
    
    impl Human { fn think(&self) { } }
    impl Pilot for Human { fn think(&self) { } }
    impl Scientist for Human { fn think(&self) { } }

    let human = Human {};
    human.think();                      // when unspecified, the inherent impl takes priority
    <Human as Pilot>::think(&human);    // when ambiguous you must use the fully qualified syntax

    
    // _____________________________________________________________________________________________
    // Marker traits
    // A trait with an empty body promises nothing behavioural — it just labels the type. It's a promise you're making as the implementing author.
    // Copy, Send, Sync, Eq, and Sized are all marker traits.
    trait Serialisable {}
    impl Serialisable for Cat {}

    
    // _____________________________________________________________________________________________
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
    // TODO (needs redoing) 
    // 1. Syntax for specifying Associated type bounds 
    // 2. implementing an iterator
    
    
    // Using (2) - taking as param (without bounds)
    // When you don't specify associated type's bound, all you can do with it is pretty much just call the 
    fn something1(c: impl Container) -> impl Container {
        let x = c.get();
        c.from_another(x)
    }
    
    // Using (3) - taking as param (with bound)
    // Constraining in a bound - means we specify a bound as a trait and specify it's associated type
    fn something(c: impl Container<X = i32>) {
        let i: i32 = c.get();   
    }
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


