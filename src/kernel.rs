use std::ops::Index;

#[derive(Debug)]
pub struct Kernel {
    pub kernel: Vec<f32>,
    pub width: u32,
    pub height: u32,
}
impl Kernel {
    pub fn from_vec_vec(kernel: Vec<Vec<f32>>) -> Kernel {
        let width = kernel[0].len() as u32;
        let height = kernel.len() as u32;
        let kernel = kernel.into_iter().flatten().collect();
        Kernel { kernel, width, height }
    }
    pub fn normalize(&mut self) {
        let sum: f32 = self.kernel.iter().sum();
        for i in 0..self.kernel.len() {
            self.kernel[i] /= sum;
        }
    }
    pub fn transpose(&self) -> Kernel {
        let mut kernel = vec![0.0; self.kernel.len()];
        for x in 0..self.width {
            for y in 0..self.height {
                kernel[(x * self.height + y) as usize] = self[(x, y)];
            }
        }
        Kernel {
            kernel,
            width: self.height,
            height: self.width,
        }
    }
}

impl Index<(u32, u32)> for Kernel {
    type Output = f32;

    fn index(&self, (x, y): (u32, u32)) -> &f32 {
        &self.kernel[(y * self.width + x) as usize]
    }
}

impl Kernel {
    pub fn new_gaussian(shape : (u32, u32), sigma : f32) -> Kernel {
        let mut kernel = vec![0.0; (shape.0 * shape.1) as usize];
        let mut sum = 0.0;
        for x in 0..shape.0 {
            for y in 0..shape.1 {
                let i = (y * shape.0 + x) as usize;
                kernel[i] = (-(((x as i32 - shape.0 as i32 / 2).pow(2) + (y as i32 - shape.1 as i32 / 2).pow(2)) as f32)/(2.0*sigma.powf(2.0))).exp();
                sum += kernel[i];
            }
        }
        for i in 0..kernel.len() {
            kernel[i] /= sum;
        }
        Kernel {
            kernel,
            width: shape.0,
            height: shape.1,
        }
    }
}