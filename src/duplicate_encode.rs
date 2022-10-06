use std::collections::HashMap;

#[allow(dead_code)]
fn duplicate_encode(word: &str)->String {
    let lower_word = word.to_lowercase();

    let mut chars_map = HashMap::new();
    lower_word.chars().for_each(|ch| {
        let counted: i32 = match chars_map.get(&ch) {
            Some(count) => *count + 1,
            None => 1,
        };
        chars_map.insert(ch, counted);
    });

    lower_word.chars().map(|ch| {
        match chars_map.get(&ch) {
            Some(1) => "(",
            _ => ")",
        }
    }).collect::<Vec<_>>().join("")
}

#[test]
fn duplicate_encode_test() {
  assert_eq!(duplicate_encode("din"),"(((");
  assert_eq!(duplicate_encode("recede"),"()()()");
  assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
  assert_eq!(duplicate_encode("(( @"),"))((");
}

