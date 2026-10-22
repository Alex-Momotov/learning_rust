    // Drawing parallels between collections/strings (and their equivalent slices) and dyn Trait:

    // Collections / strings - Runtime information determines the SIZE OF THE THING 
    // 1. either through dynamically allocating a region of memory using raw pointers (size of buffer is runtime info)
    fn some_slice_from_dynamic_arr(len: usize) -> Box<[i32]> {
        let v = Vec::with_capacity(len);
        v.into_boxed_slice()
    }

    // 2. or branching on a runtime info flag and creating two static collections / arrays
    fn some_slice_from_static_arr(flag: bool) -> Box<[i32]> {
        if flag {
            let arr1: [i32; 3] = [1, 2, 3];
            Box::from(arr1)
        } else {
            let arr2: [i32; 2] = [1, 2];
            Box::from(arr2)
        }
    }

    // Dyn Trait
    // Runtime information determines which TYPE of the thing
    // There's no dyn equivalent of case 1 because dyn never involves a genuinely runtime-determined size
    fn create_cat_or_dog(flag: bool) -> Box<dyn Animal> {
        if flag {
            let c = Cat;
            Box::from(c)
        } else {
            let d = Dog;
            Box::from(d)
        }
    }

    /* 
    ❯ so as a more generally stated principle can I say that:
      dyn Trait representation is a "collapse" of one or more possible "places/variables" compiled statically but
      branch-determined at runtime? And that the "compiled statically" is always the case here - an invariant. So in
      other words dyn Trait can't represent a thing of a size determined at runtime, only a thing of a size known at
      compile time but CHOSEN at runtime, and that's the assymetry between slice and dyn trait
    
    ⏺ The core of it is right, and "known at compile time but chosen at runtime" is the precise formulation. Two
      refinements to the wording:
    */


















    
