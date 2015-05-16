// Example 5: Methods.

struct Thing {
    label: char,
    count: i32,
}

// Example method definitions
impl Thing {                                                 /*
^~~~ The `impl` keyword introduces a collection of method     *
     implementations for a type.                              *
                                                              */

    fn get_count(&self) -> i32 { self.count }
    //           ^~~~~ `self` parameter is special; doesn't
    //                  need a type ascription (since that comes
    //                  from the `impl` block), but you still need
    //                  to indicate how it is being passed in).

    fn type_method() -> i32 { 14 }
    // ^~~~~~~~~~~~~ no `self` parameter means that the method
    //               is associated with the *type*, rather than
    //               with particular instances.
    //               (Often calls a "static" method, though the syntax
    //                here does not use that keyword)
}

// EXERCISE:
// add a `new` method for consructing of a `Thing`

// EXERCISE:
// port earlier `increment_count` from ex4 to an instance method on `Thing`

pub fn main() {
    let t1 = Thing { label: 'a', count: 5 };
    println!("{} get_count: {}, type_method: {}",
             t1.label, t1.get_count(), Thing::type_method());
}
