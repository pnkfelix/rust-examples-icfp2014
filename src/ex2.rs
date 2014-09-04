// Example 2: Structures

// A struct is a named record with fields - like struct in C or class in Java.
struct Thing {
    // `field: type` syntax (as with fn signatures, the types are manadatory)
    label: char,
    count: i32,
    //     ^~~ 32-bit signed `i`nteger.
}

pub fn main() {
    let i = 5;

    let t = Thing { label: 'a', count: i };
    //      ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ struct creation expression

    // Note that `t` is *stack-allocated*, just like `i` is above;
    // just like C/C++.
    //
    // (Compare to e.g. Java/Scheme/ML, where a record structures are
    // heap-allocated and your variables can solely hold *references*
    // to them.)

    print_thing(t); // XXX (see exercise 2 below)
}

fn print_thing(x: Thing) -> i32 {
    //         ^~~~~~~~ again, `x` is stack-allocated locally here.
    //                  Thus this thing has been *copied* from `main`
    //                  to here.

    // Field access uses the dot operator, just like C/Java.

    println!("the count of {:d} is {:d}", x.label, x.count);
    //                     ^~~~    ^~~~
    // Arguments must match the holes (in number, and specifier if present).

    // (returning x.count for no real reason)
    x.count
}

// EXERCISE 1: above is broken; compiler complains; fix it somehow.

// EXERCISE 2: try deleting the semi-colon on the line marked XXX
// above; what changes?
