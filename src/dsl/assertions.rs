use dsl::{Assertion, Matcher};

pub struct Expect<'a, A: 'a> {
    actual: &'a A,
}

impl<'a, A> Expect<'a, A> {
    pub fn new(actual: &'a A) -> Expect<'a, A> {
        Expect{
            actual: actual,
        }
    }
}

impl<'a, A: 'a> Assertion<'a, A> for Expect<'a, A> {
    fn to<M: Matcher<&'a A>>(self, matcher: M) {
        if !matcher.matches(self.actual) {
            panic!(matcher.failure_message(self.actual));
        }
    }

    fn not_to<M: Matcher<&'a A>>(self, matcher: M) {
        if matcher.matches(self.actual) {
            panic!(matcher.negated_failure_message(self.actual));
        }
    }
}


// /// Trait mapping to/not_to to should/should_not for better readability.
// pub trait AsyncAssertion<A> {
//     fn should<M>(self, matcher: M) {
//         self.to(matcher);
//     }

//     fn should_not<M>(self, matcher: M) {
//         self.not_to(matcher);
//     }
// }
