struct Thing {
    label: String,
    count: int
}

impl Thing {
    fn new(c: char) -> Thing {
        Thing { label: String::from_char(1, c), count: 0 }
    }
}

pub fn main() {
    let mut t = Thing::new('a');

    // A channel is for message passing between tasks
    let (tx, rx) = channel();

    // Spawn a new task which takes t and tx.
    // proc() is a kind of closure.
    spawn(proc() {
        // Send a message back to the main thread with t's label
        tx.send(t.label);
    });

    // Wait for the message.
    let c = rx.recv();
    println!("received a message from {}", c);
}

// EXERCISE: Add code that modifies `t` in the spawned task.  Add
// instrumentation to inspect that state in the original task; Did the
// original get modified by the spawned task?

// EXERCISE: The definition of `Thing` above uses `String` for its label.
// Change that back to `char` (and fix the code to accomodate the change).
// Then repeat previous exercise.  Discuss.
