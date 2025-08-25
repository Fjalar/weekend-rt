use std::slice::Iter;

use rand::distr::{Distribution, StandardUniform};

#[derive(Debug, Clone, Copy)]
pub(crate) enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub(crate) fn iter() -> Iter<'static, Axis> {
        static AXES: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];
        AXES.iter()
    }

    #[allow(dead_code)]
    const ALL: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];
}

impl Distribution<Axis> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Axis {
        match rng.random_range(0..=2) {
            0 => Axis::X,
            1 => Axis::Y,
            _ => Axis::Z,
        }
    }
}
