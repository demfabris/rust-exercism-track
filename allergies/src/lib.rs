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
        self.score >= 2u32.pow(allergen_score)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut result: Vec<Allergen> = vec![];
        let score_as_bits: Vec<u32> = format!("{:b}", self.score)
            .chars()
            .rev()
            .map(|b| b.to_digit(2).unwrap())
            .collect();

        for (index, bit) in score_as_bits.iter().enumerate() {
            if *bit == 1 {
                match index {
                    0 => result.push(Allergen::Eggs),
                    1 => result.push(Allergen::Peanuts),
                    2 => result.push(Allergen::Shellfish),
                    3 => result.push(Allergen::Strawberries),
                    4 => result.push(Allergen::Tomatoes),
                    5 => result.push(Allergen::Chocolate),
                    6 => result.push(Allergen::Pollen),
                    7 => result.push(Allergen::Cats),
                    _ => (),
                }
            }
        }

        result
    }
}
