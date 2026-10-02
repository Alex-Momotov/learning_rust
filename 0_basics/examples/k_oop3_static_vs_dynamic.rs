#![allow(unused)]

use std::ffi::FromBytesWithNulError;
use std::{convert::identity, ops::Deref, rc::Rc, sync::Arc};
use std::alloc::{Layout, alloc, dealloc, realloc};

fn main() {

    /* _____________________________________________________________________________________________
    POINTERS (in general)
    Normal variable - Everything in memory is reached via an address (hardware fact) incl if a variable is just a simple i32 number. 
                      let x: i32 = 5 puts four bytes somewhere, and to read the value CPU needs to read it from a memory location. That doesn't mean it's a pointer.
    Pointer - A value that IS ONLY the memory address (thin pointer), or it CONTAINS the memory address with other metadata fields (fat pointer).
              Just like a normal variable a pointer value is accessed by the memory address. The difference is that the CONTENT of the pointer value is the memory address of something else.
              
                        ADDRESS             CONTENTS
    Normal variable     0x00000000 ->       42 (data)
    
    Pointer             0x00000000 ->       0x00000001 (another address)
                        0x00000001 ->       42 (data)                                                   */

    struct ThinPointer {
        memory_address: usize
    }
    
    struct FatPointer {
        memory_address: usize,
        length: usize,          // metadata fields
        capacity: usize,
    }

    
    /* 
    POINTERS (IN RUST)
    
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
        String          - address + length + capacity (growable)        */

        
    /* _____________________________________________________________________________________________
    NORMAL EXECUTION (NO "PLACES" POINTING TO 'DYNAMIC THINGS')
    During normal execution of a program heap can grow and shrink based on runtime information all the time, because runtime inputs can decide which 
    types to create and how many of each. E.g. based on cli input create 10 cats or 20 dogs. That's not what this topic is about, because no single "place" 
    is pointing to a 'dynamic thing'. Instead, dynamism is when a single "place" is pointing to a 'dynamic thing' (unknown or changable size). */
    let rand_type = rand::random_bool(0.50);      
    let rand_how_many = rand::random_range(1..=5);

    let mut cats: [Option<Box<Cat>>; 5] = [None, None, None, None, None];
    let mut dogs: [Option<Box<Dog>>; 5] = [None, None, None, None, None];

    if rand_type {
        for i in 0..rand_how_many { cats[i] = Some(Box::from(Cat)) }
    } else {
        for i in 0..rand_how_many { dogs[i] = Some(Box::from(Dog)) }
    }
    
    /* 
    THE SIZE RULE, SIZED, POINTERS->!SIZED, DYNAMIC THINGS
    The size rule   The size of every "place" (variable, function param, function return, collection item, struct field, enum variant) must be known at compile time. 
                    So the only way a single "place" can represent something of a dynamic size is if its a pointer (which of a fixed size) to one of the two dynamic things
                    (which can't be instantiated directly, and can only exist behind pointers): 
                        1. A dynamically allocated region of memory (which can grow/shrink).
                           The ONLY way to have those is using unsafe code that works with raw pointers and an allocater to allocate, grow, and free blocks of memory. 
                           String and Vec are wrappers around that. PTR str and PTR [T] are pointers to snapshot of those.
                        2. A runtime chosen type of a value (the starting size varies, because different types have different sizes).
                           The ONLY way to have those is by creating a value normally (can be heap, stack or static), using runtime info or not, then create a pointer to it - PTR dyn Trait.

    Sized           Values. All values and "places". Known, fixed size. Pointers are also of a fixed size (Sized), so they CAN occupy "places".
    !Sized          Syntax for a pointer (not values). PTR !Sized. Syntax denoting what 'dynamic thing' a pointer is pointing to. 
                    Three !Sized primitives: [T], str (dynamically allocated regions of memory), dyn Trait (a runtime selected type for a value). 
                    Note: a type doesn't necessarily mean a type of a value. [T], str, dyn Trait are types but can never be values directly. Only can be PTR !Sized.

    * Note:         Together Pointer and !Sized (syntax of a pointer) point to a fixed snapshot of a 'dynamic thing'. Snapshots because the 'dynamic things' 
                    can change at runtime all the time, and so you can only point at a fixed snapshot of it, as it was at a given time. Hence fixed.     

    Dynamic thing   Abstract phenomena (not values, can't be written down). 'Dynamic' meaning it can change size or be of an unknown starting size, in contrast to the size rule. 
                    Only two 'dynamic things' exist: 1. Dynamically allocated region of memory (unsafe code); 2. Concrete type dynamically selected at runtime. 

    
    ___________________________________________________
    DYNAMIC THING 1 - DYNAMICALLY ALLOCATED REGIONS OF MEMORY

    UNSAFE + String + Vec - CREATING, GROWING, REMOVING DYNAMICALLY ALLOCATED REGIONS OF MEMORY
    The ONLY way to create and manage such a dynamically allocated region of memory is using UNSAFE code that does raw pointer arithmetic: 1. alloc() (ask the allocator 
    for a block of memory); write data directly into those addresses; realloc() (to grow that memory block - either get next addresses or copy where there's more space).
    Libraries such as Vec and String do this unsafe code pointer arithmetic for us behind the scenes, and that's how they are able to represent things of a dynamic size.
    Notice:
        1. The starting size of that allocated block of memory can be runtime information
        2. That allocated block of memory can grow at runtime
        3. ⭐ Since it's just a region of memory, raw pointers are the ONLY way to interact with it - create it, read it, write to it. 
           ⭐ That's why Vec and String are fat pointers (they manage a region of memory), and why slices PTR[T] and PTRstr (that represent a region of memory) are also fat pointers.

           
    SLICES (PTR[T] + PTRstr) - AGNOSTIC WAY TO POINT TO ANY REGION OF MEMORY (STATIC, STACK, HEAP), INCL A DYNAMICALLY ALLOCATED ONE
    ⭐ Slice types PTR[T] and PTRstr are pointers to a FIXED region of memory THEY DIDN'T THEMSELVES CREATE. And also they can view storage of ANY origin: heap, stack, static:
        - heap - dynamically allocated region of memory, using unsafe code and raw pointers (e.g. String, Vec)
        - stack - normal array
        - static - string literals baked into binary, constant arrays
    Those pointers point to a FIXED region of memory, meaning they cannot expand what they are pointing to. If they want to expand what they are pointing to, e.g. when the underlying buffer grows:
    The existing slice must drop; underlying buffer grows; a new slice pointing to the grown buffer is created.

    Think of [T] and str as "format of a region of memory" that acts as a qualifier to a pointer in front of it. "What KIND of a region of memory is this pointer pointing to?". 
        PTR[i32] -> the region of memory contains i32s. 
        PTRstr -> the region of memory contains valid UTF-8.
    PTR - can be &, &mut, Box, Rc, Arc and changes the pointer semantics.


    Relationship between Vec, String and PTR[T], PTRstr:
        - Vec, String - create and manage dynamically allocated regions of memory using unsafe code.
        - PTR[T], PTRstr - point to dynamically allocated regions of memory managed by Vec, and String, OR any other region of memory (stack, heap, static).

        Creating runtime-sized storage → allocator, always. 
        Representing/viewing it → slices, which create nothing and can view storage of any origin, including storage that was compile-time sized.    */

    
    unsafe { // ⭐ Unsafe code and raw pointers - is the ONLY way to create and manage dynamically allocated regions of memory.

        // Dynamically allocate a region of memory - 10 bytes
        let layout = Layout::array::<u8>(10).unwrap();       
        let p: *mut u8 = alloc(layout);                          

        // Write some stuff to it
        *p.add(1) = 1;                                               
        *p.add(9) = 9;

        // Read it
        println!("{:p} = {}", p.add(1), *p.add(1));                  
        println!("{:p} = {}", p.add(9), *p.add(9));

        // Grow the allocated region of memory - double to 20 bytes
        let new_layout = Layout::array::<u8>(20).unwrap();   
        let p = realloc(p, layout, new_layout.size());

        // Free the region of memory
        dealloc(p, layout);                                          
    } 

    
    /* ___________________________________________________
    DYNAMIC THING 2 - RUNTIME CHOSEN TYPE OF A VALUE

    CREATING RUNTIME CHOSEN TYPE OF A VALUE
    The ONLY way to create a dynamically chosen at runtime type of a value is statically. I.e. we compile a piece of code that creates different 
    values based on runtime input (e.g. Cat or a Dog), and the decision whether to actually instantiate that value or another is based on runtime inputs.

    
    PTR dyn Trait - AGNOSTIC WAY TO POINT TO A VALUE THAT IMPLEMENTS A TRAIT (STATIC, STACK, HEAP), INCL A RUNTIME CHOSEN ONE
    PTR dyn Trait is a pointer to a particular (fixed) concrete value that it didn't itself create - it was created elsewhere.
    The created value PTR dyn Trait is pointing to can actually live in any region of memory: static, stack, heap - the heap 
    being the most interesting one of course because it allows to choose the value at runtime.

    Think of dyn Trait as a qualifier to a pointer in front of it. "What KIND of a value is the pointer pointing to? What trait must it implement?"
    */

    // Runtime information choosing type (and therefore size) represented by a single "place" (animal variable)
    // The 'dynamic thing' here is the concrete type chosen at runtime (Cat or Dog) behind 'animal'.
    let runtime_information = rand::random_bool(0.50);
    let animal: Box<dyn Animal> = if runtime_information { 
        Box::from(Cat) 
    } else { 
        Box::from(Dog) 
    };
    

    /* _____________________________________________________________________________________________
    STATIC VS ENUM VS DYNAMIC DISPATCH
    The topic of static vs enum vs dynamic dispatch is all about how to do polymorphism over a trait (interface). 

    POLYMORPHISM
    Polymorphism - when code operates on an interface with potentially different implementing concrete types. "I don't care what it is, as long as it can quack I will treat it as a duck".
    Where polymorphism occurs:
    1. Function params and returns - I (a function) don't care what the implementation is, as long as it can .quack() I will call .quack() on it.
                                     A single function that returns an interface implementation, where the choice of implementation can be different between function calls.
    2. Generic bounds - when a generic parameter is expected to implement an interface (trait). The genric code can then call that trait's methods.
    3. Heterogeneous collections - one collection holding interfaces (traits) of different implementing types.
    4. Pluggable behaviour - struct/enum holding a trait object. E.g. App struct holding Logger trait.
    
    DISPATCH
    Dispatch - The act of choosing which function body actually runs for a given call. When you have a trait with two implementations behind it, how should compiler decide which implementation to call?
    - Static dispatch - The disambiguation happens at compile time, by looking at call sites and baking the implemnetation address at each.
    - Dynamic dispatch - the disambiguation happens at runtime, by fetching the implementation address from a pointer. Then call whatever address that turns out to be.   */

    
    /* _____________________________________________________________________________________________
    STATIC DISPATCH

    MONOMORPHISATION
    When using generics, the compiler writes out a separate copy of the code for each concrete type you actually use with T (generic param) substituted.
    More precisely: For each distinct set of concrete types substituted into a generic item's type parameters, compiler emits a specialised copy.
    Monomorphisation does two jobs:
    1. Generic params are substituted with concrete types.
       fn foo<T>(a: T)    ->    foo(5) and foo("hi")    ->   fn foo(a: i32)
                                                        ->   fn foo(&str)
    2. Traits are substituted with concrete implementations.
       fn foo(a: impl Animal)        desugars to     fn foo<T: Animal>(a: T)
       fn foo<T: Animal>(a: T)       foo(Cat) and foo(Dog)   ->    fn foo(a: Cat)
                                                             ->    fn foo(a: Dog)
       This is the same mechanism as 1 (T being substituted with concrete types), the only difference is that the bound restricts which types are allowed to be substituted.
    Behefit - Zero runtime cost - each copy is as fast as the hand written non-generic one would be, because it doesn't do any lookups or indirections.
    Cost - Binary size and compile time - increase. Calling a generic function for 30 types compiles it 30 times. This is a real contributor to Rust's compile times. 

    
    Static dispatch IS monomorphisation with a trait involved. It is: 1. Generics params with a trait bound; 2. Anonymous generics (which desugar to generic params with a trait bound)
    ⭐ Think of impl Trait (e.g. impl Animal) as a type hidden from you as a developer, but known and pinned by the compiler (because of monomorphisation).
    ⭐ When you see impl Trait (or any version of it), think: monomorphisation, duplicate concrete implementations, NOT interleavable together, no indirection/pointer cost; slightly bigger binaries / compile times.  */


    // Instantiating a trait
    let a = Cat;                        // Instantiate directly as concrete type
    let a = return_animal();    // From a function return (static dispatch) - Must be without annotation (impl Trait), because generic params aren't allowed in let statements


    // Taking trait as param
    fn take_animal1(a: impl Animal) {}          // 1. Anonymous generic (impl Trait), meaning the generic param itslef is not shown, as it's not used beyond specifying the generic bound. Desugars to 2.
    fn take_animal2<T: Animal>(a: T) {}         // 2. Generic with a trait bound.
    fn take_animal3<T>(a: T) where T: Animal {} // 3. Generic with a trait bound. Written differently

    take_animal1(Cat);                          // Obscures into the trait automatically at funciton boundary
    take_animal1(Dog);

    
    // Returning a trait
    fn return_animal() -> impl Animal {     // This is the only way. No desugaring is happening to a generic param
        Cat 
    }


    // LIMITATION OF STATIC DISPATCH
    // What monomorphisation cannot do - is to mix multiple types inside one invocation - which compiled copy would handle that? What would be the returned size? That's why 'dyn' and enum dispatch exist.
    // This is because each type occupies a different size, and so a single "place" (which must be Sized) like a collection item or fn return slot wouldn't know how much memory to reserve.

    // 1. Can't do heterogeneous collections
    // let v : Vec<impl Animal> = Vec::new();     // ❌ not possible

    // 2. Single function call cannot return different implementations
    fn i_return_animal_impl(flag: bool) -> impl Animal {
        // if flag { return Dog; }       // ❌ not possible
        return Cat;
    }

    
    // _____________________________________________________________________________________________
    // ENUM DISPATCH
    // When using enum dispatch, each "place" allocates the memory equal to the size of the biggest enum variant.
    // Use enum dispatch when: type needs to be decided at runtime AND the list of type choices is a known closed set that you control.
    // Benefit: 
    //    Fast dispatch, no indirection, heterogoneous collections, single function call returning different variants at runtime.
    // Cost: 
    //    1. Slight waste of space (each element reserves as much space as the biggest element)
    //    2. Adding a new variant requires updating all match statements

    
    // 1. Simple (just the enum, data lives in variants)
    enum Database { 
        Postgres { host: String }, 
        MySql { host: String, port: u16 } 
    }
    
    impl Database {
        fn connect(&self) -> String {
            match self {
                Database::Postgres { host }     => format!("Connected to Postgres: {host}"),
                Database::MySql { host, port } => format!("Connected to {host} {port}"),
            }
        }
    }

    
    // 2. Middle (structs, enum wrapping them, no trait)
    struct Reddis { host: String, port: u32 }
    struct MurMur { host: String }
    impl Reddis { fn store(&mut self, msg: String) {} }
    impl MurMur { fn store(&mut self, msg: String) {} }

    enum Cache {
        Reddis(Reddis),
        MurMur(MurMur)
    }

    impl Cache {
        fn store(&mut self, msg: String) {
            match self {
                Cache::Reddis(reddis) => reddis.store(msg),
                Cache::MurMur(mur_mur) => mur_mur.store(msg),
            }
        }
    }


    // 3. Full (structs, each implementing trait, enum wrapping them, and implementing trait itself)
    trait QueueLike { fn send(&mut self, msg: String) -> Result<String, String>; }

    struct Kafka { host: String, consumer_group: String }
    struct Nats { host: String, port: u16 }

    impl QueueLike for Kafka { fn send(&mut self, msg: String) -> Result<String, String> { Ok("done".to_string()) } }
    impl QueueLike for Nats { fn send(&mut self, msg: String) -> Result<String, String> { Ok("completed".to_string()) } }

    enum Queue {
        Kafka(Kafka),
        Nats(Nats)
    }

    impl QueueLike for Queue {
        fn send(&mut self, msg: String) -> Result<String, String> {
            match self {
                Queue::Kafka(kafka) => kafka.send(msg),
                Queue::Nats(nats) => nats.send(msg),
            }
        }
    }


    // ___________________________________________________
    // Enum dispatch - Overcoming limitations of static dispatch
    
    // Heterogeneous collections
    let postgres = Database::Postgres { host: "localhost".to_string() };
    let mysql = Database::MySql { host: "localhost".to_string(), port: 1020 };
    let v: Vec<Database> = vec![postgres, mysql];

    // Returning different variants per call
    fn i_return_db_impl(flag: bool) -> Database {
        if flag { 
            return Database::Postgres { host: "localhost".to_string() }; 
        } else {
            return Database::MySql { host: "localhost".to_string(), port: 1020 };
        }
    }


    // _____________________________________________________________________________________________
    // DYNAMIC DISPATCH

    // ⭐ How does dynamic dispatch solve the limitation of static dispatch? dyn Trait is a pointer to a concrete type, and since all pointers are of the same size in memory you can store them in a collection or return from func etc.
    // You essentially buy memory size determinism (and heterogeneity) with the price of pointer indirection.
    // ⭐ When you see dyn Trait (behind a pointer), think: sized pointers to concrete implementations; interleavable together; indirection is the cost.

    // dyn Trait -> means "Some type that implements Trait, unknowable at compile time, reachable only by pointer". 
    //              We know which concrete type only at runtime. We don't know its size (at compile time), so can't hold directly, can't put in a collection/struct directly.
    //              You can never have one as a bare value, because its 'unsized', meaning we (the compiler) don't know it's size (it could be Cat, could be Dog, so how much memory should we allocate for the variable?).
    //              You can only have it behind a pointer (&, &mut, Box, Rc, Arc) , because the pointer is of a fixed size. 
    // &          - read pointer
    // &mut       - mutable pointer
    // Box<...>   - box pointer
    // Rc<...>    - reference counted pointer
    // Arc<...>   - atomic reference counted pointer

    // let a: dyn Animal = Cat;                // ❌ can't hold directly or put in a collection or struct directly
    let a: &dyn Animal = &Cat;                 // ✅ &dyn           - read pointer
    let a: &mut dyn Animal = &mut Cat;         // ✅ &mut dyn       - mutable pointer
    let a: Box<dyn Animal> = Box::new(Cat);    // ✅ Box<dyn ...>   - box pointer
    let a: Rc<dyn Animal> = Rc::new(Cat);      // ✅ Rc<dyn ...>    - reference counted pointer
    let a: Arc<dyn Animal> = Arc::new(Cat);    // ✅ Arc<dyn ...>   - atomic reference counted pointer

    // Automatic coercion
    // PTRvalue -> coerces to PTRdyn Trait automatically when value implements Trait

    // Overcoming generics limitation 1: Heterogeneous collections
    let v: Vec<&dyn Animal> = vec![&Cat, &Dog];
    let v: Vec<&mut dyn Animal> = vec![&mut Cat, &mut Dog];
    let v: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];

    // Overcoming generics limitation 2: Returning different trait implementations per call
    fn i_return_db_impl2(flag: bool) -> Box<dyn Animal> {
        if flag { 
            return Box::new(Cat); 
        } else {
            return Box::new(Dog);     // ✅ now possible
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
    let a: Box<dyn Animal + Send + Sync> = Box::new(Cat);    // works because Send and Sync are auto-traits

    
    // IMPL TRAIT (Static dispatch)
    // impl Trait <- means "one specific concrete type that the compiler fully knows", you're just aren't writing its name. 
    // Monomorphized, fully Sized, no pointers, no indirection, duplicated concrete implementation per type.


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
    // A + B + C SYNTAX - BOTH STATIC AND DYNAMIC DISPATCH
    // Means the thing must implement all the traits specified. 

    // Static dispatch:
    // For static dispatch: A, B, C can be normal traits
    fn i_take_thing<T: Animal + Send + Sync>(a: T) {}         // Static dispatch, generic bounds syntax
    fn i_take_thing2(a: impl Animal + Send + Sync) {}         // Static dispatch, anonymous generic syntax

    // Dynamic dispatch
    // For dynamic dispatch: First trait has to a normal trait, and the rest (everything after +) have to be auto-traits or lifetimes. 
    fn i_take_thing3(a: Box<dyn Animal + Send + Sync>) {}     // Dynamic dispatch. Only works because Send and Sync are auto-traits

    
    // _____________________________________________________________________________________________
    /* 
    
    SUMMARY

    Decision tree:
    Is the type known at compile time?
      Yes -> Static dispatch
      No -> Do I know every type when I write the code (closed set)?
            Yes -> Enum dispatch
            No -> dynamic dispatch
    
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

    SIZE AS THE KEY CONCEPT: every value's size + layout must be known at compile time (Sized). Incl function return types and collection elements.
    So to be polymorphic AT ALL under that constraint, pick one of three:
      ├─ MONOMORPHISATION  - a type-specific copy per type        → static dispatch
      │    hardcoded callee, inlinable, zero cost
      │    but: a copy serves ONE type, so it can't serve a mix at runtime
      ├─ INDIRECTION       - pointer + vtable; all ptrs same size → dynamic dispatch
      │    one copy, type decided at runtime, pointer chase, no inlining
      └─ UNION OF KNOWN SIZE - enum, sized as its largest variant → enum dispatch
           no indirection, closed set, wastes space to the largest variant
    
    */
    // _____________________________________________________________________________________________

}

trait Animal {
    fn sound(&self);
}

struct Cat;
impl Animal for Cat { 
    fn sound(&self) { println!("Meow!"); } 
}

struct Dog;
impl Animal for Dog { 
    fn sound(&self) { println!("Woof!"); } 
}

struct Forest<T: Animal> {
    animal: T
}
