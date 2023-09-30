pub(crate) trait Useful {
    fn get_substring(&mut self, separator: &str) -> Option<&str>;
}

impl Useful for String {
    fn get_substring(&mut self, separator: &str) -> Option<&str> {
        if let Some(index) = self.find(separator) {
            Some(&self[index + separator.len()..])
        } else {
            None
        }
    }
}