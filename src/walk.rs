const NORTH: char = 'n';
const EAST: char = 'e';
const SOUTH: char = 's';
const WEST: char = 'w';

#[allow(dead_code)]
fn is_valid_walk(walk: &[char]) -> bool {
    // Takes 10 minutes?
    if walk.len() != 10 {
        return false;
    }

    // Check if vectors sum to 0
    let plus_x = walk.iter().filter(|&ch| ch == &NORTH).count();
    let minus_x = walk.iter().filter(|&ch| ch == &SOUTH).count();

    if plus_x != minus_x {
        return false;
    }

    let plus_y = walk.iter().filter(|&ch| ch == &EAST).count();
    let minus_y = walk.iter().filter(|&ch| ch == &WEST).count();

    if plus_y != minus_y {
        return false;
    }

    true
}

#[test]
fn is_valid_walk_test() {
    assert!(is_valid_walk(&['n','s','n','s','n','s','n','s','n','s']));
    assert!(!is_valid_walk(&['w','e','w','e','w','e','w','e','w','e','w','e']));
    assert!(!is_valid_walk(&['w']));
    assert!(!is_valid_walk(&['n','n','n','s','n','s','n','s','n','s']));
    assert!(!is_valid_walk(&['e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's']))
}

