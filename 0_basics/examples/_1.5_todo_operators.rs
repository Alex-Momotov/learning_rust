/*
Operators & numbers
- NO mixed-type arithmetic: i32 + f64 or i32 + u8 won't compile - cast explicitly with 'as'.
- NO ** operator -> .pow() / .powf(). NO ++/-- -> use += 1.
- integer overflow: panics in debug builds, silently wraps in --release. (checked_add etc. exist)
- % remainder keeps the sign of the LEFT operand (like C, UNLIKE Python): -5 % 3 == -2.
*/

#![allow(unused)]

pub fn main() {
	// Arithmetic ------------------------------
	let a = 5 + 2;
	let a = 5 - 2;
	let a = 5 * 2;
	let a = 5 / 2;          // integer / integer = integer division -> 2 (truncates toward zero)
	let a = 5.0 / 2.0;      // float division -> 2.5
	let a = 5 % 2;          // remainder -> 1
	let a = -5 % 3;         // -> -2 !! sign follows dividend, unlike Python where -5 % 3 == 1
	let a = (-5i32).rem_euclid(3);  // -> 1, this is Python's % behavior

	// no ** operator:
	let a = 2i32.pow(10);       // integer power -> 1024
	let a = 2.0f64.powi(10);    // float to integer power
	let a = 2.0f64.powf(0.5);   // float to float power
	let a = 2.0f64.sqrt();

	// Incrementing ------------------------------
	let mut n = 5;
	n += 1;                 // also -= *= /= %=  (no ++ or --)

	// Casting - 'as' ------------------------------
	// mixed types don't compile: 5 + 2.5 or 5i32 + 5i64 -> error
	let a = 5 as f64 + 2.5;         // int -> float
	let a = 2.9 as i32;             // -> 2, truncates toward zero (not rounding!)
	let a = 2.9f64.round() as i32;  // -> 3
	let n = 300;
	let a = n as u8;                // -> 44 !! out-of-range cast wraps silently, be careful
	// (with a literal, `300 as u8` is even a compile error - the compiler catches what it can see)
	let a = i32::MAX;               // every int type has MIN / MAX

	// Useful number methods ------------------------------
	let a = (-5i32).abs();
	let a = 5.max(2);               // -> 5
	let a = 5.min(2);               // -> 2
	let a = 2.567f64.round();       // also .floor() .ceil() .trunc()

	// Division edge cases ------------------------------
	// 5 / 0        -> integer division by zero PANICS (compile error if literal)
	let a = 5.0 / 0.0;              // -> inf (floats follow IEEE: inf, -inf, NaN)
	let nan = f64::NAN;
	let e = nan == nan;             // -> false! NaN equals nothing. use .is_nan()

	// Comparison ------------------------------
	let e = 5 == 2;                 // == != < > <= >=
	let e = 5 != 2;
	// different types don't compare: 5i32 == 5u8 won't compile -> cast first

	// Boolean ------------------------------
	let e = true && false;          // AND (short-circuits)
	let e = true || false;          // OR  (short-circuits)
	let e = !true;                  // NOT
	// no 'and/or/not' keywords, no truthiness (if 1 {} doesn't compile)

	// no 'in' operator - each type has a contains method:
	let e = "cat".contains('c');
	let e = vec![1, 3, 5].contains(&5);

	// Bitwise (exist, same as C) ------------------------------
	let a = 0b1010 & 0b0110;        // also | ^ << >>

	// String <-> number ------------------------------
	let n: i32 = "42".parse().unwrap();     // parse returns Result - .unwrap() = "or crash" (proper handling: ch 9)
	let n = "42".parse::<i32>().unwrap();   // same, turbofish style
	let s = 42.to_string();
}
