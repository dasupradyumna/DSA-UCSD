use rand::distributions::{uniform, Distribution, Uniform};
use rand::rngs;

pub struct Rng<'a, T>
where
    T: uniform::SampleUniform,
{
    rng: &'a mut rngs::ThreadRng,
    dists: Vec<Uniform<T>>,
}
impl<'a, T> Rng<'a, T>
where
    T: uniform::SampleUniform,
{
    pub fn new(rng: &'a mut rngs::ThreadRng) -> Self {
        Self { rng, dists: vec![] }
    }

    pub fn add_dist(&mut self, lower: T, upper: T) {
        self.dists.push(Uniform::from(lower..=upper));
    }

    pub fn from_dist(&mut self, i: usize) -> T {
        self.dists[i].sample(self.rng)
    }

    pub fn create_vec(&mut self, len: usize, i: usize) -> Vec<T> {
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(self.from_dist(i));
        }
        vec
    }
}
