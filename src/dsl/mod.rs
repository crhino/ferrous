mod matchers;
mod assertions;

use time::Duration;
use dsl::assertions::*;
use dsl::matchers::*;

pub trait Assertion<A> {
    fn to<M: Matcher<A>>(self, matcher: M);

    fn not_to<M: Matcher<A>>(self, matcher: M);
}

/// Trait mapping to/not_to to should/should_not for better readability.
pub trait AsyncAssertion<A> {
    fn should<M: Matcher<A>>(self, matcher: M);

    fn should_not<M: Matcher<A>>(self, matcher: M);
}

impl<A, B> AsyncAssertion<A> for B where B: Assertion<A> {
    fn should<M: Matcher<A>>(self, matcher: M) {
        self.to(matcher);
    }

    fn should_not<M: Matcher<A>>(self, matcher: M) {
        self.not_to(matcher);
    }
}

pub trait Matcher<A> {
    fn matches(&self, actual: &A) -> bool;
    fn failure_message(&self, actual: &A) -> String;
    fn negated_failure_message(&self, actual: &A) -> String;
}


pub fn expect<'a, A>(actual: &'a A) -> Expect<'a, A> {
    Expect::new(actual)
}

pub fn eventually<F, A>(f: F) -> Eventually<A> where F: 'static + Fn() -> A {
    let timeout = Duration::seconds(1);
    eventually_with_timeout(timeout, f)
}

pub fn eventually_with_timeout<F, A>(timeout: Duration, f: F)
-> Eventually<A> where F: 'static + Fn() -> A {
    Eventually::new(timeout, f)
}

// pub fn consistently<F, A>(f: F) -> Consistently<A> where F: Fn() -> A {
// }

pub fn equal<'a, E>(expected: &'a E) -> Equals<'a, E> {
    Equals::new(expected)
}

pub fn contain<'a, E>(expected: &'a E) -> Contain<'a, E> {
    Contain::new(expected)
}

pub fn be_some() -> OptionMatcher {
    OptionMatcher::SomeMatch
}

pub fn be_none() -> OptionMatcher {
    OptionMatcher::NoneMatch
}

pub fn be_ok() -> ResultMatcher {
    ResultMatcher::OkMatch
}

pub fn be_err() -> ResultMatcher {
    ResultMatcher::ErrMatch
}
