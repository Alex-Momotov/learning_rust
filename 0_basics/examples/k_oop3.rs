#![allow(unused)]

fn main() {

    // _____________________________________________________________________________________________
    // TYPE
    // Creates an alias for a type that already exists. 
    type Km = i32;          // type ALIAS = TYPE; 
    let distance: Km = 10;

    // Reasons why it exists
    // 1. Shorten a verbose type you'd otherwise be repeating everywhere
    type Thunk = Box<dyn Fn(i32) -> i32 + Send>;

    // 2. One place to change - pin the type once and every signature is short (e.g. error types)    
    type Result<T> = std::result::Result<T, String>;        // This is literally what `std::io::Result<T>` is.

    // --------------------------
    
    // Inside a trait 'type' declares an associated type.
    trait Something { type Item; }

    // Inside impl Trait, 'type' implements the associated type
    impl Something for String { type Item = i32; }

    
    // _____________________________________________________________________________________________
    // ASSOCIATED TYPES
    // Associated type is declared 'type x;' inside a trait and allows the trait to refer to it as a type param using 'Self::x' everywhere, and for the implementer to choose what type it is.
    // So similar to a trait with a generic param, but the difference is that the implementer pins down the associated type once, and so end users don't have to specify the generic param everywhere.

    // Choosing between associated type and generic param for a trait
    //   - Can one type meaninfgully implement this trait in more than one way?   -> generic parameter.
    //   - Is the type uniquely pinned down by the implementing type?             -> associated type.

    trait Container {
        type X;                      // Declare associated type
        fn get(self) -> Self::X;     // referred to as Self::X 
    }

    // Implementer chooses the type once
    struct Drawer(i32);
    impl Container for Drawer {
        type X = i32;
        fn get(self) -> i32 { self.0 }      // Note: once we choose the associated type in 'type X = i32' we refer to it as i32 throughput
    }

    // The benefit is that the user doesn't have to specify the type everywhere like with the generic param that has multiple impls.
    let drawer = Drawer(5);
    let i = drawer.get();

    // Constraining in a bound
    // Means we specify a bound as a trait and specify it's associated type
    fn something(c: impl Container<X = i32>) {
        let i: i32 = c.get();   
    }

    // _____________________________________________________________________________________________
    


}