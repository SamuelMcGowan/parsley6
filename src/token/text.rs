/// An ASCII character.
pub trait Ascii {
    fn is_ascii_alphabetic(&self) -> bool;
    fn is_ascii_alphanumeric(&self) -> bool;

    fn is_ascii_digit(&self) -> bool;
    fn is_ascii_hexdigit(&self) -> bool;

    fn is_ascii_space(&self) -> bool;
}

impl Ascii for u8 {
    #[inline]
    fn is_ascii_alphabetic(&self) -> bool {
        self.is_ascii_alphabetic()
    }

    #[inline]
    fn is_ascii_alphanumeric(&self) -> bool {
        self.is_ascii_alphanumeric()
    }

    #[inline]
    fn is_ascii_digit(&self) -> bool {
        self.is_ascii_digit()
    }

    #[inline]
    fn is_ascii_hexdigit(&self) -> bool {
        self.is_ascii_hexdigit()
    }

    #[inline]
    fn is_ascii_space(&self) -> bool {
        self.is_ascii_whitespace()
    }
}

impl Ascii for char {
    #[inline]
    fn is_ascii_alphabetic(&self) -> bool {
        self.is_ascii_alphabetic()
    }

    #[inline]
    fn is_ascii_alphanumeric(&self) -> bool {
        self.is_ascii_alphanumeric()
    }

    #[inline]
    fn is_ascii_digit(&self) -> bool {
        self.is_ascii_digit()
    }

    #[inline]
    fn is_ascii_hexdigit(&self) -> bool {
        self.is_ascii_hexdigit()
    }

    #[inline]
    fn is_ascii_space(&self) -> bool {
        self.is_ascii_digit()
    }
}
