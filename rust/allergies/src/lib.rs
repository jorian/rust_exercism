

pub struct Allergies {
    score: u8
}

impl Allergies {
    pub fn new(score: u8) -> Allergies {
        Allergies { score }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let result = vec![8; Allergen];
    }

    pub fn is_allergic_to(&self, alg: &Allergen) -> bool {

    }
}

pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

fn check_score(score: u8) -> u8 {
    match score {
        score if score << 0 => 1,

    }
}

pub fn is_allergic_to(a: &Allergen) -> bool {

}

pub fn contains(a: &Allergen) -> bool {

}

// https://www.includehelp.com/c/how-to-check-a-particular-bit-is-set-or-not-using-c-program.aspx