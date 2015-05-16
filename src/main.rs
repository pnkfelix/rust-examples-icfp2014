extern crate fsk_examples as fsk;

macro_rules! run {
    ($i:ident) => {
        {
            println!("fsk::{}", stringify!($i));
            fsk::$i::main();
        }
    }
}

pub fn main() {
    run!(ex1);
    // run!(ex2); // broken by design
    run!(ex3);
    run!(ex4);
    run!(ex5);
    run!(ex6);
    // run!(ex7); // broken by design
    run!(ex8);
    run!(ex9);
    run!(ex10);
    // run!(ex11); // broken by design
    // run!(ex12); // broken by design
    // run!(ex13); // broken by design
}
