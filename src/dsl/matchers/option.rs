use std::fmt::Debug;
use dsl::Matcher;

#[derive(Debug)]
pub enum OptionMatcher {
    SomeMatch,
    NoneMatch,
}

impl<E: Debug> Matcher<Option<E>> for OptionMatcher {
    fn matches(&self, actual: &Option<E>) -> bool {
        match *self {
            OptionMatcher::SomeMatch => actual.is_some(),
            OptionMatcher::NoneMatch => actual.is_none(),
        }
    }

    fn failure_message(&self, actual: &Option<E>) -> String {
        match *self {
            OptionMatcher::SomeMatch => {
                format!("expected Some variant, found {:?}", actual)
            },
            OptionMatcher::NoneMatch => {
                format!("expected None variant, found {:?}", actual)
            },
        }
    }

    fn negated_failure_message(&self, actual: &Option<E>) -> String {
        match *self {
            OptionMatcher::SomeMatch => { format!("expected None variant, found {:?}", actual)
            },
            OptionMatcher::NoneMatch => {
                format!("expected Some variant, found {:?}", actual)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use dsl::Matcher;
    use super::*;

    #[test]
    fn test_some_matches() {
        let test = OptionMatcher::SomeMatch;
        assert!(test.matches(&Some(1)));
    }

    #[test]
    fn test_some_matches_fails() {
        let test = OptionMatcher::SomeMatch;
        assert!(!test.matches(&None as &Option<u8>));
    }

    #[test]
    fn test_some_failure_msg() {
        let test = OptionMatcher::SomeMatch;
        let msg = test.failure_message(&None as &Option<Vec<usize>>);
        assert_eq!(msg, String::from("expected Some variant, found None"));
    }

    #[test]
    fn test_some_negated_failure_msg() {
        let test = OptionMatcher::SomeMatch;
        let msg = test.negated_failure_message(&Some(1));
        assert_eq!(msg, String::from("expected None variant, found Some(1)"));
    }

    #[test]
    fn test_none_matches() {
        let test = OptionMatcher::NoneMatch;
        assert!(test.matches(&None as &Option<OptionMatcher>));
    }

    #[test]
    fn test_none_matches_fail() {
        let test = OptionMatcher::NoneMatch;
        assert!(!test.matches(&Some(1u8)));
    }

    #[test]
    fn test_none_failure_msg() {
        let test = OptionMatcher::NoneMatch;
        let msg = test.failure_message(&Some(1u8));
        assert_eq!(msg, String::from("expected None variant, found Some(1)"));
    }

    #[test]
    fn test_none_negated_failure_msg() {
        let test = OptionMatcher::NoneMatch;
        let msg = test.negated_failure_message(&None as &Option<u8>);
        assert_eq!(msg, String::from("expected Some variant, found None"));
    }
}
