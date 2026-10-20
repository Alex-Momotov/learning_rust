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