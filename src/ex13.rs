// Exercise 13: A (toy) linked list.

extern crate arena;

type A<'a, X> = &'a arena::TypedArena<X>;

pub struct HsNode<'a, X:'a> {
    arena: A<'a, HeavyStack<'a, X>>,
    value: X,
    next: &'a HeavyStack<'a, X>,
}

pub struct HsLeaf<'a, X:'a> {
    arena: A<'a, HeavyStack<'a, X>>
}

pub enum HeavyStack<'a, X:'a> {
    Cons(HsNode<'a,X>),
    Null(HsLeaf<'a,X>),
}

trait StackMaker<'a,X> {
    fn make_null(&'a self) -> HS<X>;
    fn make_cons(&'a self, h: X, t: HS<'a, X>) -> HS<X>;
}

impl<'a,X> StackMaker<'a,X> for arena::TypedArena<HeavyStack<'a,X>> {
    fn make_null(&'a self) -> HS<X> {
        self.alloc(HeavyStack::Null(HsLeaf{ arena: self }))
    }
    fn make_cons(&'a self, x: X, n: HS<'a,X>) -> HS<X> {
        self.alloc(HeavyStack::Cons(HsNode { arena: self, value: x, next: n }))
    }
}


pub type HSV<'a,X> = HeavyStack<'a, X>;
pub type HA<'a,X> = A<'a, HSV<'a,X>>;
pub type HS<'a,X> = &'a HSV<'a,X>;

impl<'a, X> HeavyStack<'a,X> {
    pub fn is_empty(&self) -> bool {
        match *self {
            HeavyStack::Cons(_) => false,
            HeavyStack::Null(_) => true,
        }
    }

    pub fn arena(&self) -> HA<'a,X> {
        match *self {
            HeavyStack::Cons(ref node) => node.arena,
            HeavyStack::Null(ref leaf) => leaf.arena,
        }
    }

    pub fn empty(arena: HA<'a,X>) -> HS<'a,X> {
        arena.make_null()
    }

    pub fn cons(&'a self, x: X) -> HS<'a,X> {
        self.arena().make_cons(x, self)
    }

    pub fn tail(&'a self) -> Option<HS<X>> {
        match self {
            &HeavyStack::Null(_) => None,
            &HeavyStack::Cons(ref n) => Some(n.next),
        }
    }

    pub fn head(&'a self) -> Option<X> {
        match self {
            &HeavyStack::Null(_) => None,
            &HeavyStack::Cons(n) => Some(n.value),
        }
    }
}

pub fn main() {
    let r = arena::TypedArena::<HSV<i32>>::new();
    let r1 = &r;
    let s0a = HeavyStack::empty(r1);            // ()
    assert!(s0a.is_empty());

    let s1b = s0a.cons(1);       // (1)
    assert!(s1b.head().unwrap() == 1);
    println!("s1b.head: {:?}", s1b.head());
}

// EXERCISE: The code does not compile. There is something
// fundamentally wrong with the "desired" API above. Come
// up with a fix.
//
// HINT: Review first exercise from ex10.


// EXERCISE: Add an additional `head_and_tail` method that follows
// through on the original intent of `fn head`.


// EXERCISE: Inspect the size_of the HeavyStack<int> type.  (See end
// of ex7.rs)
//
// Can you explain where every byte comes from?
//
// Can you make an alternative API what is more size-efficient?
