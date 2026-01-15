use rand::distr::{Alphanumeric, SampleString};
use rand::rng;

pub struct EntropyGenerator {}

impl EntropyGenerator {
    const ENTROPY_LENGTH: usize = 200_usize;

    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self) -> String {
        let mut random_generator = rng();
        Alphanumeric.sample_string(&mut random_generator, Self::ENTROPY_LENGTH)
    }
}
