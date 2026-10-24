use std::fmt::Display;


fn main() {
    // _____________________________________________________________________________________________
    // LIFETIMES

    // Lifetime - All references have a lifetime - a time period of how long a reference is valid for. 
    // This period must not be longer than the data the reference is pointing to, otherwise its a compile error.
    // String    ██████████████████████████████████████████
    //           create                                drop
    //                         Lifetime of &str
    // &str              ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
    //                   borrow            last use

    // Lifetime parameters (<'a>) - describe relationship between lifetimes of references. 
    // They describe how the lifetimes of references relate to each other (tied together or kept independent) whenever there is an ambiguity (compiler can't infer it).
    // Think of specifying lifetime parameters as the act of tying together lifetimes of different references to help compiler.
    // Also, you can think of lifetime params as a tree - declare them at functions and type definitions -> then the actual usage and "tying together" flows down to function returns and type fields.
    
    // Lifetime elision - whenever there is only one reference as input then all output references get its lifetime (automatically inferred by compiler), so no need to specify lifetime params
    // If one of the inputs is &self/&mut self, self's lifetime is given to all outputs by default (unless otherwise specified).
    fn lifetime_elision(s: &str) -> &str { s }                          // 1 ref input, 1 ref output
    fn lifetime_elision2(s: &str) -> (&str, &str) { (s, &s[0..1]) }     // 1 ref input, 2 ref output
    fn words(s: &str) -> Vec<&str> { s.split_whitespace().collect() }   // 1 ref input, Vec of refs output - all &str in Vec borrow from s


    // _____________________________________________________________________________________________
    // LIFETIME PARAM - FUNCTIONS
    // The lifetime of a return value reference must always be tied to the lifetime of one of the parameter references (implicitly if one param (lifetime elision), explicitly if multiple params (litetime params)).
    // Without lifetime specifiers the examples below will not compile, because lets say Runtime info determins whether String a outlives String b. 
    // Since references can't outlive their data, compiler needs to know whether to tie the lifetime of return &str to String a or String b - so we need to tell it which one, by adding a lifetime specifier: 
    fn first<'a>(s1: &'a str, s2: &str) -> &'a str {  // tie lifetime of return reference to lifetime of String s1. Meaning compiler can now enforce that return ref must not outlive String s1
        s1
    }

    fn second<'a>(s1: &str, s2: &'a str) -> &'a str {  // tie lifetime of return reference to lifetime of String s2. Meaning compiler can now enforce that return ref must not outlive String s2
        s2
    }

    fn both<'a>(a: &'a str, b: &'a str) -> &'a str { // tie lifetime of return reference to lifetimes of both String s1 and s2. Meaning compiler can now enforce that return ref must not outlive String s1 and s2 
        if a.len() > b.len() {
            return a
        }
        return b
    }

    let s1 = "hi".to_string();
    let s2 = "hi".to_string();
    let r = first(&s1, &s2);    // Lifetime of reference r is tied to String s1
    // drop(s1);                      // ❌ Error - Compiler can now successfully enforce that the lifetime of r reference must not outlife String s1.
    println!("{:?}", r);

    let s1 = "hi".to_string();
    let s2 = "hi".to_string();
    let r = second(&s1, &s2);     // Lifetime of reference r is tied to String s2
    // drop(s2);                        // ❌ Error - Compiler can now successfully enforce that the lifetime of r reference must not outlife String s2.
    println!("{:?}", r);

    let s1 = "hi".to_string();
    let s2 = "hi".to_string();
    let r = both(&s1, &s2);       // Lifetime of r is tied to both String s1 and s2
    // drop(s1);                        // ❌ Error - Compiler can now successfully enforce that the lifetime of r reference must not outlife String s1.
    // drop(s2);                        // ❌ Error - Compiler can now successfully enforce that the lifetime of r reference must not outlife String s2.
    println!("{:?}", r);

    
    // _____________________________________________________________________________________________
    // LIFETIME PARAM - STRUCTS
    // A struct can't outlive the data it borrows. Since struct fields cannot be null, struct's reference fields must stay valid references throughout the lifetime of a struct.
    // Therefore, we can think of structs containing references having "lifetimes" of their own, in a sense that a struct with references can't outlife the data that those references point to.

    struct Record<'a> {    // This struct's instance can't outlive data that it's references point to.
        data: &'a str
    }
    
    // If a struct has a lifetime param, its impl block must have a lifetime param too. Either:
    impl Record<'_> {}     // Anonymous lifetime - if you never refer to 'a inside the impl, write `impl Record<'_> { ... }` (anonymous lifetime).
    impl<'a> Record<'a> {  // Full syntax (like generic)
        fn get(&self) -> &str { self.data }             // tied to &self (elision)              Returned reference can't outlive struct itself
        fn get_raw(&self) -> &'a str { self.data }      // tied to underlying String buffer     Returned reference can't outlive underlying String buffer
        fn compare(&'a self, other: &'a str) -> &'a str { 
            other
        }
    }
    
    let s = "hi".to_string();
    let rec = Record { data: &s };
    // drop(s);       // ❌ Error - This would mean the struct outlives the data that it borrows
    rec;


    // * Why isn't struct's "lifetime" automatically tied to the lifetime of all references inside it? Why do we have to specify it?
    //   Because there are edge cases where you can use a struct as a "passthrough" of a reference - the struct dies, but this function still returns a reference of what it was pointing to
    fn outlive<'a>(m: Record<'a>, s: &str) -> &'a str {
        let r = m.data;
        drop(m);
        r
    }

    // _____________________________________________________________________________________________
    // STATIC LIFETIMES
    // &'static str and T: 'static share one idea, "valid until the end of the program"
    //   - &'static str: this reference stays valid forever.
    //   - T: 'static: a value of type T could be kept forever (if needed) without anything inside it going stale.
    
    // 1. 'static as reference lifetime parameter -  means a reference will life as long as the program. All string literals have a static lifetime because they are baked into the binary's static region.
    let s: &'static str = "hi";
    
    // 2. 'static as a generic bound - T must not contain any references that could expire - it either owns its data or only borrows 'static data (e.g. string literals). 
    // Put another way, it's always safe to keep a T for as long as you like. It does NOT mean the T value lives forever.
    fn foo<T: 'static>(t: T) {}

    struct Owning(String);          // Fine - T owns everything it contains
    foo(Owning("hi".to_string()));
    
    struct Borrowing<'a>(&'a str);
    let s1 = "hi".to_string();
    // foo(Borrowing(&s1));         // ❌ Error - T borrows something that could expire

    foo(Borrowing("hi"));           // Fine - T borrows data that lives forever

    
    // _____________________________________________________________________________________________
    // GENERIC AND LIFETIME PARAMS TOGETHER
    // They are simply specified in the same brackets <>, lifetimes first, then generics, separated by comma.
    
    fn longest_with_announcement<'a, T: Display>(s1: &'a str, s2: &'a str, announcement: T) -> &'a str {
        let longest = if s1.len() > s2.len() {
            s1
        } else {
            s2
        };

        println!("{} {}", announcement, longest);
        longest
    }
    

    // _____________________________________________________________________________________________
    
}