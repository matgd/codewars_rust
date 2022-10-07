#[allow(dead_code)]
fn parts_sums(ls: &[u64]) -> Vec<u64> {
    let mut sums: Vec<u64> = vec![0];

    let mut current_sum: u64 = 0;
    for desc_i in (0..ls.len()).rev() {
        current_sum += ls[desc_i];
        sums.push(current_sum);
    }

    // https://stackoverflow.com/a/42395836
    sums.reverse();
    sums
}

#[test]
fn example() {
    assert_eq!(parts_sums(&vec![0, 1, 3, 6, 10]), vec![20, 20, 19, 16, 10, 0]);
    assert_eq!(parts_sums(&vec![]), vec![0]);
    assert_eq!(parts_sums(&vec![1, 2, 3, 4, 5, 6]), vec![21, 20, 18, 15, 11, 6, 0]);
    assert_eq!(parts_sums(&vec![744125, 935, 407, 454, 430, 90, 144, 6710213, 889, 810, 2579358]),
        vec![10037855, 9293730, 9292795, 9292388, 9291934, 9291504, 9291414, 9291270, 2581057, 2580168, 2579358, 0]); 
}

