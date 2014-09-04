// Example 1: Hello World, functions, types.

// On with the show!

fn say_hi(x:u8) -> u8 {  /* A function signature */          /*
^  ^     ^      ^  ^                                          *
|  |     |      |  |                                          *
|  |     |      |  ^~ 8-bit `u`nsigned integer                *
|  |     |      |                                             *
|  |     |      ^~~~~~ arrow followed by return type          *
|  |     |                                                    *
|  |     ^~~~~~ parenthesis-delimited argument list           *
|  |                                                          *
|  ^~~~~~ function name                                       *
|                                                             *
^~ `fn` keyword marks start of a function item                *
                                                              *
                                                              */

    println!("Hello world from ex1"); /* a statement */      /*
    ^        ^                      ^~   terminated by `;`    *
    |        |                                                *
    |        ^~~~~~~~~~~~~~~~~~~~~~ string literal in ""      *
    |                                                         *
    ^~~~~~~~ The `println!` macro; all macro invocations      *
             are (currently) marked with a `!` at the end     *
             of their name. Other macros include              *
             `assert!`, `assert_eq!`, `debug!`, `fail!`       *
                                                              */

    return x+1;                                              /*
    ^~~~~~ Ahh; good old `return`                             *
                                                              */
}

fn call_say_hi() -> u8 {

      say_hi(103)                                            /*
      ^~~~~~~~~~~^~~ a call expression...                     *
                 |                                            *
                 ^~ ... but, something's different here ...   *
                                                              *
  ^~~  ... hint: what's "missing" over here                   *
                                                              */
}

fn call_say_hi_twice() -> u8 { say_hi(103); say_hi(103) }
//                Easy to write one-liners: ~~~~~~~~~~^

pub fn main() {
    // There are at least two "interesting" differences between the
    // signatures of `main` and the `say_hi` family above.  Discuss.  :)

    let x : u8 = call_say_hi();                              /*
    ^     ^~~~ a type annotation, just like in fn signatures  *
    |
    ^~~ local variable binding introduced by `let`.           *
                                                              */

    println!("`say_hi` returned {:u}", x);
    //                          ^    ^~~ a second argument
    //                          |
    //                          ^~~~ a hole for the argument (includes
    //                               the format-specifier `u`, meaning
    //                               "unsigned decimal", like %u in
    //                               printf).

    let x = call_say_hi_twice();

    println!("Repeat: it returned {}", x);
    //                            ^~ (but unlike printf, Rust has a
    //                                general-purpose default that
    //                                types can also opt into.)

    let c;
    //   ^~ can leave off the initial value for later assignment.
    //
    //      Also letting the type be inferred; Rust allows this for
    //      local variable declarations, but not for `fn` signatures.

    c = x as char;
    //    ^~~~~~~ casting syntax, read like `(char)x` in C/C++/Java.

    println!("x (`{}`) as a char is `{}`", x, c);
}

// EXERCISE: Change all of the above fn's to take and return `u32`
// instead of `u8`.
//
// What happens? Why? How can we address it?
