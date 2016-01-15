use std::fmt::Debug;
use dsl::Matcher;

pub struct Contain<'a, E: 'a> {
    expected: &'a E,
}

impl<'a, E> Contain<'a, E> {
    pub fn new(expected: &'a E) -> Contain<E> {
        Contain{
            expected: expected,
        }
    }
}

impl<'c, E: Debug + PartialEq<T>, I, T> Matcher<I> for Contain<'c, E>
where for<'a> &'a I: IntoIterator<Item=&'a T> {
    fn matches(&self, actual: &I) -> bool {
        for a in actual {
            if self.expected.eq(&a) {
                return true
            }
        }

        false
    }

    fn failure_message(&self, _actual: &I) -> String {
        format!("expected to find {:?} in iterator", self.expected)
    }

    fn negated_failure_message(&self, _actual: &I) -> String {
        format!("expected not to find {:?} in iterator", self.expected)
    }
}

#[cfg(test)]
mod tests {
    use dsl::Matcher;
    use super::*;

    #[test]
    fn test_equal_matches() {
        let test = 1u8;
        let actual = vec![1, 2, 3];
        let contain = Contain::new(&test);
        assert!(contain.matches(&actual));
    }

    #[test]
    fn test_equal_failure_msg() {
        let test = 4u8;
        let actual = vec![1, 2, 3];
        let contain = Contain::new(&test);
        let msg = contain.failure_message(&actual);
        assert_eq!(msg, String::from("expected to find 4 in iterator"));
    }

    #[test]
    fn test_equal_negated_failure_msg() {
        let test = 2u8;
        let actual = vec![1, 2, 3];
        let contain = Contain::new(&test);
        let msg = contain.negated_failure_message(&actual);
        assert_eq!(msg, String::from("expected not to find 2 in iterator"));
    }
}
