use std::fmt::Debug;
use dsl::Matcher;

pub struct Equals<'a, E: 'a> {
    expected: &'a E,
}

impl<'a, E> Equals<'a, E> {
    pub fn new(expected: &'a E) -> Equals<E> {
        Equals{
            expected: expected,
        }
    }
}

impl<'a, 'e, E: Debug + PartialEq> Matcher<&'e E> for Equals<'a, E> {
    fn matches(&self, actual: &'e E) -> bool {
        self.expected == actual
    }

    fn failure_message(&self, actual: &'e E) -> String {
        format!("expected {:?} to equal {:?}", self.expected, actual)
    }

    fn negated_failure_message(&self, actual: &'e E) -> String {
        format!("expected {:?} not to equal {:?}", self.expected, actual)
    }
}

#[cfg(test)]
mod tests {
    use dsl::Matcher;
    use super::*;

    #[test]
    fn test_equal_matches() {
        let test = 1;
        let equals = Equals::new(&test);
        assert!(equals.matches(&1));
    }

    #[test]
    fn test_equal_failure_msg() {
        let test = 1;
        let equals = Equals::new(&test);
        let msg = equals.failure_message(&2);
        assert_eq!(msg, String::from("expected 1 to equal 2"));
    }

    #[test]
    fn test_equal_negated_failure_msg() {
        let test = 1;
        let equals = Equals::new(&test);
        let msg = equals.negated_failure_message(&1);
        assert_eq!(msg, String::from("expected 1 not to equal 1"));
    }
}
