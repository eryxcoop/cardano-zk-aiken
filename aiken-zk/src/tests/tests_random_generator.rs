use crate::entropy_generator::EntropyGenerator;

#[test]
fn test_minimum_length_of_random_for_verification_key_entropy() {
    let entropy_generator = EntropyGenerator::new();
    let entropy = entropy_generator.generate();
    assert!(entropy.len() >= 200);
    assert!(entropy.chars().all(|character| character.is_alphanumeric()));
}

#[test]
fn test_random_for_verification_key_entropy_is_alphanumeric() {
    let entropy_generator = EntropyGenerator::new();
    let entropy = entropy_generator.generate();
    assert!(entropy.chars().all(|character| character.is_alphanumeric()));
}