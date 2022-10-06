#[allow(dead_code)]
fn validate_pin(pin: &str) -> bool {
    [4, 6].contains(&pin.len()) && pin.chars().all(|ch| ch.is_digit(10))
}

#[test]
fn validate_pin_test() {
    assert_eq!(validate_pin("1"), false);
    assert_eq!(validate_pin("12"), false);
    assert_eq!(validate_pin("123"), false);
    assert_eq!(validate_pin("12345"), false);
    assert_eq!(validate_pin("1234567"), false);
    assert_eq!(validate_pin("-1234"), false);
    assert_eq!(validate_pin("1.234"), false);
    assert_eq!(validate_pin("-1.234"), false);
    assert_eq!(validate_pin("00000000"), false);
    assert_eq!(validate_pin("a234"), false);
    assert_eq!(validate_pin(".234"), false);
    assert_eq!(validate_pin("1234"), true);
    assert_eq!(validate_pin("0000"), true);
    assert_eq!(validate_pin("1111"), true);
    assert_eq!(validate_pin("123456"), true);
    assert_eq!(validate_pin("098765"), true);
    assert_eq!(validate_pin("000000"), true);
    assert_eq!(validate_pin("123456"), true);
    assert_eq!(validate_pin("090909"), true);
}

