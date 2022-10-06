#[allow(dead_code)]
fn min_max(lst: &[i32]) -> (i32, i32) {
    (*lst.iter().min().unwrap(), *lst.iter().max().unwrap())
}

#[test]
fn min_max_test() {
  assert_eq!(min_max(&[1]), (1,1));
  assert_eq!(min_max(&[1,2,3,4,5]), (1,5));
  assert_eq!(min_max(&[2334454,5]), (5,2334454));
}


