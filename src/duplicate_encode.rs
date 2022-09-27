use std::collections::HashMap;

#[allow(dead_code)]
pub fn duplicate_encode(word: &str)->String {
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
