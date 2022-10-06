#[allow(dead_code)]
fn number(lines: &[&str]) -> Vec<String> {
    lines.iter().enumerate().map(|i_line| format!("{}: {}", i_line.0 + 1, i_line.1)).collect()
}

#[test]
fn testing_1_2_3_test() {
    let empty_arr: &[&str] = &[];
    let empty_str_vec: Vec<String> = vec![];
    assert_eq!(number(empty_arr), empty_str_vec);

    let non_empty_arr: &[&str] = &["a", "b", "c"];
    let non_empty_str_vec: Vec<String> = vec!["1: a".to_string(), "2: b".to_string(), "3: c".to_string()];
    assert_eq!(number(non_empty_arr), non_empty_str_vec);
}
