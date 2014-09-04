/*
^~ First things first: Delimited comment syntax
*/

// ... and many people prefer end-of-line terminated style.
// Like C++, we offer both.  (Though Rust's /* */ can nest.)
                                                            /*
^                 I will use slash-star style occasionally,  *
|                 e.g. when I want to point out something in *
|                 the left-most column...                    *
^~ ... like this                                             *
                                                             */

//   The style is *solely* for expository purposes for today  ^
//  I have never seen any production code that actually puts  |
//                   stars on the right-hand-side like this ~~^

// A quick word of warning: some comment forms are special.
// Namely `///`, `//!`, `/** */`, and `/*! */`; they turn
// into documentation blocks that try to attach themselves
// to some piece of code.  We won't be using these today.

// Anyway, on with the show!
