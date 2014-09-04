// Example 10: Iterators!

use std::iter;

pub fn main() {
    // Felix, a Schemer at heart, likes unnecessary_parens
    #![allow(unnecessary_parens)]

    println!("We've seen this before");
    for i in range(0u32, 10) {
        println!("i: {}", i);
    }

    println!("But how about this:");
    for j in range(0u32, 100).filter(|k| k % 15 == 1) {
        println!("j: {}", j);
    }

    println!("Or this:");
    let v : Vec<Vec<uint>> = (iter::range_inclusive(1, 5)
                              .map(|i| Vec::from_elem(i, i))
                              .inspect(|v| println!("  iteration is seeing {}", v))
                              .collect());

    for (i, elem_a) in v.iter().enumerate() {
        println!("I didn't take elem[{:u}] == {}", i, elem_a);
        //                           exercise 1 below ^~~~~~
    }

    for (i, elem_b) in v.move_iter().enumerate().filter(|&(j, _)| j % 2 == 0) {
        println!("took elem[{:u}] == {}", i, elem_b);
    }
}

// EXERCISE 1: What type have `elem_a`/`elem_b` above?
//
// HINT: If unsure, add new binding with a type ascription and see if
// type checker accepts it.

// EXERCISE 2: Figure out how to iterate over the characters
// (codepoints) in a string literal.

// EXERCISE 3: Figure out how to iterate over the space-separated
// words in a string literal.
//
// HINT: Play with methods here: http://doc.rust-lang.org/std/str/primitive.str.html
