#[allow(dead_code)]
pub fn rot13(message: &str) -> String {
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
