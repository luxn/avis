use rand;
use rand::{Rng, SeedableRng, StdRng, Rand};

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

    fn new_with_seed(seed: Vec<usize>) -> Random {   
        let rng = SeedableRng::from_seed(seed.as_slice());

        Random {
            rng: rng,
            seed: seed
        }
    }

    fn reseed(&mut self, seed: Vec<usize>) {
        self.rng.reseed(seed.as_slice());
        self.seed = seed;
    }

    fn next<T: Rand>(&mut self) -> T {
        self.rng.gen::<T>()
    }

}