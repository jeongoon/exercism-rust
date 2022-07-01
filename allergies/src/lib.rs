use self::Allergen::*;

pub struct Allergies(u32);
#[derive(Debug, PartialEq, Copy, Clone)]
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

// trying some new style but this is maybe not effienct.
// I would go for `closure` in most of times.
impl From<&Allergen> for Allergies {
    fn from(allergen: &Allergen) -> Self {
        Self(1u32 << (*allergen as u8))
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score)
    }

    pub fn get_value(&self) -> u32 {
        self.0
    }

    pub fn iter() -> impl Iterator<Item = Allergen> {
        [
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ]
        .iter()
        .copied() // important!
    }
    pub fn is_allergic_to<T: Into<Allergies>>(&self, allergen: T) -> bool {
        (self.get_value() & allergen.into().get_value()) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergies::iter()
            .filter(|a| self.is_allergic_to(a))
            .collect::<Vec<Allergen>>()
    }
}
