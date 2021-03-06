// each file in the tests/ directory is compile as its own separate crate
// Files in subdirectories of the tests directory don’t get compiled as separate
// crates or have sections in the test output.
use test_organization;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_organization::add_two(2));
}
