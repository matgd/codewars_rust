#[allow(dead_code)]
fn rot13(message: &str) -> String {
    let lowers = ('a'..='z').collect::<Vec<char>>();
    let uppers = ('A'..='Z').collect::<Vec<char>>();
    
    let mut rotated: Vec<char> = vec![];
    message.chars().for_each(|ch| {
        if ch.is_lowercase() {
            let index = lowers.iter().position(|v| v == &ch).unwrap();
            rotated.push(lowers[(index + 13) % lowers.len()]);
        } else if ch.is_uppercase() {
            let index = uppers.iter().position(|v| v == &ch).unwrap();
            rotated.push(uppers[(index + 13) % uppers.len()]);
        } else {
            rotated.push(ch);
        }
    });

    rotated.iter().collect()
}

#[test]
fn rot13_test() {
    assert_eq!(rot13("test"), "grfg");
    assert_eq!(rot13("Test"), "Grfg");
    assert_eq!(rot13("T est"), "G rfg");
    assert_eq!(rot13("123Test"), "123Grfg");
}
