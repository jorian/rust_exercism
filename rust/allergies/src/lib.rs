#[macro_use]
extern crate strum_macros;
extern crate strum;

use strum::IntoEnumIterator;

#[derive(Clone, Copy, PartialEq, Debug, EnumIter)]
pub enum Allergen {
    Eggs           = 1 << 0,
    Peanuts        = 1 << 1,
    Shellfish      = 1 << 2,
    Strawberries   = 1 << 3,
    Tomatoes       = 1 << 4,
    Chocolate      = 1 << 5,
    Pollen         = 1 << 6,
    Cats           = 1 << 7,
}

pub struct Allergies {
    score: u8,
}

impl Allergies {
    pub fn new(score: u8) -> Allergies {
        Allergies { score }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut output = Vec::new();

        for i in 0..8 {
            match self.score & (1<<i) != 0 {
                true  => output.push(Allergen::iter().nth(i).unwrap()),
                _     => ()
            }
        }

        output
    }

    pub fn is_allergic_to(&self, alg: &Allergen) -> bool {
        self.score & (*alg as u8) != 0
    }
}