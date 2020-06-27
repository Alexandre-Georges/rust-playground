// Dashes are replaced with underscores
use test_project;

mod common;

// This is an integration test file, integration tests are run only if unit tests pass
#[test]
fn it_adds_two_integration() {
  common::setup();
  assert_eq!(4, test_project::add_two(2));
}