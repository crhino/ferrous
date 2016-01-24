use dsl::Matcher;

#[derive(Debug)]
pub enum BooleanMatcher {
    TrueMatch,
    FalseMatch,
}

impl Matcher<bool> for BooleanMatcher {
    fn matches(&self, actual: &bool) -> bool {
        match *self {
            BooleanMatcher::TrueMatch => *actual,
            BooleanMatcher::FalseMatch => !*actual,
        }
    }

    fn failure_message(&self, actual: &bool) -> String {
        match *self {
            BooleanMatcher::TrueMatch => {
                format!("expected true, found {:?}", actual)
            },
            BooleanMatcher::FalseMatch => {
                format!("expected false, found {:?}", actual)
            },
        }
    }

    fn negated_failure_message(&self, actual: &bool) -> String {
        match *self {
            BooleanMatcher::TrueMatch => { format!("expected false, found {:?}", actual)
            },
            BooleanMatcher::FalseMatch => {
                format!("expected true, found {:?}", actual)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use dsl::Matcher;
    use super::*;

    #[test]
    fn test_true_matches() {
        let test = BooleanMatcher::TrueMatch;
        assert!(test.matches(&true));
    }

    #[test]
    fn test_true_matches_fails() {
        let test = BooleanMatcher::TrueMatch;
        assert!(!test.matches(&false));
    }

    #[test]
    fn test_true_failure_msg() {
        let test = BooleanMatcher::TrueMatch;
        let msg = test.failure_message(&false);
        assert_eq!(msg, String::from("expected true, found false"));
    }

    #[test]
    fn test_true_negated_failure_msg() {
        let test = BooleanMatcher::TrueMatch;
        let msg = test.negated_failure_message(&true);
        assert_eq!(msg, String::from("expected false, found true"));
    }

    #[test]
    fn test_false_matches() {
        let test = BooleanMatcher::FalseMatch;
        assert!(test.matches(&false));
    }

    #[test]
    fn test_false_matches_fail() {
        let test = BooleanMatcher::FalseMatch;
        assert!(!test.matches(&true));
    }

    #[test]
    fn test_false_failure_msg() {
        let test = BooleanMatcher::FalseMatch;
        let msg = test.failure_message(&true);
        assert_eq!(msg, String::from("expected false, found true"));
    }

    #[test]
    fn test_false_negated_failure_msg() {
        let test = BooleanMatcher::FalseMatch;
        let msg = test.negated_failure_message(&false);
        assert_eq!(msg, String::from("expected true, found false"));
    }
}
