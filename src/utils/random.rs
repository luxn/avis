/*
use rand;
use rand::prelude::*;
use rand::{Rng, SeedableRng, StdRng, R};

pub struct Random {
    rng: StdRng,
    seed: Vec<usize>
}

impl Random {

    fn new() -> Random {
        let seed = vec![rand::random::<usize>(), rand::random::<usize>(), rand::random::<usize>(), rand::random::<usize>()];
        let rng = SeedableRng::from_seed(seed.as_slice());

        Random {
            rng: rng,
            seed: seed
        }
    }

    fn new_with_seed(seed: &[usize]) -> Random {
        let rng = SeedableRng::from_seed(seed);

        Random {
            rng,
            seed: Vec::from(seed)
        }
    }

    fn reseed(&mut self, seed: &[usize]) {
        self.rng.
        self.rng.reseed(seed);
        self.seed = Vec::from(seed);
    }

    fn next<T>(&mut self) -> T {
        self.rng.gen()
    }

}
*/