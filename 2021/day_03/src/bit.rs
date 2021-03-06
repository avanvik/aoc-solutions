use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Bit {
    Zero,
    One,
}

impl Bit {
    pub fn flip(&mut self) -> Self {
        match self {
            Bit::One => Bit::Zero,
            Bit::Zero => Bit::One,
        }
    }
}

impl std::fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn bool_to_bit(b: &Bit) -> &'static str {
            match b {
                Bit::One => "1",
                Bit::Zero => "0",
            }
        }

        write!(f, "{}", bool_to_bit(self))
    }
}

pub type Bitvec = Vec<Bit>;
