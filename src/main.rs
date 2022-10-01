mod string_ends_with;
mod testing_1_2_3;
mod duplicate_encode;
mod min_max;
mod walk;
mod validate_pin;
mod reverse_5;
mod scoring_words;

fn main() {
}

#[test]
fn string_ends_with_test() {
    assert_eq!(string_ends_with::solution("abc", "bc"), true); // returns true
    assert_eq!(string_ends_with::solution("abc", "d"), false); // returns false
}

#[test]
fn testing_1_2_3_test() {
    let empty_arr: &[&str] = &[];
    let empty_str_vec: Vec<String> = vec![];
    assert_eq!(testing_1_2_3::number(empty_arr), empty_str_vec);

    let non_empty_arr: &[&str] = &["a", "b", "c"];
    let non_empty_str_vec: Vec<String> = vec!["1: a".to_string(), "2: b".to_string(), "3: c".to_string()];
    assert_eq!(testing_1_2_3::number(non_empty_arr), non_empty_str_vec);
}

#[test]
fn duplicate_encode_test() {
  assert_eq!(duplicate_encode::duplicate_encode("din"),"(((");
  assert_eq!(duplicate_encode::duplicate_encode("recede"),"()()()");
  assert_eq!(duplicate_encode::duplicate_encode("Success"),")())())","should ignore case");
  assert_eq!(duplicate_encode::duplicate_encode("(( @"),"))((");
}

#[test]
fn min_max_test() {
  assert_eq!(min_max::min_max(&[1]), (1,1));
  assert_eq!(min_max::min_max(&[1,2,3,4,5]), (1,5));
  assert_eq!(min_max::min_max(&[2334454,5]), (5,2334454));
}

#[test]
fn is_valid_walk_test() {
    assert!(walk::is_valid_walk(&['n','s','n','s','n','s','n','s','n','s']));
    assert!(!walk::is_valid_walk(&['w','e','w','e','w','e','w','e','w','e','w','e']));
    assert!(!walk::is_valid_walk(&['w']));
    assert!(!walk::is_valid_walk(&['n','n','n','s','n','s','n','s','n','s']));
    assert!(!walk::is_valid_walk(&['e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's']))
}

#[test]
fn validate_pin_test() {
    assert_eq!(validate_pin::validate_pin("1"), false);
    assert_eq!(validate_pin::validate_pin("12"), false);
    assert_eq!(validate_pin::validate_pin("123"), false);
    assert_eq!(validate_pin::validate_pin("12345"), false);
    assert_eq!(validate_pin::validate_pin("1234567"), false);
    assert_eq!(validate_pin::validate_pin("-1234"), false);
    assert_eq!(validate_pin::validate_pin("1.234"), false);
    assert_eq!(validate_pin::validate_pin("-1.234"), false);
    assert_eq!(validate_pin::validate_pin("00000000"), false);
    assert_eq!(validate_pin::validate_pin("a234"), false);
    assert_eq!(validate_pin::validate_pin(".234"), false);
    assert_eq!(validate_pin::validate_pin("1234"), true);
    assert_eq!(validate_pin::validate_pin("0000"), true);
    assert_eq!(validate_pin::validate_pin("1111"), true);
    assert_eq!(validate_pin::validate_pin("123456"), true);
    assert_eq!(validate_pin::validate_pin("098765"), true);
    assert_eq!(validate_pin::validate_pin("000000"), true);
    assert_eq!(validate_pin::validate_pin("123456"), true);
    assert_eq!(validate_pin::validate_pin("090909"), true);
}

#[test]
fn spin_words_test() {
    assert_eq!(reverse_5::spin_words("Welcome"), "emocleW");
    assert_eq!(reverse_5::spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
    assert_eq!(reverse_5::spin_words("This is a test"), "This is a test");
    assert_eq!(reverse_5::spin_words("This is another test"), "This is rehtona test");
    assert_eq!(reverse_5::spin_words("You are almost to the last test"), "You are tsomla to the last test");
    assert_eq!(reverse_5::spin_words("Just kidding there is still one more"), "Just gniddik ereht is llits one more");
    assert_eq!(reverse_5::spin_words("Seriously this is the last one"), "ylsuoireS this is the last one");
}

#[test]
fn test_high_scoring() {
    assert_eq!(scoring_words::high("man i need a taxi up to ubud"), "taxi");               
    assert_eq!(scoring_words::high("what time are we climbing up the volcano"), "volcano");
    assert_eq!(scoring_words::high("take me to semynak"), "semynak");                      
    assert_eq!(scoring_words::high("massage yes massage yes massage"), "massage");         
    assert_eq!(scoring_words::high("take two bintang and a dance please"), "bintang"); 
    assert_eq!(scoring_words::high("aa b"), "aa");         
    assert_eq!(scoring_words::high("b aa"), "b");     
    assert_eq!(scoring_words::high("bb d"), "bb");                            
    assert_eq!(scoring_words::high("d bb"), "d"); 
    assert_eq!(scoring_words::high("aaa b"), "aaa");                                     
}
