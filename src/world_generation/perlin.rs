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
use rand::distributions::Open01;
use std::iter::zip;
use std::collections::HashMap;

pub struct PerlinNoise {
    seed: u64,
    gradients: HashMap<[i64; 2], [f32; 2]>,
    scale: f32,
}

impl PerlinNoise {
    pub fn new(seed: u64, scale: f32) -> Self {
	Self { seed, gradients: HashMap::new(), scale }
    }

    pub fn at(&mut self, coord: [f32; 2]) -> f32 {
	let [xc, yc] = self.closest_corner(coord);
	let corners = [
	    [xc, yc],
	    [xc+1, yc],
	    [xc, yc+1],
	    [xc+1, yc+1],
	];

	let [xf, yf] = self.coord_to_fractional_space(coord);
	let [xr, yr] = [xf - xc as f32, yf - yc as f32];
	
	let corner_gradients = corners
	    .map(|corner| self.get_or_generate_gradient(&corner).clone());
	
	let values: Vec<_> = zip(corners, corner_gradients)
	    .map(|([xc2, yc2], gradient)|
		 dot(&[xf - xc2 as f32, yf - yc2 as f32], &gradient))
	    .collect();
	
	lerp(
            fade(yr),
            lerp(fade(xr), values[0], values[1]),
            lerp(fade(xr), values[2], values[3])
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
	
	self.gradients.entry(corner.clone())
	    .or_insert_with_key(|coord| random_gradient(coord, seed))
    }

}

fn random_gradient(coord: &[i64; 2], seed: u64) -> [f32; 2] {
    let mut specific_seed = [0u8; 32];
    specific_seed[..8].copy_from_slice(&seed.to_be_bytes());
    specific_seed[8..16].copy_from_slice(&coord[0].to_be_bytes());
    specific_seed[16..24].copy_from_slice(&coord[1].to_be_bytes());

    // TODO currently, the world generation is nondeterministic
    // let mut rng = SmallRng::from_seed(specific_seed);
    let mut rng = SmallRng::from_entropy();

    // TODO uniform distributions do not yield uniform normalized vectors !
    // Should use either gaussians, or a polar representation
    let x = rng.sample::<f32, Open01>(Open01);
    let y = rng.sample::<f32, Open01>(Open01);
    let r = norm(&[x, y]);

    [x / r, y / r]
}

/// Courtesy of the original Perlin noise implementation by Perlin
fn fade(t: f32) -> f32 {
    t*t*t * (t * (t * 6. - 15.) + 10.)
}

fn lerp(t: f32, a: f32, b: f32) -> f32{
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
	let u = random_gradient(&[0, 0], 42);
	let v = random_gradient(&[0, 0], 42);
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
	    assert_eq!(noise.coord_to_fractional_space([1.5, 2.5]), [1.5 / 8., 2.5 / 8.])
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
}
