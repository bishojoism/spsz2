use std::hash::{DefaultHasher, Hash, Hasher};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::seq::SliceRandom;

pub fn shuffle(count: usize, seed: &str) -> Vec<usize> {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let mut rng = StdRng::seed_from_u64(hasher.finish());
    let mut result = (0..count).collect::<Vec<_>>();
    result.shuffle(&mut rng);
    result
}