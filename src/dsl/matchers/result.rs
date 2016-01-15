use std::fmt::Debug;
use dsl::Matcher;

#[derive(Debug)]
pub enum ResultMatcher {
    OkMatch,
    ErrMatch,
}

impl<E: Debug, O: Debug> Matcher<Result<O, E>> for ResultMatcher {
    fn matches(&self, actual: &Result<O, E>) -> bool {
        match *self {
            ResultMatcher::ErrMatch => actual.is_err(),
            ResultMatcher::OkMatch => actual.is_ok(),
        }
    }

    fn failure_message(&self, actual: &Result<O, E>) -> String {
        match *self {
            ResultMatcher::ErrMatch => {
                format!("expected Err variant, found {:?}", actual)
            },
            ResultMatcher::OkMatch => {
                format!("expected Ok variant, found {:?}", actual)
            },
        }
    }

    fn negated_failure_message(&self, actual: &Result<O, E>) -> String {
        match *self {
            ResultMatcher::ErrMatch => { format!("expected Ok variant, found {:?}", actual)
            },
            ResultMatcher::OkMatch => {
                format!("expected Err variant, found {:?}", actual)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use dsl::Matcher;
    use super::*;

    #[test]
    fn test_err_matches() {
        let test = ResultMatcher::ErrMatch;
        assert!(test.matches(&Err(1u8) as &Result<u8, u8>));
    }

    #[test]
    fn test_err_matches_fails() {
        let test = ResultMatcher::ErrMatch;
        assert!(!test.matches(&Ok(9u8) as &Result<u8, u8>));
    }

    #[test]
    fn test_err_failure_msg() {
        let test = ResultMatcher::ErrMatch;
        let msg = test.failure_message(&Ok(1u8) as &Result<u8, u8>);
        assert_eq!(msg, String::from("expected Err variant, found Ok(1)"));
    }

    #[test]
    fn test_err_negated_failure_msg() {
        let test = ResultMatcher::ErrMatch;
        let msg = test.negated_failure_message(&Err(1u8) as &Result<u8, u8>);
        assert_eq!(msg, String::from("expected Ok variant, found Err(1)"));
    }

    #[test]
    fn test_ok_matches() {
        let test = ResultMatcher::OkMatch;
        assert!(test.matches(&Ok(1u8) as &Result<u8, u8>));
    }

    #[test]
    fn test_ok_matches_fail() {
        let test = ResultMatcher::OkMatch;
        assert!(!test.matches(&Err(1u8) as &Result<u8, u8>));
    }

    #[test]
    fn test_ok_failure_msg() {
        let test = ResultMatcher::OkMatch;
        let msg = test.failure_message(&Err(1u8) as &Result<u8, u8>);
        assert_eq!(msg, String::from("expected Ok variant, found Err(1)"));
    }

    #[test]
    fn test_ok_negated_failure_msg() {
        let test = ResultMatcher::OkMatch;
        let msg = test.negated_failure_message(&Ok(1u8) as &Result<u8, u8>);
        assert_eq!(msg, String::from("expected Err variant, found Ok(1)"));
    }
}
