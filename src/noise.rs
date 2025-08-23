use rand::{SeedableRng, seq::SliceRandom};
use rand_chacha::ChaCha8Rng;

use crate::{color::Color, point::Point, vec3::Vec3};

#[derive(Debug)]
pub(crate) struct Perlin {
    scale: f32,
    randvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

const POINT_COUNT: usize = 256;

impl Perlin {
    pub(crate) fn new(scale: f32, seed: u64) -> Perlin {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let randvec = (0..POINT_COUNT)
            .map(|_| Vec3::random_unit_vector(&mut rng))
            .collect();

        let mut v = (0..POINT_COUNT).collect::<Vec<_>>();
        v.shuffle(&mut rng);

        let perm_x = v.clone();

        v.shuffle(&mut rng);

        let perm_y = v.clone();

        v.shuffle(&mut rng);

        let perm_z = v.clone();

        Perlin {
            scale,
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub(crate) fn noise(&self, point: &Point) -> f32 {
        // let x = point.x.abs();
        // let y = point.y.abs();
        // let z = point.z.abs();

        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        #[allow(clippy::needless_range_loop)]
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[self.perm_x[(i + di as i32) as usize & 255]
                        ^ self.perm_y[(j + dj as i32) as usize & 255]
                        ^ self.perm_z[(k + dk as i32) as usize & 255]];
                }
            }
        }

        Self::trilinear_interpolation(&c, u, v, w)
    }

    pub(crate) fn value(&self, point: &Point) -> Color {
        Color::new(0.5, 0.5, 0.5)
            * (1.0 + (self.scale * point.z + 10.0 * self.turbulence(point, 7)).sin())
    }

    fn trilinear_interpolation(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f32;
                    let j = j as f32;
                    let k = k as f32;

                    let weight_v = Vec3::new(u - i, v - j, w - k);

                    accum += (i * u + (1.0 - i) * (1.0 - u))
                        * (j * v + (1.0 - j) * (1.0 - v))
                        * (k * w + (1.0 - k) * (1.0 - w))
                        * c[i as usize][j as usize][k as usize].dot(weight_v)
                }
            }
        }

        accum
    }

    fn turbulence(&self, point: &Point, depth: u32) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = Vec3::from(*point);
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * Self::noise(self, &temp_p.into());
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}
