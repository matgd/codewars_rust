#[allow(dead_code)]
fn valid_parentheses(s: &str) -> bool {
    let mut opened_parentheses = 0;

    for ch in s.chars() {
        if opened_parentheses < 0 {
            return false;
        }

        match ch {
            '(' => opened_parentheses += 1,
            ')' => opened_parentheses -= 1,
            _ => continue
        }
    }

    opened_parentheses == 0
}

#[test]
fn valid_parentheses_test() {
    assert_eq!(valid_parentheses("()"), true);
    assert_eq!(valid_parentheses("())"), false);
    assert_eq!(valid_parentheses(""), true);
    assert_eq!(valid_parentheses("(}{)"), true);
    assert_eq!(valid_parentheses(")(()))"), false);
    assert_eq!(valid_parentheses("("), false);
    assert_eq!(valid_parentheses("(())((()())())"), true);
}
