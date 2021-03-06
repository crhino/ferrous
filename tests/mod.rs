extern crate ferrous;

mod boolean;

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

#[test]
fn test_expect_to_ok() {
    let actual: Result<TestEnum, TestEnum> = Ok(TestEnum::Pass);
    expect(&actual).to(be_ok());
}

#[test]
#[should_panic(expected="expected Ok variant, found Err(Fail)")]
fn test_expect_to_ok_panic() {
    let actual: Result<TestEnum, TestEnum> = Err(TestEnum::Fail);
    expect(&actual).to(be_ok());
}

#[test]
#[should_panic(expected="expected Err variant, found Ok(Pass)")]
fn test_expect_not_to_ok_panic() {
    let actual: Result<TestEnum, TestEnum> = Ok(TestEnum::Pass);
    expect(&actual).not_to(be_ok());
}

#[test]
fn test_expect_to_err() {
    let actual: Result<TestEnum, TestEnum> = Err(TestEnum::Fail);
    expect(&actual).to(be_err());
}

#[test]
#[should_panic(expected="expected Err variant, found Ok(Pass)")]
fn test_expect_to_err_panic() {
    let actual: Result<TestEnum, TestEnum> = Ok(TestEnum::Pass);
    expect(&actual).to(be_err());
}

#[test]
#[should_panic(expected="expected Ok variant, found Err(Fail)")]
fn test_expect_not_to_err_panic() {
    let actual: Result<TestEnum, TestEnum> = Err(TestEnum::Fail);
    expect(&actual).not_to(be_err());
}

#[test]
fn test_eventually_should_err() {
    let actual: Result<TestEnum, TestEnum> = Err(TestEnum::Pass);

    eventually(move || {
        actual.clone()
    }).should(be_err());
}

#[test]
#[should_panic(expected="expected Ok variant, found Err(Fail)")]
fn test_eventually_should_not_err_panic() {
    let actual: Result<TestEnum, TestEnum> = Err(TestEnum::Fail);

    eventually(move || {
        actual.clone()
    }).should_not(be_err());
}

#[test]
fn test_consistently_should_err() {
    let actual: Result<TestEnum, TestEnum> = Err(TestEnum::Pass);

    consistently(move || {
        actual.clone()
    }).should(be_err());
}

#[test]
#[should_panic(expected="expected Ok variant, found Err(Fail)")]
fn test_consistently_should_not_err_panic() {
    let actual: Result<TestEnum, TestEnum> = Err(TestEnum::Fail);

    consistently(move || {
        actual.clone()
    }).should_not(be_err());
}
