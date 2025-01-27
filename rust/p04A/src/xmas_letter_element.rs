use super::xmas_letter::XmasLetter;

#[derive(Debug)]
pub struct XmasLetterElement {
    pub letter: XmasLetter,
    pub word_count: usize,
}

impl XmasLetterElement {
    // Constructor for creating new XmasLetterElement instances
    pub fn new(letter: XmasLetter) -> Self {
        XmasLetterElement {
            letter,
            word_count: 0
        }
    }
}