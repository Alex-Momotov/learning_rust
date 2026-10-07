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
    // Trait content is a list of signatures without the body. Read this as a contract / list of behaviours every implementing type must satisfy.
    // Good trait design    - Required vs default methods is your API's minimum-vs-convenience split. When designing a trait, ruthlessly minimise required methods to make it easier for implementers.
    //                        So a trait becomes: implement those 1-2 things, get those 20 things for free. The trait defines the algorithm and the implementer fills in the holes.
    // Self                     - Inside a trait Self means "whatever teh concrete type ends up implementing this". Used for returning the implementer's own type with '-> Self'.
    // Self::X                  - Reference to associated type
    // self, &self, &mut self   - they mean the implementer's instance itself and define the api contract
    trait Animal {
        type X;                     // Associated type - Implementer must choose what type it is, then the trait can refer to it as 'Self::X'.
        const LEGS: i32;            // Associated constant - Every implementer must add the const value. Associated constants can have a default value e.g. 'const LEGS: i32 = 4;' (implementers can override)

        fn new() -> Self;           // Associated function 
        fn sound(&self);            // Method
        fn non_required(&self) {    // Default method - Has body, can call required methods - Because implementers are guaranteed to provide them. Implementers can use it for free or override it.
            self.sound();           
        }
    }


    // Implementing
    // You must implement every required method, associated type, and associated constant. 
    // You can't add extra methods or constants (things not declared by trait).
    impl Animal for Cat {
        type X = i32;           // Implementer chooses the associated type once. This pins it for this type and trait forever
        const LEGS: i32 = 4;

        fn new() -> Self { Cat { alive: true, hungry: false } }
        fn sound(&self) { println!("meow"); }
    }

    
    // Instantiating
    // You can't annotate a value directly as trait 'let a: Animal' because a trait is not a type - its a constraint something can satisfy. Annotation = memory layout information.
    // Also, there are several ways to have a trait value (static vs dynamic dispatch), with different tradeoffs each, and so Rust makes you pick one explicitly.
    // let a: Database = Postgres;   // ❌
    
    // Your choices:
    let db = Postgres;                       // Instantiate directly as concrete type
    let db = return_db();               // From a function return (static dispatch) - Must be without annotation (impl Trait), because generic params aren't allowed in let statements
    let db: Box<dyn Database> = Box::new(Postgres);    // Pointer to the value (dynamic dispatch)

    
    // Using
    // After implementing a trait for a type you can automatically start calling that trait's methods on the type's instances.
    let cat: Cat = Animal::new();            // trait-qualified
    let cat = Cat::new();               // same, from type name
    let cat = <Cat as Animal>::new();   // fully qualified
    cat.sound();                             // method syntax


    // _____________________________________________________________________________________________
    // You cannot implement the same trait for a given type twice 
    // (unless the trait is generic and implementations use diff generic types)
    trait Tr {};
    trait GenTr<T> {};
    
    struct A;
    
    impl Tr for A {}
    // impl Tr for A {}     ❌ Not allowed to implement it second time
    
    impl GenTr<i32> for A {}
    impl GenTr<bool> for A {}   // ✅ Multiple implemnetations allowed, because they use different generic param types (monomorphisation, so in fact they are different traits)
    
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

    // 1. Auto-traits are marker traits - they don't have methods, instead they assert a property (e.g. "safe to move to another thread", etc).
    // 2. There are only a handful of them: Send, Sync, Unpin, UnwindSafe, RefUnwindSafe
    // 3. Automatic traits Send and Sync mean every type gets correct thread-safety marking for free without authors thinking about it,
    //    and a single non-thread-safe field poisons the whole type exactly as it should
    

    // _____________________________________________________________________________________________
    // Associated types vs Generic traits

    // The difference between the two is that associated type means this is a normal trait without generic params and so the number of times you can implement it for a given type is restricted to once only.
    // Choose associated type over generic trait when it makes sense to have only one trait implementaiton (like with iterators).
    
    // Generic trait - multiple implementations per type. Which one is chosen by the caller at the call site.
    // Associated type - one implemnetation per type. Which one is chosen by the implemented once.


    // Generic trait - can have multiple implementations for a given type
    trait GenericIter<T> {
        fn next(&mut self) -> Option<T>;
    }
    // Normal trait - the number of implementations is restricted to one. Implementer chooses and pins associated type once
    trait AssociatedTypeIter {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    
    struct MyArray {
        array: [i32; 10],
        i: usize
    }

    // First implementation for T = i32
    impl GenericIter<i32> for MyArray {
        fn next(&mut self) -> Option<i32> {
            match self.array.get(self.i) {
                Some(n) => {self.i += 1; return Some(*n)},
                None => return None,
            }
        }
    }

    // Second implementation for T = bool     (we can implement generic trait for a type multiple times, even though it doesn't make sense)
    impl GenericIter<bool> for MyArray {
        fn next(&mut self) -> Option<bool> {
            Some(true)
        }
    }

    // The only one implemnetation, pinning the associated type
    impl AssociatedTypeIter for MyArray {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            match self.array.get(self.i) {
                Some(n) => {self.i += 1; return Some(*n)},
                None => None,
            }
        }
    }

    // _____________________________________________________________________________________________
    // Implementing an Iterator
    
    struct NumberArray {
        array: [i32; 5],
        i: usize
    }

    impl Iterator for NumberArray {
        type Item = i32;

        fn next(&mut self) -> Option<i32> {
            match self.array.get(self.i) {
                Some(n) => {self.i += 1; return Some(*n)},
                None => return None,
            }
        }
    }

    // You get all the methods for free now
    let mut arr = NumberArray { array: [1, 2, 3, 4, 5], i: 0};

    for i in &mut arr {
        println!("{:?}", i);
    }
    
    let v: Vec<i32> = arr.into_iter()
        .map(|i| i * i)
        .filter(|i| i % 2 == 0)
        .collect();
    


    // _____________________________________________________________________________________________

    // Using (2) - taking as param (without bounds)
    // When you don't specify associated type's bound, all you can do with it is pretty much just call the 
    // fn something1(c: impl Container) -> impl Container {
    //     let x = c.get();
    //     c.from_another(x)
    // }
    
    // // Using (3) - taking as param (with bound)
    // // Constraining in a bound - means we specify a bound as a trait and specify it's associated type
    // fn something(c: impl Container<X = i32>) {
    //     let i: i32 = c.get();   
    // }
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


trait Database {
    fn connect(&self);
}

struct MySql;
impl Database for MySql { 
    fn connect(&self) { println!("MySql connected!"); } 
}

struct Postgres;
impl Database for Postgres { 
    fn connect(&self) { println!("Postgres connected!"); } 
}

fn return_db() -> impl Database {
    Postgres
}

