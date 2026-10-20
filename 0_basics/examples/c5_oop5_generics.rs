#![allow(unused)]

fn main() {
    // _____________________________________________________________________________________________
    // GENERICS
    // How to think of generic code:
    // Structure (with some parts generic, some concrete "utils") and behaviours that can operate on that structure 
    // using other parametrised types (vectors, collections, containers) and methods from generic bounds.

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



    // GENERIC TRAIT
    // ...

    // Choosing between associated type and generic param for a trait
    //   - Is the type uniquely pinned down by the implementing type?             -> associated type.
    //   - Can one type meaninfgully implement this trait in more than one way?   -> generic parameter.



    
    
    
    // _____________________________________________________________________________________________
}