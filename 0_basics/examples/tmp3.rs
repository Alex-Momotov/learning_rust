#![allow(unused)]

fn main() {


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
    let arr = NumberArray { array: [1, 2, 3, 4, 5], i: 0};

    // iterating with a loop
    for i in arr {
        println!("{:?}", i);
    }

    // let out: Vec<i32> = arr.into_iter().map(|i| i * i).filter(|i| i % 2 == 0).collect();

    












    // _____________________________________________________________________________________________
    // 1. Footgun of modifying a copy when you think you're modifying the original happens when you do let b = a; b+= 1; but if you do a += 1; or let b = &mut a; *b += 1; its fine
    // 2. &mut itself is a move type (its a reference), so doing let b = &mut a; let c = b; makes b dead and moves the reference to c.
    //    & is a copy type, so doing let b = &a; let c = b; does not invalidate b. Both b and c are alive read references.
    // 3.   
    
    // let mut a = 10;

    // let m = &mut a;

    // let mut k = *m;
    // k += 1;

    // println!("{:?}", a);
}
