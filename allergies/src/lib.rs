pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Clone)]
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

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_score = allergen.to_owned() as u32;
        self.score >= allergen_score
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let score = self.score;
        let possible_allergens: Vec<Allergen> = vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ]
        .into_iter()
        .filter(|x| x.to_owned() as u32 <= score)
        .collect();
    }
}
