#[allow(dead_code)]
pub fn validate_pin(pin: &str) -> bool {
    [4, 6].contains(&pin.len()) && pin.chars().all(|ch| ch.is_digit(10))
}
