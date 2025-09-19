pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
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
        if score <= 128 {
            Allergies { score }
        } else {
            let base: u32 = 2;
            let s: u32 = if 31 - score.leading_zeros() > 7 {
                score - base.pow(31 - score.leading_zeros())
            } else {
                score % 128 + 128
            };
            Allergies { score: s }
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut result: Vec<Allergen> = Vec::new();
        
        if self.score == 0 { return result; }
        
        let base: u32 = 2;
        let mut score = self.score;
        
        while score != 0 {
            let power = 31 - score.leading_zeros();
            
            match power {
                0 => result.push(Allergen::Eggs),
                1 => result.push(Allergen::Peanuts),
                2 => result.push(Allergen::Shellfish),
                3 => result.push(Allergen::Strawberries),
                4 => result.push(Allergen::Tomatoes),
                5 => result.push(Allergen::Chocolate),
                6 => result.push(Allergen::Pollen),
                7 => result.push(Allergen::Cats),
                _ => ()
            }
        
            score -= base.pow(power);
        }
            
        result.reverse();
        result
    }
}
