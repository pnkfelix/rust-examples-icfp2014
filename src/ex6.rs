struct Thing {
    label: char,
    count: int
}

impl Thing {
    fn new(c: char) -> Thing {
        Thing { label: c, count: 0 }
    }
}

pub fn main() {
    let t = Thing::new('a');

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
