#[derive(Debug)]
#[derive(PartialEq)]
pub enum XmasLetter {
    X,
    M,
    A,
    S,
    Dot,  // Representing '.' as Dot since identifiers can't be punctuation marks
}

// Example implementation
impl XmasLetter {
    // Convert from char to XmasLetter
    pub fn from_char(c: char) -> Option<XmasLetter> {
        match c {
            'X' => Some(XmasLetter::X),
            'M' => Some(XmasLetter::M),
            'A' => Some(XmasLetter::A),
            'S' => Some(XmasLetter::S),
            '.' => Some(XmasLetter::Dot),
            _ => None,
        }
    }

    // Convert XmasLetter back to char
    pub fn to_char(&self) -> char {
        match self {
            XmasLetter::X => 'X',
            XmasLetter::M => 'M',
            XmasLetter::A => 'A',
            XmasLetter::S => 'S',
            XmasLetter::Dot => '.',
        }
    }
}