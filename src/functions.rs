use super::structs::Total;
pub fn validate_text_input(input: &str) -> Option<String> {
    if input.is_empty() {
        Some("Input cannot be empty".to_string())
    } else {
        None
    }
}
pub fn contains_field(vec: &Vec<Total>, position: &(usize, i32)) -> bool {
    vec.iter().any(|s| s.position == *position)
}
pub fn sanitize_string(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() {
                '_'
            } else {
                '_'
            }
        })
        .collect()
}
