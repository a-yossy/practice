use new_crate::add;

#[test]
fn integration_test() {
  assert_eq!(3, add(1, 2));
}
