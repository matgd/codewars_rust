mod string_ends_with;
mod testing_1_2_3;
mod duplicate_encode;

fn main() {
}

#[test]
fn string_ends_with_test() {
    assert_eq!(string_ends_with::solution("abc", "bc"), true); // returns true
    assert_eq!(string_ends_with::solution("abc", "d"), false); // returns false
}

#[test]
fn testing_1_2_3_test() {
    let empty_arr: &[&str] = &[];
    let empty_str_vec: Vec<String> = vec![];
    assert_eq!(testing_1_2_3::number(empty_arr), empty_str_vec);

    let non_empty_arr: &[&str] = &["a", "b", "c"];
    let non_empty_str_vec: Vec<String> = vec!["1: a".to_string(), "2: b".to_string(), "3: c".to_string()];
    assert_eq!(testing_1_2_3::number(non_empty_arr), non_empty_str_vec);
}

#[test]
fn duplicate_encode_test() {
  assert_eq!(duplicate_encode::duplicate_encode("din"),"(((");
  assert_eq!(duplicate_encode::duplicate_encode("recede"),"()()()");
  assert_eq!(duplicate_encode::duplicate_encode("Success"),")())())","should ignore case");
  assert_eq!(duplicate_encode::duplicate_encode("(( @"),"))((");
}
