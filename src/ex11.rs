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

    let requests : [Request<String>; 3] = [Request::GoSwimming,
                                           Request::LookAtStars,
                                           Request::EatLunch];
    for request in requests.iter() {
        match parent.can_i(request) {

            Answer::Yes(_) => {
                println!("Hooray!");
                return;
            }
            Answer::No(why) => {
                println!("I do not believe you when you say {}!", why);
            }
            Answer::Maybe => {
                println!("Well, how about ...");
            }
        }
    }
}

impl<X> Answer<X> {
    fn map<Y, F>(self, f: F) -> Answer<Y> where F: Fn(X) -> Y {
        unimplemented!(); // XXX see exercise 2 below.
    }
}

impl Parent {
    fn can_i<X>(&self, r: Request<X>) -> Answer<String> {
        let answer = match r {
            Request::LookAtStars => Answer::Maybe,

            Request::GoSwimming |
            Request::PlayOutsideWith(_) => Answer::No("we live in on Mars"),

            Request::EatLunch => Answer::Yes("it is close to noon"),
        };

        answer.map(|string_literal| string_literal.to_string())
    }
}

// EXERCISE: code does not compile; fix it somehow.

// EXERCISE: implement `Answer::map`
