#[allow(dead_code)]
fn solution(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}

#[test]
fn string_ends_with_test() {
    assert_eq!(solution("abc", "bc"), true); // returns true
    assert_eq!(solution("abc", "d"), false); // returns false
}
