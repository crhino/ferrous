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

impl<'a, E: Debug + PartialEq> Matcher<E> for Equals<'a, E> {
    fn matches(&self, actual: &E) -> bool {
        self.expected == actual
    }

    fn failure_message(&self, actual: &E) -> String {
        format!("expected {:?} to equal {:?}", self.expected, actual)
    }

    fn negated_failure_message(&self, actual: &E) -> String {
        format!("expected {:?} not to equal {:?}", self.expected, actual)
    }
}
