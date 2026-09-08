#![allow(unused)]
use std::{collections::HashMap, fs::File};
use std::thread;


/*
ITEMS
- Items are: 
    - mod, use, fn, type, const, static, enum, struct, impl, trait
- Properties of items:
    - Legal in any block — Can be defined anywhere, in any scope. And defining an item inside a fn scopes it to that fn.
    - Oder-independent - items are all visible to each other within that scope/file regardless of where they appear in the scope/file.
    - No captured environment — a nested `fn` can't see the enclosing fn's locals or generics.

COERCION
- Coercion is when the compiler implicitly converts a value of one type to another at specific points in the code - similar to auto-boxing in Java.
- Rust is strict about types so the list of coercions is deliberately narrow:
    - Deref coercion -  &String → &str       &Vec<T> → &[T]       &Box<T> → &T
      If you have &T and need &U, and T: Deref<Target = U>, the compiler inserts the conversion automatically.
      This is why you can pass a &String to a function expecting &str without calling .as_str().
    - Unsized coercion: &[T; N] → &[T] (array to slice), or &ConcreteType → &dyn Trait (turning a concrete type into a trait object).
    - Weakening: &mut coerces to & - Passing a &mut T where &T is wanted works
- Where coercion happens: function call arguments, let bindings with explicit type.
    
*/

#[tokio::main]
async fn main() {
    // _____________________________________________________________________________________________
    // PROPERTIES OF ITEMS

    // Legal in any block
    // Functions, just like other items can be defined anywhere incl inside another function. 
    fn outer() {
        fn inner() {}
    }
    
    // Order independent
    // Functions, just like other items can be used before they are defined.
    helper();   // calling it before defining it
    fn helper() {}

    // No captured environment
    // Functions, just like other items cannot capture environment from the enclosing scope
    let a = 1;
    fn hi() {
        // a;   ❌ can't access enclosing scope's variables
    }
    
    // _____________________________________________________________________________________________
    // WHAT A FUNCTION CAN ACCESS
    // Can access:   - Parameters
    //               - Other items (privacy permitting) - fn, const, static, type, trait, mod, enum, struct, impl
    // Can't access: - Locals of an enclosing function

    const CON: i32 = 5;
    static STAT: i32 = 5;
    fn outer_fn() {}
    struct Stru {a: i32}
    let outer_var = Stru{a: 1};

    fn what_can_i_access(my_param: i32) {
        // items - constants and statics 
        CON;                // ✅ constant 
        STAT;               // ✅ static

        // items - functions, structs, etc
        outer_fn();         // ✅ function 
        Stru{a: 1};         // ✅ struct

        // outer variables
        // outer_var;       // ❌ can't access outer variables
    }
    
    // _____________________________________________________________________________________________
    // PARAMETERS 
    // Each parameter must declare: type, borrowing (own, & - read, &mut - exclusive access)
    fn consumes(s: String) {}
    fn reads(s: &String) {}
    fn mutates(s: &mut String) {}

    // Calling a function - params must be passed with the same type and borrow level
    let mut s = String::from("hi");
    consumes(s.clone());    // String
    reads(&s);              // &String
    mutates(&mut s);        // &mut String

    // PARAMETERS AND OWNERSHIP
    // Signature choice is API design. Take by value when you genuinely need to store or transform-and-return the thing. Borrow otherwise.
    fn take(s: String) {}           // I'm taking it, you loose it      Caller looses: variable entirely
    fn read(s: &String) {}          // Lend me a read                   Caller looses: O, W
    fn mutate(s: &mut String) {}    // Lend me exclusive access         Caller looses: O, W, R
    fn copy(i: i32) {}              // Copy - caller keeps their own    Caller looses: nothing

    // PARAMETERS AND MUTABILITY
    // mut in a parameter is not part of the contract
    // mut n says "the local binding inside my body is mutable." The caller doesn't need mut, doesn't care, and can't observe it. 
    fn count(mut n: i32) -> i32 {   // caller sees `fn count(i32) -> i32`
        n += 1;
        n
    }
    
    // PARAMETERS AND COERCION
    fn take_str(s: &str) {}
    fn take_bytes(b: &[i32]) {}
    fn take_read_ref(a: &i32) {}
    
    take_str(&String::from("hi"));  // &String -> &str
    take_bytes(&vec![1, 2, 3]);     // &Vec<i32> -> &[T]
    take_read_ref(&mut 5);          // &mut -> &

    // PARAMETERS ARE PATTERNS
    // They are irrefutable, meaning they can bind and destructure, but can't test. Destructuring happens in the signature.

    // destructuring tuple in the signature:
    fn i_take_point1(x: (i32, i32)) {}                      // x is (i32, i32)
    fn i_take_point2((x1, y1): (i32, i32)) {}     // x1, y1 are i32

    // destructuring struct in the signature:
    struct Point { x: i32, y: i32 }
    fn i_take_point3(p: Point) {}                           // p is Point
    fn i_take_point4( Point{x, y}: Point ) {}     // x, y are i32

    // _____________________________________________________________________________________________
    // DEFAULT PARAMETERS, NAMED PARAMETERS, OVERLOADING
    // Rust does NOT have: default parameters, named parameters, function overloading. 
    // Instead: one name, one signature, all arguments are positional and required.

    // Separate constructors - the answer to overloading 
    Vec::<i32>::new();
    Vec::<i32>::with_capacity(10);

    // Option<T> parameters - the answer to default parameters
    fn example(opt: Option<&str>) {
        match opt {
            Some(s) => println!("got something {s}"),
            None          => println!("got nothing"),
        }
    }

    // Using struct as a bundle of parameters
    struct KafkaConsumerParams {
        host: String,
        port: usize,
        topic: String,
    }
    fn connect(consumer_params: KafkaConsumerParams) {}

    // normal use
    let consumer_params = KafkaConsumerParams{host: String::from(""), port: 1, topic: String::from("")};
    connect(consumer_params);

    // '..struct_value' means - specify only what differs
    let consumer_params = KafkaConsumerParams{host: String::from(""), port: 1, topic: String::from("")};
    connect(KafkaConsumerParams { host: "hi".to_string(), ..consumer_params});

    // Taking trait as param
    // Taking parameters that implement a trait - the function can use that trait's methods.
    fn greet(name: impl Into<String>) { let name = name.into(); }
    greet("bob");                 // &str
    greet(String::from("bob"));   // String
    
    // _____________________________________________________________________________________________
    // RETURN 
    // Return type (if function returns) must be declared with '-> type'.

    // No '->' makes the function return '()'
    fn nothing() {}
    let a = nothing();      // a is ()
    
    // Last expression is implicitly the return
    // Last expression in a function (no semicolon) is implicitly the return value (and is more idiomatic). Or you can use 'return' keyword (with semicolon).
    fn some_int() -> i32 {
        return 5;       // ❌ not idiomatic 
    }
    fn some_int2() -> i32 { 
        5               // ✅ idiomatic 
    }

    // Convention - use bare expression at the end for normal return path, and only use 'return' keywoard for early return in the middle of the function.
    fn classify(n: i32) -> &'static str {
        if n < 0 { return "negative"; }   // early return
        if n > 0 { return "positive"; }
        "zero"                            // normal path — end expression
    }

    // RETURNING MULTIPLE THINGS
    fn two_things() -> (i32, String) {
        (5, String::from("hi"))              // use tuple
    }
    let (a, b) = two_things();  // then destructure

    // The never return type !
    // A function that never returns is written -> !
    fn fatal() -> ! { std::process::exit(1); }  // exits process - possible because exit() returns !
    fn forever() -> ! { loop {} }               // loops forever - possible because loop returns !

    // RETURNING REFERENCES
    // You can't return a reference to a local because the it would outlive the data. So a returned reference must borrow from input. 
    // ⭐ The practical rule while learning: return owned data (String, Vec<T>) and don't fight it. Returning references is
    // a performance/API refinement, and it's the single most common place beginners hit lifetime annotations.

    // RETURNING TRAITS
    // '-> impl Trait' means the function returns SOME concrete type implementing this trait (doesn't tell you which).
    fn evens(v: &[i32]) -> impl Iterator<Item = &i32> {     // returns Iterator of i32
        v.iter().filter(|x| *x % 2 == 0)
    }

    // _____________________________________________________________________________________________
    // CONSTANT FUNCTIONS
    // Their result can initialise a const or array length. Callable at compile time. The body is restricted to a subset of the language.
    const fn const_fn() -> &'static str { "hi" }
    const A: &str = const_fn();

    // _____________________________________________________________________________________________
    // GENERIC FUNCTIONS AND TURBOFISH
    // Generic functions use type parameters so one function can work with many types.
    // The compiler usually infers the concrete type from the arguments, so you often don't write it explicitly.
    fn id<T>(x: T) -> T { x }
    let n = id(5);                      // T inferred as i32
    let s = id(String::from("hi"));  // T inferred as String

    // Generic bounds (traits) say what operations the type must support.
    fn print_it<T: std::fmt::Display>(x: T) {
        println!("{x}");
    }
    print_it(123);
    print_it("hello");

    // Multiple type parameters are allowed.
    fn pair<A, B>(a: A, b: B) -> (A, B) { (a, b) }
    let p = pair(1, "a");

    // TURBOFISH
    // '::<...>' lets you provide generic arguments explicitly when inference is not enough or when you want to be specific.
    fn stuff<T>(a: T) -> T {a}
    stuff::<i32>(1);
    
    let v = Vec::<i32>::new();   // tell Vec what element type to use
    let x = id::<i32>(5);             // explicitly choose T = i32

    // A common place for turbofish is parsing, because the target type may not be obvious.
    let parsed = "42".parse::<i32>().unwrap();

    // _____________________________________________________________________________________________
    // UNSAFE FUNCTIONS
    // Unsafe operations mean they have preconditions that compiler cannot verify, e.g. array size.
    // Unsafe operations can only be called from blocks and functions declared 'unsafe'. You must ensure the preconditions are there.

    unsafe fn first_unchecked(slice: &[i32]) -> &i32 {
        slice.get_unchecked(0)  // SAFETY: The caller guarantees `slice` is not empty, so index 0 exists.
    }

    let nums = vec![10, 20, 30];
    let first = unsafe {
        first_unchecked(&nums)
    };

    // _____________________________________________________________________________________________
    // ASYNC FUNCTIONS
    // They return impl Future. They are lazy - nothing happens until you do future.await.
    async fn example_async() { println!("hi"); }
    let f = example_async();
    f.await;
    
    // _____________________________________________________________________________________________
    // FUNCTION POINTERS AND CLOSURES - TYPE SYNTAX
    //    fn(A) -> B          Function pointer.     Function pointers and non-capturing closures (via coercion). Taking parameter, Storing.
    //    impl Fn(A) -> B     Closure.              Function pointers (via coercion) and closures. Taking parameter.
    //    &dyn CLOSURE                              Borrowed, closures. Taking parameter, Storing.
    //    Box<dyn CLOSURE>                          Owned closures. Taking parameter, Storing.
    
    // Taking as parameter:     If you need to take a function/closure as parameter: take closure and it will accept both. impl Fn(A) -> B.
    // Storing:                 Choices: 1. function pointer (works for functions and non-capturing closures). 2. &dyn CLOSURE. 3. Box<dyn CLOSURE>.
    // Capturing environment:   Function pointers can't capture environment, closures can.
    // Coercion:                Function pointer coerces to closure. Non-closure coerces coerce to function pointers.
    
    // _____________________________________________________________________________________________
    // FUNCTION POINTERS
    // It's a variable assigned to a a function itself - its just a pointer to a compiled function code. 
    // It's a concrete type - because it doesn't capture environment, so it can affort to be one (unline closures).
    fn func() -> i32 { 5 }
    let f = func; // store it in a variable
    f();                            // call it

    // Function pointers coerce to closures (Fn, MutFn, FnOnce)
    fn i_take_closure(cl: impl Fn() -> i32) {}    
    i_take_closure(func);             // ✅ accepts funcion pointer
    i_take_closure(|| 5);             // ✅ accepts closure 

    // You can store function pointer in a struct or array without using generics or Box<dyn Fn> - because it has a concrete type.
    fn func2() -> i32 { 5 }
    let arr = [func, func2];

    // _____________________________________________________________________________________________
    // CLOSURES
    // 1. Closures are expressions (can be assigned to variables).
    // 2. Closures (unlike functions) can capture environment - variables from outside.
    // Closures don't have a concrete type - their type is anonymous, unnamable, and dynamically generated by the compiler (because it captures variable).
    // As a consequence you cannot write the closure's type directly. Instead, their type is written as trait implemenetation: impl Fn(A, B) -> C

    // The syntax
    let hi = || println!("hi");                                                    // no params
    let add = |a: i32, b: i32| a + b;                               // the usual form - no return type, no braces required
    let add = |a: i32, b: i32| -> i32 { let c = a + b; c };    // braces are only required if body has more than one expression

    // Calling a closure - like a normal function
    add(1, 2);  
    hi();

    // Parameter types are inferred on first use - and they are permanently locked in after the first use
    let func = |x| x;
    let a = func(5);        // Inferred - func is now permanently |i32| -> i32
    // let b = f("hello");       // ❌ ERROR

    // Non-capturing closures coerce to function pointers
    let f: fn(i32) -> i32 = |x| x + 1;
    
    // ----------------------------------
    // CAPTURING VARIABLES
    // A closure can capture variables, to be used inside it's body later. That means it must either: & borrow, &mut borrow, or move the variable inside it, just like any other normal variable.
    // Capture type (& borrow, &mut borrow, or move) is determined automatically based on what the closue does with the variable inside it's body. And it affects what trait the closure becomes - Fn, FnMut, FnOnce.

    // Fn       Closure reads captured variable (or doesn't capture).  & borrow. Closure can be invoked many times.
    // FnMut    Closure mutates captured variable inside it's body.    &mut borrow. Closure itself must be mut. Closure can be invoked many times.
    // FnOnce   Closure moves captured variable inside it's body.      move. Closure can be invoked only once - because on second invokation the captured var will no longer exist (it's moved).

    let mut list = vec![1, 2, 3];
    let read = || println!("{list:?}");     // Fn       captures &list
    let mutate = || list.push(4);        // FnMut    captures &mut list         
    let own = || drop(list);            // FnOnce   captures list (moves it)


    // The variable capture starts when the closure is defined, and ends at closure's last use (just like a normal borrow).
    let mut list = vec![1, 2, 3];
    let mut mutate = || list.push(4);   // &mut borrow starts       Note: FnMut clouse must itself be mut in order to work.
    mutate();                                         // &mut borrow ends (last use)
    println!("{:?}", list);                           // list unlocks again
    
    
    // 'move' keyword forces closure to move the captured variable inside itself (overriding the inferrence). It doesn't affect the capture trait (Fn, FnMut, FnOnce) - which still depends on what the capture DOES with the variable.
    // Used when the closue is not inferred as FnOnce but must outlive the current scope. E.g. when: 1. Giving the closure to a thread; 2. Returning the closure; 3. Storing the closure in a struct.
    let data = vec![1, 2, 3];
    let handle = move || println!("{data:?}");  // data now owned into the closure

    // 1. spawn a thread
    thread::spawn(handle);                                  

    // 2. return closue with captured variable inside
    fn make_closure() -> impl FnOnce() -> Vec<i32> {        
        let data = vec![1, 2, 3];
        move || data
    }

    // ----------------------------------
    // TAKING CLOSURE AS PARAMETER
    fn i_take_closure2(f: impl Fn(i32) -> i32) {}   // default
    fn i_take_closure3(f: &dyn Fn(i32) -> i32) {}   // when you need dyn

    // RETURNING CLOSURE
    fn i_return_closure() -> impl Fn(i32) -> i32 { |x| x + 1 }                  // default
    fn i_return_closure2() -> Box<dyn Fn(i32) -> i32> { Box::from(|x| x + 1) }  // when you need dyn

    // Returning a closure that captures a local variable 
    // Must use 'move' to move the var inside closusure, so it doesn't borrow (references can't outlive data)
    fn i_return_closure3() -> impl Fn(i32) -> i32 {
        let n = 1;
        move |x| x + n
    }
    
    // STORING CLOSURE - IN A COLLECTION OR STRUCT
    // Can't use the bare impl Fn(A) -> B syntax. Your options: 
    let closures: Vec<fn(i32) -> i32>  = Vec::new();            // Non-capturing closure? Use function pointer
    let closures: Vec<Box<dyn Fn(i32) -> i32>>  = Vec::new();   // Capturing closure? Use Box<dyn> or &dyn
    
    // _____________________________________________________________________________________________
    

}



