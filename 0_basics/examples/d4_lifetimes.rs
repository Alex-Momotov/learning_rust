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
    // Also struct/enum methods which take &self or &mut self - if return is a reference it will automatically get the self's lifetime - no need to specify (unless this is a wrong assumption and then you specify explicitly)
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
    // Since struct fields cannot be null, struct's reference fields must stay valid references throughout the lifetime of a struct.
    // Therefore, we can think of structs containing references having "lifetimes" of their own, in a sense that a struct with references can't outlife the data that those references point to.

    struct Message<'a> {    // This struct's instance can't outlive data that it's references point to.
        data: &'a str
    }
    
    // If a struct has a lifetime param, its impl block must have a lifetime param too. Either:
    impl Message<'_> {                          // Anonymous lifetime - if you never refer to 'a inside the impl, write `impl Message<'_> { ... }` (anonymous lifetime).
        fn get(&self) -> &str { self.data }
    }
    
    impl<'a> Message<'a> {                      // Or full syntax (like generic)
        fn compare(&'a self, other: &'a str) -> &'a str { 
            if self.data.len() > other.len() { 
                return self.data 
            }
            other
        }
    }
    

    let s = "hi".to_string();
    let m = Message { data: &s };
    // drop(s);             // ❌ Error - This would mean the struct outlives the data that it's references point to
    m;


    // * Why isn't struct's "lifetime" automatically tied to the lifetime of all references inside it? Why do we have to specify it?
    //   Because there are edge cases where you can use a struct as a "passthrough" of a reference - the struct dies, but this function still returns a reference of what it was pointing to
    fn outlive<'a>(m: Message<'a>, s: &str) -> &'a str {
        let r = m.data;
        drop(m);
        r
    }

    // _____________________________________________________________________________________________
    // STATIC LIFETIMES
    // A static lifetime means a reference will life as long as the program. All string literals have a static lifetime because they are baked into the binary's static region.
    let s: &'static str = "hi";
    

    // _____________________________________________________________________________________________
    // GENERIC AND LIFETIME PARAMS TOGETHER
    // They are simply specified in the same <> separated by comma.
    
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


    // Note: Rust checks each function call using only its signature, never the body. The signature is a contract:
    // the caller's side is checked against the signature ("is r used after s1 is dropped?")
    // the body is checked against the signature separately ("does it actually return something borrowed from s1?")
    // So the lifetime annotation is there because the signature has to say which input the output borrows from.

    
    // Lifetime syntax - ...


    // _____________________________________________________________________________________________
    /* 
    [] Make sue you understand what this is about:
    
    Returning references — you can only hand back what you were lent
    
      You can't return a reference to a local; the frame dies. So a returned reference must borrow from an input, and
      lifetime elision decides which input, by three rules:
    
      1. Each elided input reference gets its own lifetime.
      2. If there's exactly one input lifetime, it's given to all outputs.
      3. If one of the inputs is &self/&mut self, self's lifetime is given to all outputs.
    
      fn first_word(s: &str) -> &str { .. }        // rule 2 — output borrows from s
      fn longest(a: &str, b: &str) -> &str { .. }  // ERROR: two inputs, ambiguous
      fn longest<'a>(a: &'a str, b: &'a str) -> &'a str { .. }  // must be explicit
    
      ⭐ The practical rule while learning: return owned data (String, Vec<T>) and don't fight it. Returning references is
      a performance/API refinement, and it's the single most common place beginners hit lifetime annotations.
    
    */
    // _____________________________________________________________________________________________
}