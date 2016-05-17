use dsl::{Assertion, Matcher};
use std::time::{Duration, Instant};
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

#[derive(Clone, Debug, Copy)]
pub enum AsyncType {
    Eventual,
    Consistent,
}

pub struct Async<A> {
    func: Box<Fn() -> A>,
    timeout: Duration,
    polling_interval: Duration,
    async_type: AsyncType
}

impl<A> Async<A> {
    pub fn new<F>(async_type: AsyncType, timeout: Duration, f: F) -> Async<A>
        where F: 'static + Fn() -> A {
            Async {
                func: Box::new(f),
                timeout: timeout,
                polling_interval: Duration::from_millis(10),
                async_type: async_type,
            }
        }

    // Return value signifies the while loop should break early.
    fn check_match<M: Matcher<A>>(&self, matcher: &M, actual: A) -> bool {
        match self.async_type {
            AsyncType::Eventual => {
                if matcher.matches(&actual) {
                    true
                } else {
                    false
                }
            },
            AsyncType::Consistent => {
                if matcher.matches(&actual) {
                    false
                } else {
                    panic!(matcher.failure_message(&actual));
                }
            },
        }
    }

    // Return value signifies the while loop should break early.
    fn check_negated_match<M: Matcher<A>>(&self, matcher: &M, actual: A) -> bool {
        match self.async_type {
            AsyncType::Eventual => {
                if !matcher.matches(&actual) {
                    true
                } else {
                    false
                }
            },
            AsyncType::Consistent => {
                if !matcher.matches(&actual) {
                    false
                } else {
                    panic!(matcher.negated_failure_message(&actual));
                }
            },
        }
    }

    fn check_at_end<M: Matcher<A>>(&self, matcher: &M, actual: A) {
        match self.async_type {
            AsyncType::Eventual => {
                panic!(matcher.failure_message(&actual));
            },
            AsyncType::Consistent => {},
        }
    }

    fn check_negated_at_end<M: Matcher<A>>(&self, matcher: &M, actual: A) {
        match self.async_type {
            AsyncType::Eventual => {
                panic!(matcher.negated_failure_message(&actual));
            },
            AsyncType::Consistent => {},
        }
    }
}

impl<A> Assertion<A> for Async<A> {
    fn to<M: Matcher<A>>(self, matcher: M) {
        let ref f = self.func;
        let start = Instant::now();
        while start.elapsed() < self.timeout {
            let actual = f();
            if self.check_match(&matcher, actual) {
                return
            }
            thread::sleep(self.polling_interval);
        }

        let actual = f();
        self.check_at_end(&matcher, actual);
    }

    fn not_to<M: Matcher<A>>(self, matcher: M) {
        let ref f = self.func;
        let start = Instant::now();
        while start.elapsed() < self.timeout {
            let actual = f();
            if self.check_negated_match(&matcher, actual) {
                return
            }
            thread::sleep(self.polling_interval);
        }

        let actual = f();
        self.check_negated_at_end(&matcher, actual);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;
    use std::thread;
    use dsl::*;

    #[derive(Debug, PartialEq)]
    struct Test(usize);

    #[test]
    fn test_eventually() {
        let timeout = Duration::from_secs(1);
        let change = Arc::new(AtomicBool::new(false));
        let return_bool = change.clone();
        let assertion = Async::new(AsyncType::Eventual, timeout, move || {
            if return_bool.load(Ordering::SeqCst) {
                Test(100)
            } else {
                Test(0)
            }
        });

        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(500));
            change.store(true, Ordering::SeqCst);
        });

        assertion.should(equal(&Test(100)));
        handle.join().unwrap();
    }

    #[test]
    #[should_panic(expected="expected Test(100) to equal Test(0)")]
    fn test_eventually_timeout() {
        let timeout = Duration::from_secs(1);
        let assertion = Async::new(AsyncType::Eventual, timeout, move || {
            Test(0)
        });

        assertion.should(equal(&Test(100)));
    }

    #[test]
    fn test_eventually_negated() {
        let timeout = Duration::from_secs(1);
        let change = Arc::new(AtomicBool::new(false));
        let return_bool = change.clone();
        let assertion = Async::new(AsyncType::Eventual, timeout, move || {
            if return_bool.load(Ordering::SeqCst) {
                Test(100)
            } else {
                Test(0)
            }
        });

        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(500));
            change.store(true, Ordering::SeqCst);
        });

        assertion.should_not(equal(&Test(0)));
        handle.join().unwrap();
    }

    #[test]
    #[should_panic(expected="expected Test(0) not to equal Test(0)")]
    fn test_eventually_negated_timeout() {
        let timeout = Duration::from_secs(1);
        let assertion = Async::new(AsyncType::Eventual, timeout, move || {
            Test(0)
        });

        assertion.should_not(equal(&Test(0)));
    }

    #[test]
    fn test_consistently() {
        let timeout = Duration::from_secs(1);
        let assertion = Async::new(AsyncType::Consistent, timeout, move || {
            Test(100)
        });

        assertion.should(equal(&Test(100)));
    }

    #[test]
    #[should_panic(expected="expected Test(100) to equal Test(0)")]
    fn test_consistently_fail() {
        let timeout = Duration::from_secs(1);
        let change = Arc::new(AtomicBool::new(true));
        let return_bool = change.clone();
        let assertion = Async::new(AsyncType::Consistent, timeout, move || {
            if return_bool.load(Ordering::SeqCst) {
                Test(100)
            } else {
                Test(0)
            }
        });

        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(500));
            change.store(false, Ordering::SeqCst);
        });

        assertion.should(equal(&Test(100)));
        handle.join().unwrap();
    }

    #[test]
    fn test_consistently_negated() {
        let timeout = Duration::from_secs(1);
        let assertion = Async::new(AsyncType::Consistent, timeout, move || {
            Test(100)
        });

        assertion.should_not(equal(&Test(0)));
    }

    #[test]
    #[should_panic(expected="expected Test(0) not to equal Test(0)")]
    fn test_consistently_negated_fail() {
        let timeout = Duration::from_secs(1);
        let change = Arc::new(AtomicBool::new(false));
        let return_bool = change.clone();
        let assertion = Async::new(AsyncType::Consistent, timeout, move || {
            if return_bool.load(Ordering::SeqCst) {
                Test(100)
            } else {
                Test(0)
            }
        });

        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(500));
            change.store(true, Ordering::SeqCst);
        });

        assertion.should_not(equal(&Test(0)));
        handle.join().unwrap();
    }
}
