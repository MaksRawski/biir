pub use unicode_segmentation::UnicodeSegmentation;

pub type UnicodeString<'a> = Vec<&'a str>;

pub fn string_to_unicode_string(string: &str) -> UnicodeString {
    string.graphemes(true).collect()
}
