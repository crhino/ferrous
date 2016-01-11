extern crate ferrous;

use ferrous::dsl::*;

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

#[test]
fn test_expect_to_contain() {
    let actual = vec![1, 2, 3, 4, 5];
    expect(&actual).to(contain(&1u8));
}

#[test]
#[should_panic]
fn test_expect_to_contain_panic() {
    let actual = vec![1, 2, 3, 4, 5];
    expect(&actual).not_to(contain(&1u8));
}

#[test]
fn test_expect_to_some() {
    let actual = Some(TestEnum::Pass);
    expect(&actual).to(be_some());
}

#[test]
#[should_panic(expected="expected Some variant, found None")]
fn test_expect_to_some_panic() {
    let actual: Option<TestEnum> = None;
    expect(&actual).to(be_some());
}

#[test]
#[should_panic(expected="expected None variant, found Some(Pass)")]
fn test_expect_not_to_some_panic() {
    let actual: Option<TestEnum> = Some(TestEnum::Pass);
    expect(&actual).not_to(be_some());
}

#[test]
fn test_expect_to_none() {
    let actual: Option<TestEnum> = None;
    expect(&actual).to(be_none());
}

#[test]
#[should_panic(expected="expected None variant, found Some(Pass)")]
fn test_expect_to_none_panic() {
    let actual: Option<TestEnum> = Some(TestEnum::Pass);
    expect(&actual).to(be_none());
}

#[test]
#[should_panic(expected="expected Some variant, found None")]
fn test_expect_not_to_none_panic() {
    let actual: Option<TestEnum> = None;
    expect(&actual).not_to(be_none());
}
