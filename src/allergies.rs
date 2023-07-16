/*
Given a person's allergy score, determine whether or not they're allergic to a given item, and their full list of allergies.
An allergy test produces a single numeric score which contains the information about all the allergies the person has (that they were tested for).
The list of items (and their value) that were tested are:
    eggs (1)
    peanuts (2)
    shellfish (4)
    strawberries (8)
    tomatoes (16)
    chocolate (32)
    pollen (64)
    cats (128)
So if Tom is allergic to peanuts and chocolate, he gets a score of 34.
Now, given just that score of 34, your program should be able to say:
    Whether Tom is allergic to any one of those allergens listed above.
    All the allergens Tom is allergic to.
Note: a given score may include allergens not listed above (i.e. allergens that score 256, 512, 1024, etc.). Your program should ignore those components of the score. For example, if the allergy score is 257, your program should only report the eggs (1) allergy.

*/
pub struct Allergies {
    value: u32,
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

const ALLALERGEGIES: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        return Allergies { value: score };
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (self.value & *allergen as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLALERGEGIES
            .into_iter()
            .filter(|x| self.is_allergic_to(x))
            .collect()
    }
}

#[cfg(test)]
mod allergies_tests {

    use crate::allergies::*;
    fn compare_allergy_vectors(expected: &[Allergen], actual: &[Allergen]) {
        for element in expected {
            if !actual.contains(element) {
                panic!("Allergen missing\n  {element:?} should be in {actual:?}");
            }
        }
        if actual.len() != expected.len() {
            panic!(
            "Allergy vectors are of different lengths\n  expected {expected:?}\n  got {actual:?}"
        );
        }
    }
    #[test]
    fn is_not_allergic_to_anything() {
        let allergies = Allergies::new(0);
        assert!(!allergies.is_allergic_to(&Allergen::Peanuts));
        assert!(!allergies.is_allergic_to(&Allergen::Cats));
        assert!(!allergies.is_allergic_to(&Allergen::Strawberries));
    }
    #[test]
    fn is_allergic_to_eggs() {
        assert!(Allergies::new(1).is_allergic_to(&Allergen::Eggs));
    }
    #[test]
    fn is_allergic_to_eggs_and_shellfish_but_not_strawberries() {
        let allergies = Allergies::new(5);
        assert!(allergies.is_allergic_to(&Allergen::Eggs));
        assert!(allergies.is_allergic_to(&Allergen::Shellfish));
        assert!(!allergies.is_allergic_to(&Allergen::Strawberries));
    }
    #[test]
    fn no_allergies_at_all() {
        let expected = &[];
        let allergies = Allergies::new(0).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
    #[test]
    fn allergic_to_just_eggs() {
        let expected = &[Allergen::Eggs];
        let allergies = Allergies::new(1).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
    #[test]
    fn allergic_to_just_peanuts() {
        let expected = &[Allergen::Peanuts];
        let allergies = Allergies::new(2).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
    #[test]
    fn allergic_to_just_strawberries() {
        let expected = &[Allergen::Strawberries];
        let allergies = Allergies::new(8).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
    #[test]
    fn allergic_to_eggs_and_peanuts() {
        let expected = &[Allergen::Eggs, Allergen::Peanuts];
        let allergies = Allergies::new(3).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
    #[test]
    fn allergic_to_eggs_and_shellfish() {
        let expected = &[Allergen::Eggs, Allergen::Shellfish];
        let allergies = Allergies::new(5).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
    #[test]
    fn allergic_to_many_things() {
        let expected = &[
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];
        let allergies = Allergies::new(248).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
    #[test]
    fn allergic_to_everything() {
        let expected = &[
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];
        let allergies = Allergies::new(255).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
    #[test]
    fn scores_over_255_do_not_trigger_false_positives() {
        let expected = &[
            Allergen::Eggs,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];
        let allergies = Allergies::new(509).allergies();
        compare_allergy_vectors(expected, &allergies);
    }
}
