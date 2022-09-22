pub fn number(lines: &[&str]) -> Vec<String> {
    lines.iter().enumerate().map(|i_line| format!("{}: {}", i_line.0 + 1, i_line.1)).collect()
}
