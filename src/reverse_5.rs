#[allow(dead_code)]
pub fn spin_words(words: &str) -> String {
    let split_words = words.split(' ');

    let reversed = split_words.map(|w| {
        match w.len() >= 5 {
            true => w.chars().rev().collect::<String>(),
            false => w.to_owned()
        }
    }).collect::<Vec<String>>();

    reversed.join(" ")
}
