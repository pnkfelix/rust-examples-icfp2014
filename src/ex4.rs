// Example 4: Ownership and references

struct Thing {
    label: char,
    count: i32,
}

fn print_thing_val(x: Thing) {
    // (This is the same code we were looking at before.)
    println!("the count of {:c} is {:d}", x.label, x.count);
}

fn print_thing_ref(x: &Thing) {
    //                ^~ this is a "reference type constructor"
    //                   `&T` is pronounced "(shared) reference to T"
    println!("the count of {:c} is {:d}", x.label, x.count);
}

fn print_thing_box(x: Box<Thing>) {
    //                      ^~~ This is another type constructor
    //                          (One of many library-provided "smart-pointers")
    //                          `Box<T>` is pronounced "boxed T"
    println!("the count of {:c} is {:d}", x.label, x.count);
}

pub fn main() {
    // stack-allocated, as before
    let t1 = Thing { label: 'a', count: 5 };

    // These two calls have the same side-effects ...
    print_thing_val(t1);
    print_thing_ref(&t1);
    // ... but they do their work in very diferrent ways.

    // *heap*-allocated Thing
    let t2 = box Thing { label: 'b', count: 4 };
    print_thing_box(t2);

    // EXERCISE: Add code below here to print both `t1` and `t2`
    // again.  Discuss.
}


// Box<Thing> behaves very differently from `Thing` and `&Thing`.
//
// A detail we've been avoiding due to something special with `Thing`.
//
// But it is *not* something special about `Box`; it is *Thing* that
// is special; discuss.


// EXERCISE: add a new function, `increment_count`, that takes a
// `Thing` and adds 1 to its count field in a way that the *caller*
// can observe.
