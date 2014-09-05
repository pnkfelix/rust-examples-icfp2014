// Example 11: enums

// (Can you believe we got this far without "real" Algebraic Data Types?)

enum Request<X> {
    PlayOutsideWith(X),
    GoSwimming,
    LookAtStars,
    EatLunch,
}

enum Answer<X> {
    Yes(X),
    No(X),
    Maybe,
}

struct Parent;
//     ^~~~~~~ whoa.  (zero-sized!)

pub fn main() {
    let parent = Parent;

    let requests : [Request<String>, ..3] = [GoSwimming, LookAtStars, EatLunch];
    for request in requests.iter() {
        match parent.can_i(request) {

            Yes(_) => {
                println!("Hooray!");
                return;
            }
            No(why) => {
                println!("I do not believe you when you say {:s}!", why);
            }
            Maybe => {
                println!("Well, how about ...");
            }
        }
    }
}

impl<X> Answer<X> {
    fn map<Y>(self, f: |X| -> Y) -> Answer<Y> {
        unimplemented!(); // XXX see exercise 2 below.
    }
}

impl Parent {
    fn can_i<X>(&self, r: Request<X>) -> Answer<String> {
        let answer = match r {
            LookAtStars => Maybe,

            GoSwimming |
            PlayOutsideWith(_) => No("we live in on Mars"),

            EatLunch => Yes("it is close to noon"),
        };

        answer.map(|string_literal| String::from_str(string_literal))
    }
}

// EXERCISE: code does not compile; fix it somehow.

// EXERCISE: implement `Answer::map`
