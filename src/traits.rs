use {crate::TextSize, std::convert::TryInto};

/// Text-like structures that have a text size.
pub trait TextSized: Copy {
    /// The size of this text-alike.
    fn text_size(self) -> TextSize;
}

impl TextSized for &'_ str {
    fn text_size(self) -> TextSize {
        self.len()
            .try_into()
            .unwrap_or_else(|| panic!("string too large ({}) for TextSize", self.len()))
    }
}

impl TextSized for char {
    fn text_size(self) -> TextSize {
        TextSize(self.len_utf8() as u32)
    }
}
