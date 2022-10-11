use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use std::ops::{Add, Neg};

pub trait Perturbable<T: SampleUniform + PartialOrd + Add<Output = T> + Neg<Output = T>>:
    Sized
{
    const MIN: T;
    const MAX: T;
    const MAX_DELTA: T;

    fn new_with_value_and_distribution(value: T, distribution: Uniform<T>) -> Self;
    fn get_distribution(&self) -> Uniform<T>;
    fn get_value(&self) -> T;

    fn new_with_value(value: T) -> Self {
        let distribution = Uniform::new_inclusive(-Self::MAX_DELTA, Self::MAX_DELTA);
        Self::new_with_value_and_distribution(value, distribution)
    }

    fn new_random() -> Self {
        let new_distribution = Uniform::new_inclusive(Self::MIN, Self::MAX);
        let mut rng = rand::thread_rng();
        let new_value = new_distribution.sample(&mut rng);
        Self::new_with_value(new_value)
    }

    fn perturb(&self) -> Self {
        let value = self.get_value();
        let distribution = self.get_distribution();

        let mut rng = rand::thread_rng();

        let delta = distribution.sample(&mut rng);
        let mut new_value = value + delta;

        if new_value < Self::MIN {
            new_value = Self::MIN;
        }

        if new_value > Self::MAX {
            new_value = Self::MAX;
        }

        Self::new_with_value_and_distribution(new_value, distribution)
    }
}

#[macro_export]
macro_rules! generate_perturbable {
    ($type: ident, $name: ident, $min: expr, $max: expr, $max_delta: expr) => {
        #[derive(Copy, Clone)]
        pub struct $name {
            value: $type,
            distribution: rand::distributions::Uniform<$type>,
        }

        impl Perturbable<$type> for $name {
            const MIN: $type = $min;
            const MAX: $type = $max;
            const MAX_DELTA: $type = $max_delta;

            fn new_with_value_and_distribution(
                value: $type,
                distribution: rand::distributions::Uniform<$type>,
            ) -> Self {
                Self {
                    value,
                    distribution,
                }
            }

            fn get_value(&self) -> $type {
                self.value
            }

            fn get_distribution(&self) -> rand::distributions::Uniform<$type> {
                self.distribution
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("value", &self.value)
                    .finish()
            }
        }
    };
}
