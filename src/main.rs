#![feature(macro_rules)]

extern crate "fsk-examples" as fsk;

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
    run!(ex2);
    run!(ex3);
    run!(ex4);
    run!(ex5);
    run!(ex6);
}
