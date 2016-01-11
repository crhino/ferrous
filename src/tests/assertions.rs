use dsl::*;

#[derive(Debug, Clone, Eq, PartialEq)]
enum TestEnum {
    Pass,
    Fail
}

#[test]
fn test_expect_to() {
    let actual = 1;
    expect(&actual).to(equal(&1));
}

#[test]
#[should_panic]
fn test_expect_to_panic() {
    let actual = 1;
    expect(&actual).to(equal(&2));
}

#[test]
fn test_expect_not_to() {
    let actual = 2;
    expect(&actual).not_to(equal(&1));
}

#[test]
#[should_panic]
fn test_expect_not_to_panic() {
    let actual = 1;
    expect(&actual).not_to(equal(&1));
}

#[test]
fn test_expect_to_custom() {
    let actual = TestEnum::Pass;
    expect(&actual).to(equal(&TestEnum::Pass));
}

#[test]
#[should_panic]
fn test_expect_to_custom_panic() {
    let actual = TestEnum::Fail;
    expect(&actual).to(equal(&TestEnum::Pass));
}
