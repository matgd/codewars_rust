#[allow(dead_code)]
fn spin_words(words: &str) -> String {
    let split_words = words.split(' ');

    let reversed = split_words.map(|w| {
        match w.len() >= 5 {
            true => w.chars().rev().collect::<String>(),
            false => w.to_owned()
        }
    }).collect::<Vec<String>>();

    reversed.join(" ")
}

#[test]
fn spin_words_test() {
    assert_eq!(spin_words("Welcome"), "emocleW");
    assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
    assert_eq!(spin_words("This is a test"), "This is a test");
    assert_eq!(spin_words("This is another test"), "This is rehtona test");
    assert_eq!(spin_words("You are almost to the last test"), "You are tsomla to the last test");
    assert_eq!(spin_words("Just kidding there is still one more"), "Just gniddik ereht is llits one more");
    assert_eq!(spin_words("Seriously this is the last one"), "ylsuoireS this is the last one");
}

