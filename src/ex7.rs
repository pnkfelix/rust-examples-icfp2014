// Example 7: loops, fixed length arrays, and slices.

pub fn main() {
    let some_nums : [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    //              ^           ^~~~~~~~~~~~~~~~~~~~~~~~~ an array literal
    //              |
    //              ^~~~~~~~~~ fixed-length array type

    print_via_for(some_nums);

    print_via_loop(some_nums);

    print_via_while(&some_nums[..]);

    let short : [i32; 4] = [1,2,3,4];

    print_via_while(&short[..]);

    // print_via_loop(short); // XXX (see exercise below)

}

fn print_via_for(some_nums: [i32; 9]) {
    for k in 0..9 {
        // We will see more about `for` loops in exercise TODO
        println!("for, some_nums[k]: {}", some_nums[k]);
    }
}

fn print_via_loop(some_nums: [i32; 9]) {
    let mut i = 0;
    loop {
        println!("loop, some_nums[i]: {}", some_nums[i]);
        i += 1;
        if i > some_nums.len() { break; }
    }
}


fn print_via_while(some_nums: &[i32]) {
    // This is very different ~~~~~^
    let mut j = 0;
    while j < some_nums.len() {
        println!("while, some_nums[j]: {}", some_nums[j]);
        j += 1;
    }
}

// EXERCISE: Code above has (trivial) bug, so you never see
// `print_via_while` get called. Identify problem; fix in some way.

// EXERCISE: Uncomment the `print_via_loop` call that is commented out
// above. It does not work, while `print_via_while` does.  Why?
// (Hint: it has nothing to do with `while` vs `loop`.)
//
// Figure out how to adapt `print_via_loop` accordingly.

// EXERCISE: Code above uses `as_slice()` to do certain conversions.
// Explore swapping in `slice_to` and `slice_from` instead; reverse
// engineer their semantics.

// EXERCSE: Add a call from `main` to the `print_sizes` method below.
// Discuss.

fn print_sizes() {
    #![allow(dead_code)]
    use std::mem;

    println!(" size_of::<&[i32; 9]>() == {:2} bytes", mem::size_of::<&[i32; 9]>());
    println!("      size_of::<&[i32]>() == {:2} bytes", mem::size_of::<&[i32]>());
}
