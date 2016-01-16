use dsl::{Assertion, Matcher};
use time::{Duration, PreciseTime};
use std::thread;

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

impl<'a, A: 'a> Assertion<A> for Expect<'a, A> {
    fn to<M: Matcher<A>>(self, matcher: M) {
        if !matcher.matches(self.actual) {
            panic!(matcher.failure_message(self.actual));
        }
    }

    fn not_to<M: Matcher<A>>(self, matcher: M) {
        if matcher.matches(self.actual) {
            panic!(matcher.negated_failure_message(self.actual));
        }
    }
}

pub struct Eventually<A> {
    func: Box<Fn() -> A>,
    timeout: Duration,
    polling_interval: u32,
}

impl<A> Eventually<A> {
    pub fn new<F>(timeout: Duration, f: F) -> Eventually<A>
        where F: 'static + Fn() -> A {
            Eventually {
                func: Box::new(f),
                timeout: timeout,
                polling_interval: 10,
            }
        }
}

impl<A> Assertion<A> for Eventually<A> {
    fn to<M: Matcher<A>>(self, matcher: M) {
        let f = self.func;
        let start = PreciseTime::now();
        while start.to(PreciseTime::now()) < self.timeout {
            let actual = f();
            if matcher.matches(&actual) {
                return
            }
            thread::sleep_ms(self.polling_interval);
        }

        let actual = f();
        panic!(matcher.failure_message(&actual));
    }

    fn not_to<M: Matcher<A>>(self, matcher: M) {
        let f = self.func;
        let start = PreciseTime::now();
        while start.to(PreciseTime::now()) < self.timeout {
            let actual = f();
            if !matcher.matches(&actual) {
                return
            }
            thread::sleep_ms(self.polling_interval);
        }

        let actual = f();
        panic!(matcher.negated_failure_message(&actual));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Duration;
    use std::sync::{Arc};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;
    use dsl::*;

    #[derive(Debug, PartialEq)]
    struct Test(usize);

    #[test]
    fn test_eventually() {
        let timeout = Duration::seconds(1);
        let change = Arc::new(AtomicBool::new(false));
        let return_bool = change.clone();
        let assertion = Eventually::new(timeout, move || {
            if return_bool.load(Ordering::SeqCst) {
                Test(100)
            } else {
                Test(0)
            }
        });

        let handle = thread::spawn(move || {
            thread::sleep_ms(500);
            change.store(true, Ordering::SeqCst);
        });

        assertion.should(equal(&Test(100)));
        handle.join().unwrap();
    }

    #[test]
    #[should_panic(expected="expected Test(100) to equal Test(0)")]
    fn test_eventually_timeout() {
        let timeout = Duration::seconds(1);
        let assertion = Eventually::new(timeout, move || {
            Test(0)
        });

        assertion.should(equal(&Test(100)));
    }

    #[test]
    fn test_eventually_negated() {
        let timeout = Duration::seconds(1);
        let change = Arc::new(AtomicBool::new(false));
        let return_bool = change.clone();
        let assertion = Eventually::new(timeout, move || {
            if return_bool.load(Ordering::SeqCst) {
                Test(100)
            } else {
                Test(0)
            }
        });

        let handle = thread::spawn(move || {
            thread::sleep_ms(500);
            change.store(true, Ordering::SeqCst);
        });

        assertion.should_not(equal(&Test(0)));
        handle.join().unwrap();
    }

    #[test]
    #[should_panic(expected="expected Test(0) not to equal Test(0)")]
    fn test_eventually_negated_timeout() {
        let timeout = Duration::seconds(1);
        let assertion = Eventually::new(timeout, move || {
            Test(0)
        });

        assertion.should_not(equal(&Test(0)));
    }
}
