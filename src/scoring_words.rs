use std::collections::HashMap;

fn score_word(score_mapping: &HashMap<char, usize>, word: &str) -> usize {
    word.chars().fold(0, |acc, ch| acc + score_mapping.get(&ch).unwrap())
}

#[allow(dead_code)]
fn high(input: &str) -> &str {
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

#[test]
fn high_scoring_test() {
    assert_eq!(high("man i need a taxi up to ubud"), "taxi");               
    assert_eq!(high("what time are we climbing up the volcano"), "volcano");
    assert_eq!(high("take me to semynak"), "semynak");                      
    assert_eq!(high("massage yes massage yes massage"), "massage");         
    assert_eq!(high("take two bintang and a dance please"), "bintang"); 
    assert_eq!(high("aa b"), "aa");         
    assert_eq!(high("b aa"), "b");     
    assert_eq!(high("bb d"), "bb");                            
    assert_eq!(high("d bb"), "d"); 
    assert_eq!(high("aaa b"), "aaa");                                     
}

