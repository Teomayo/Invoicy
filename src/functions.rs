use eframe::egui;

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
pub fn load_icon(path: &str) -> egui::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        (image.into_raw(), width, height)
    };
    egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sanitize_string() {
        assert_eq!(sanitize_string("Hey There 1 2 3 $"), "hey_there_1_2_3__");
    }
    #[test]
    fn test_validate_text_input() {
        assert_eq!(
            validate_text_input(""),
            Some("Input cannot be empty".to_string())
        );
    }
}
