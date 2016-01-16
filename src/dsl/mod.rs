//! The DSL module for the ferrous crate.
//!
//! Usually, a user of this library would want to import all symbols from this module.
//!
//! ```
//! use ferrous::dsl::*;
//!
//! expect(&1).to(equal(&1));
//! ```

mod matchers;
mod assertions;

use time::Duration;
use dsl::assertions::*;
use dsl::matchers::*;

/// Trait for assertion types.
pub trait Assertion<A> {
    /// Positive assertion with the given matcher.
    fn to<M: Matcher<A>>(self, matcher: M);

    /// Negative assertion with the given matcher.
    fn not_to<M: Matcher<A>>(self, matcher: M);
}

/// Trait mapping to/not_to to should/should_not for better readability.
pub trait AsyncAssertion<A> {
    /// Positive assertion with the given matcher.
    fn should<M: Matcher<A>>(self, matcher: M);

    /// Negative assertion with the given matcher.
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

/// Trait that must be implemented by any matcher type.
pub trait Matcher<A> {
    /// Defines whether or not the actual value matches the expected value
    fn matches(&self, actual: &A) -> bool;
    /// The positive failure message
    fn failure_message(&self, actual: &A) -> String;
    /// The megative failure message
    fn negated_failure_message(&self, actual: &A) -> String;
}


/// Simple expectation assertion.
pub fn expect<'a, A>(actual: &'a A) -> Expect<'a, A> {
    Expect::new(actual)
}

/// Asynchronous assertion that asserts the given closure **eventually** returns a value
/// that satisfies the given matcher.
pub fn eventually<F, A>(f: F) -> Async<A> where F: 'static + Fn() -> A {
    let timeout = Duration::seconds(1);
    eventually_with_timeout(timeout, f)
}

/// Asynchronous eventual assertion with user-defined timeout.
pub fn eventually_with_timeout<F, A>(timeout: Duration, f: F)
-> Async<A> where F: 'static + Fn() -> A {
    Async::new(AsyncType::Eventual, timeout, f)
}

/// Asynchronous assertion that asserts the given closure **consistently** returns a value
/// that satisfies the given matcher.
pub fn consistently<F, A>(f: F) -> Async<A> where F: 'static + Fn() -> A {
    let timeout = Duration::seconds(1);
    consistently_with_timeout(timeout, f)
}

/// Asynchronous consistent assertion with user-defined timeout.
pub fn consistently_with_timeout<F, A>(timeout: Duration, f: F)
-> Async<A> where F: 'static + Fn() -> A {
    Async::new(AsyncType::Consistent, timeout, f)
}

/// Simple equality matcher
///
/// Uses the PartialEq trait.
pub fn equal<'a, E>(expected: &'a E) -> Equals<'a, E> {
    Equals::new(expected)
}

/// Container matcher that asserts a given element exists within an iterator.
///
/// Uses the PartialEq trait.
pub fn contain<'a, E>(expected: &'a E) -> Contain<'a, E> {
    Contain::new(expected)
}

/// Matcher that asserts a value is a Some variant.
pub fn be_some() -> OptionMatcher {
    OptionMatcher::SomeMatch
}

/// Matcher that asserts a value is a None variant.
pub fn be_none() -> OptionMatcher {
    OptionMatcher::NoneMatch
}

/// Matcher that asserts a value is a Ok variant.
pub fn be_ok() -> ResultMatcher {
    ResultMatcher::OkMatch
}

/// Matcher that asserts a value is a Err variant.
pub fn be_err() -> ResultMatcher {
    ResultMatcher::ErrMatch
}
