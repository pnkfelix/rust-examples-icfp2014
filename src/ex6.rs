use std::thread;
use std::sync::mpsc;

// Example 6: Taste of concurrency.

struct Thing {
    label: String,
    count: i32
}

impl Thing {
    fn new(c: char) -> Thing {
        let mut s = String::new();
        s.push(c);
        Thing { label: s, count: 0 }
    }
}

pub fn main() {
    let mut t = Thing::new('a');
    t.count += 1;

    // A channel is for message passing between tasks
    let (tx, rx) = mpsc::channel();

    // Spawn a new task which takes t and tx.
    thread::spawn(move || {
        // Send a message back to the main thread with t's label
        tx.send(t.label).ok().expect("if none, send must have errored");
    });

    // Wait for the message.
    let c = rx.recv();
    println!("received a message from {:?}", c);
}

// EXERCISE: Add code that modifies `t` in the spawned task.  Add
// instrumentation to inspect that state in the original task; Did the
// original get modified by the spawned task?

// EXERCISE: The definition of `Thing` above uses `String` for its label.
// Change that back to `char` (and fix the code to accomodate the change).
// Then repeat previous exercise.  Discuss.
