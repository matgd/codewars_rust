#[allow(dead_code)]
pub fn create_phone_number(numbers: &[u8]) -> String {
    let numbers_s: String = numbers
        .iter()
        .map(|&x| format!("{}", x))
        .collect::<Vec<String>>()
        .join("");
    format!("({}) {}-{}", &numbers_s[..3], &numbers_s[3..6], &numbers_s[6..])
}

