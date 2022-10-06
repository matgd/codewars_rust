#[allow(dead_code)]
fn create_phone_number(numbers: &[u8]) -> String {
    let numbers_s: String = numbers
        .iter()
        .map(|&x| format!("{}", x))
        .collect::<Vec<String>>()
        .join("");
    format!("({}) {}-{}", &numbers_s[..3], &numbers_s[3..6], &numbers_s[6..])
}

#[test]
fn create_phone_number_test() {
    assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), "(123) 456-7890");
    assert_eq!(create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "(111) 111-1111");
    assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), "(123) 456-7899");
}

