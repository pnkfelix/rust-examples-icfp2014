// Example 12: lifetimes

pub fn main() {
    let s_1_0 = [1, 0];
    let s_0_1 = [0, 1];
    let first_choice = choose(&s_1_0, &s_0_1);
    println!("first_choice: {:?}", first_choice);

    let second_choice = subdata_then_choose();
    println!("second_choice: {:?}", second_choice);
}

fn choose<'a>(x: &'a [usize], y: &'a [usize]) -> &'a [usize] {
    //    ^       ^               ^               ^
    //    |       |               |               |
    //    |       |               |               ^~ These ...
    //    |       |               ^~ are ...
    //    |       ^~ all ...
    //    |
    //    ^~~~ lifetimes.  (Bound here, referenced above.)

    // A semi-random complex predicate between inputs.
    if y[x[0]] > x[y[0]] {
        x
    } else {
        y
    }
}

// EXERCISE 1: What happens if you just remove all the lifetimes from
// signature of `choose` ?

// EXERCISE 2: Remove the `return constant;` in `subdata_then_choose`
// and uncomment the other code there.  Discuss.  How could you
// address this?

fn subdata_then_choose<'a>() -> &'a [u32]  {
    return &A_CONSTANT; // XXX
    /*

    let s_1_0_0 = [1, 0, 0];
    let s_0_2_0 = [0, 2, 0];
    choose(s_1_0_0, s_0_2_0)

    */
}

static A_CONSTANT : [u32; 3] = [0,1,2];
