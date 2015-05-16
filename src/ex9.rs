// Example 9: Vec, slices (and some fun with literals)

pub fn main() {
    let mut v = Vec::new();
    //          ^~~~~~~~ `Vec` is a generic dynamically growable vector.
    //                   Such an ubiquitous library type, it is
    //                   included in standard prelude.

    v.push(1_u64);
    //      ^~~~ Note: I don't *have* to have this type specifier right now;
    //           They are usually inferred.  (I have put it in to ease an
    //           exercise below.)

    println!("first call");
    print_contents(&v);

    //       In general you can put underscores anywhere in a numeric literal
    //       which is super useful for things like this:
    v.push(0xbadc_0deb_ebad);

    //       or this:
    v.push(0b1111_0111_0011_0001_0000);

    println!("second call");
    print_contents(&v);

    fourth_exercise();
}

fn print_contents(v: &Vec<u64>) {
    //               ^~~~~~~~~ see first exercise below
    for i in 0..v.len() {
        let elem = &v[i];
        println!("v[{}]: {:16x}", i, elem);
        //                           ^~~~ XXX (see third exercise below)
    }
}

// EXERCISE 1: Try removing the `&` from `&Vec<u64>` in the signature of `print_contents`
// and from the call sites.  What happens?

// EXERCISE 2: Try generalizing `print_contents` so that it takes "any" kind of `Vec<X>`.
// (hint i. think about ex8.)
// (hint ii. You may also need to use `use` to import something, similar to ex7::print_sizes.)
//
// NOTE: You do not need to complete this exercise to move on to the others below.

// EXERCISE 4: Uncomment code in fourth_exercise below.  What goes wrong?  How could
// you fix this?  (Hint: review ex7.)

fn fourth_exercise() {
    let _what_about_me = [0xfab_u64, 0xeba, 0xb0b];
    // print_contents(_what_about_me);
}
