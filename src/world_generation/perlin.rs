use rand::distributions::Open01;
/// We need a Perlin noise generator, with a few characteristics:
///
/// - it is not a one shot thing: we must be able to generate on demand more
///   noise
/// - it must be infinite
/// - we must be able to spawn a few different instances
/// - we must be able to change the frequency of the noise
/// - it must be **fully** deterministic, in the sense that generating noise in
///   a different order will not change the outcome
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use std::iter::zip;

/// Number of scales to use in the Perlin Noise
const SCALE_LEVEL_PERLIN: usize = 5;

/// Class containing the different scales of Perlin noise,
/// combines them to return a single value for each querried coord.
pub struct MultiscalePerlinNoise {
    perlin_noises: Vec<PerlinNoise>,
    amplitudes: [f32; SCALE_LEVEL_PERLIN],
}

impl MultiscalePerlinNoise {
    /// Create a new MultiscalePerlinNoise, requires the level of scales and amplitudes associated.
    /// These values will change the world aspect.
    pub fn new(
        seed: u32,
        scales: [f32; SCALE_LEVEL_PERLIN],
        amplitudes: [f32; SCALE_LEVEL_PERLIN],
    ) -> Self {
        let mut temp_perlin_noises: Vec<PerlinNoise> = Vec::new();

        for i in 0..SCALE_LEVEL_PERLIN {
            let complete_seed = (seed as u64) + (i as u64);

            temp_perlin_noises.push(PerlinNoise::new(complete_seed, scales[i]))
        }

        Self {
            perlin_noises: temp_perlin_noises,
            amplitudes,
        }
    }

    /// Returns the noise value at a given coordinate of the world
    pub fn at(&mut self, coord: [f32; 2]) -> f32 {
        let mut value: f32 = 0.0;

        for i in 0..SCALE_LEVEL_PERLIN {
            value += self.amplitudes[i] * self.perlin_noises[i].at(coord);
        }

        value
    }
}

/// Single scale Perlin noise
pub struct PerlinNoise {
    seed: u64,
    gradients: HashMap<[i64; 2], [f32; 2]>,
    scale: f32,
}

impl PerlinNoise {
    pub fn new(seed: u64, scale: f32) -> Self {
        Self {
            seed,
            gradients: HashMap::new(),
            scale,
        }
    }

    /// Returns the noise at a given world coordinate.
    pub fn at(&mut self, coord: [f32; 2]) -> f32 {
        let [xc, yc] = self.closest_corner(coord);
        let corners = [[xc, yc], [xc + 1, yc], [xc, yc + 1], [xc + 1, yc + 1]];

        let [xf, yf] = self.coord_to_fractional_space(coord);
        let [xr, yr] = [xf - xc as f32, yf - yc as f32];

        let corner_gradients = corners.map(|corner| self.get_or_generate_gradient(&corner).clone());

        let values: Vec<_> = zip(corners, corner_gradients)
            .map(|([xc2, yc2], gradient)| dot(&[xf - xc2 as f32, yf - yc2 as f32], &gradient))
            .collect();

        lerp(
            fade(yr),
            lerp(fade(xr), values[0], values[1]),
            lerp(fade(xr), values[2], values[3]),
        )
    }

    /// Change the coordinates to the noise-specific coordinate system, e.g. 0.5
    /// for noise.scale / 2
    fn coord_to_fractional_space(&self, coord: [f32; 2]) -> [f32; 2] {
        [coord[0] / self.scale, coord[1] / self.scale]
    }

    fn closest_corner(&self, coord: [f32; 2]) -> [i64; 2] {
        let [x, y] = self.coord_to_fractional_space(coord);

        [x.floor() as i64, y.floor() as i64]
    }

    fn get_or_generate_gradient(&mut self, corner: &[i64; 2]) -> &[f32; 2] {
        let seed = self.seed;

        self.gradients
            .entry(corner.clone())
            .or_insert_with_key(|coord| random_gradient(coord, seed))
    }
}

/// Returns a deterministic random gradient for a given coord and seed
fn random_gradient(coord: &[i64; 2], seed: u64) -> [f32; 2] {
    let mut specific_seed = [0u8; 32];
    specific_seed[..8].copy_from_slice(&seed.to_be_bytes());
    specific_seed[8..16].copy_from_slice(&coord[0].to_be_bytes());
    specific_seed[16..24].copy_from_slice(&coord[1].to_be_bytes());

    // TODO currently, the world generation is nondeterministic
    let mut rng = SmallRng::from_seed(specific_seed);

    // Need to discard the first value because it is too similar to the seed
    // i.e. not random
    rng.sample::<f32, Open01>(Open01);

    // TODO uniform distributions do not yield uniform normalized vectors !
    // Should use either gaussians, or a polar representation
    let x = rng.sample::<f32, Open01>(Open01);
    let y = rng.sample::<f32, Open01>(Open01);
    let r = norm(&[x, y]);

    [x / r, y / r]
}

/// Courtesy of the original Perlin noise implementation by Perlin
fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6. - 15.) + 10.)
}

fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

fn dot(u: &[f32; 2], v: &[f32; 2]) -> f32 {
    u[0] * v[0] + u[1] * v[1]
}

fn norm(v: &[f32; 2]) -> f32 {
    dot(v, v).sqrt()
}

#[cfg(test)]
mod tests {
    use crate::world_generation::perlin::*;

    #[test]
    fn test_normalized_gradient() {
        let v = random_gradient(&[0, 0], 42);
        assert!((norm(&v) - 1.).abs() < 1e-5)
    }

    #[test]
    #[ignore] // TODO remove this when determinism is ensured
    fn test_reproducible_gradient() {
        let u = random_gradient(&[1, 250], 42);
        let v = random_gradient(&[1, 250], 42);
        assert_eq!(u, v)
    }

    #[test]
    fn test_fractional_space() {
        {
            let mut noise = PerlinNoise::new(42, 1.);
            assert_eq!(noise.coord_to_fractional_space([1.5, 2.5]), [1.5, 2.5])
        }
        {
            let mut noise = PerlinNoise::new(42, 8.);
            assert_eq!(
                noise.coord_to_fractional_space([1.5, 2.5]),
                [1.5 / 8., 2.5 / 8.]
            )
        }
    }

    #[test]
    fn test_closest_corner() {
        {
            let mut noise = PerlinNoise::new(42, 1.);
            assert_eq!(noise.closest_corner([1.5, 2.1]), [1, 2])
        }
        {
            let mut noise = PerlinNoise::new(42, 4.);
            assert_eq!(noise.closest_corner([1.5, 6.1]), [0, 1])
        }
    }

    #[test]
    fn test_determinism_multiscale() {
        let mut noise = MultiscalePerlinNoise::new(
            42,
            [64.0, 40.0, 32.0, 24.0, 8.0],
            [1.0, 0.7, 0.3, 0.1, 0.7],
        );

        let test_coords: [f32; 2] = [0.0, 1.0];

        let noise_a = noise.at(test_coords);
        let noise_b = noise.at(test_coords);

        assert_eq!(noise_a, noise_b);
    }

    #[test]
    fn show_various_gradients() {
        let seed: u64 = 42;
        let x0: i64 = 0;
        let y0: i64 = 0;

        let mut last_grad: [f32; 2] = [0.0, 0.0];

        for i in -2..3 {
            for j in -2..3 {
                let x = x0 + i;
                let y = y0 + j;
                let grad = random_gradient(&[x, y], seed);

                assert_ne!(grad[0], last_grad[0]);
                assert_ne!(grad[1], last_grad[1]);

                last_grad = grad.clone();

                print!("({}; {}) - {}    ", &i, &j, &grad[0]);
            }
            print!("\n");
        }
    }
}
