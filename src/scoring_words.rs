use std::collections::HashMap;

fn score_word(score_mapping: &HashMap<char, usize>, word: &str) -> usize {
    word.chars().fold(0, |acc, ch| acc + score_mapping.get(&ch).unwrap())
}

#[allow(dead_code)]
pub fn high(input: &str) -> &str {
    let mut score_mapping: HashMap<char, usize> = HashMap::new();
    for (i, ch) in ('a'..='z').enumerate() {
        score_mapping.insert(ch, i + 1);
    }
    
    let mut best_word: &str = "";
    let mut best_score: usize = 0;

    input.split(' ').for_each(|w| {
        let score = score_word(&score_mapping, w);
        if score > best_score {
            best_word = w;
            best_score = score;
        }
    });

    best_word
}
