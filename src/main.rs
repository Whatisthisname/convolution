pub mod kernel;
use std::env;

use crate::kernel::Kernel;

use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, ImageBuffer};

fn main() {
    let img = load_image("./source.png");

    let args: Vec<String> = env::args().map(|s| s.trim().to_owned()).collect();
    
    let (sigma, size) = {
        let sigma = args[1].parse::<f32>().unwrap();
        let size = args[2].parse::<u32>().unwrap();
        (sigma, size)
    };

    let name = format!("blur_{}s_{}k.png", sigma, size);


    // // doing convolution in two steps in parallel:

    // let kernel = Kernel::new_gaussian((1, size), sigma);

    // let now = std::time::Instant::now();
    // let convolved = convolve_rayon(&img, &kernel);
    // let kernel = kernel.transpose();
    // let convolved = convolve_rayon(&convolved, &kernel);
    // println!("convolved rayon = {}s", now.elapsed().as_secs_f64());
    // convolved.save("./canny_rayon.png").unwrap();




    // doing whole convolution at once:

    let kernel = Kernel::new_gaussian((size, size), sigma);

    let now = std::time::Instant::now();
    let convolved = convolve(&img, &kernel);
    println!("convolved quadratic  = {}s", now.elapsed().as_secs_f64());
    convolved.save(format!("naïve_{}", name)).unwrap();

    // separating kernel into two:

    let kernel = Kernel::new_gaussian((1, size), sigma);

    let now = std::time::Instant::now();
    let convolved = convolve(&img, &kernel);
    let kernel = kernel.transpose();
    let convolved = convolve(&convolved, &kernel);
    println!("convolved decomposed = {}s", now.elapsed().as_secs_f64());
    convolved.save(format!("smart_{}", name)).unwrap();
}

fn load_image(path: &str) -> DynamicImage {
    ImageReader::open(path).unwrap().decode().unwrap()
}

fn convolve(img: &DynamicImage, kernel: &Kernel) -> DynamicImage {
    let mut new_img = ImageBuffer::new(img.width(), img.height());

    let x_margin: u32 = kernel.width / 2;
    let y_margin: u32 = kernel.height / 2;

    for y in y_margin..(img.height() - y_margin - (if kernel.height % 2 == 0 { 1 } else { 0 })) {
        for x in x_margin..(img.width() - x_margin - (if kernel.width % 2 == 0 { 1 } else { 0 })) {
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;
            for kx in 0..kernel.width {
                for ky in 0..kernel.height {
                    let pixel = img.get_pixel(x + kx - x_margin, y + ky - y_margin);
                    let k = kernel[(kx, ky)];
                    r += pixel[0] as f32 * k;
                    g += pixel[1] as f32 * k;
                    b += pixel[2] as f32 * k;
                }
            }
            new_img.put_pixel(x, y, image::Rgb([r as u8, g as u8, b as u8]));
        }
    }
    DynamicImage::from(new_img)
}

// use rayon::prelude::*;

// fn convolve_rayon(img: &DynamicImage, kernel: &Kernel) -> DynamicImage {
//     let x_margin: u32 = kernel.width / 2;
//     let y_margin: u32 = kernel.height / 2;

//     let k = (x_margin..(img.width() - x_margin - (if kernel.width % 2 == 0 {1} else {0})))
//         .into_par_iter()
//         .map(|x| {
//             let mut res = Vec::with_capacity(img.height() as usize - kernel.width as usize +1);
//             for y in y_margin..(img.height() - y_margin - (if kernel.height % 2 == 0 {1} else {0})) {
//                 let mut r = 0.0;
//                 let mut g = 0.0;
//                 let mut b = 0.0;
//                 for kx in 0..kernel.width {
//                     for ky in 0..kernel.height {
//                         let pixel = img.get_pixel(x + kx - x_margin, y + ky - y_margin);
//                         let k = kernel[(kx, ky)];
//                         r += pixel[0] as f32 * k;
//                         g += pixel[1] as f32 * k;
//                         b += pixel[2] as f32 * k;
//                     }
//                 }
//                 res.push([r as u8, g as u8, b as u8]);
//             }
//             res
//         });

//     let buf = ImageBuffer::<Rgb<u8>, &[u8;3]>::from_raw(img.width(), img.height(), k.flatten().collect::<Vec<&[u8;3]>>());
//     match buf {
//         Some(buf) => DynamicImage::ImageRgb8(buf),
//         None => panic!("failed to create image buffer"),
//     }
//     DynamicImage::from(buf)
// }
