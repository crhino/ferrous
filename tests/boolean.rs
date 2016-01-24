extern crate ferrous;

use ferrous::dsl::*;

#[test]
fn test_expect_to_true() {
    let actual = true;
    expect(&actual).to(be_true());
}

#[test]
#[should_panic(expected="expected true, found false")]
fn test_expect_to_true_panic() {
    let actual = false;
    expect(&actual).to(be_true());
}

#[test]
#[should_panic(expected="expected false, found true")]
fn test_expect_not_to_true_panic() {
    let actual = true;
    expect(&actual).not_to(be_true());
}

#[test]
fn test_expect_to_false() {
    let actual = false;
    expect(&actual).to(be_false());
}

#[test]
#[should_panic(expected="expected false, found true")]
fn test_expect_to_false_panic() {
    let actual = true;
    expect(&actual).to(be_false());
}

#[test]
#[should_panic(expected="expected true, found false")]
fn test_expect_not_to_false_panic() {
    let actual = false;
    expect(&actual).not_to(be_false());
}
