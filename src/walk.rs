const NORTH: char = 'n';
const EAST: char = 'e';
const SOUTH: char = 's';
const WEST: char = 'w';

#[allow(dead_code)]
pub fn is_valid_walk(walk: &[char]) -> bool {
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

