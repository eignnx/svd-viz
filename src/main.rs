#[macro_use]
extern crate rulinalg;
extern crate image;
//extern crate imageproc;

use image::{DynamicImage, GenericImage, Pixel};
use rulinalg::matrix::{Matrix, BaseMatrix};

fn img_rgb_mats(img: &DynamicImage) -> (Matrix<u8>, Matrix<u8>, Matrix<u8>) {
    let (w, h) = (img.width() as usize, img.height() as usize);

    let r = Matrix::from_fn(h, w, |x, y| img.get_pixel(x as u32, y as u32).to_rgb().channels4().0);
    let g = Matrix::from_fn(h, w, |x, y| img.get_pixel(x as u32, y as u32).to_rgb().channels4().1);
    let b = Matrix::from_fn(h, w, |x, y| img.get_pixel(x as u32, y as u32).to_rgb().channels4().2);

    (r, g, b)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = match args.get(1) {
        Some(x) => x,
        None => "bread.jpeg"
    };
    let original_img = image::open(filename).unwrap();

    // let a = Matrix::new(3, 2, vec![
    //     1.0, 2.0,
    //     3.0, 4.0,
    //     5.0, 6.0
    // ]);

    // let b = &a + &a;
    // let (sigma, u, v) = a.svd().expect("SVD failed!");

    // println!("{}", b);
    // println!("\nsigma =\n{}\nu =\n{}\nv =\n{}", sigma, u, v);
    // println!("u . sigma . v^T =\n{}", &u * &sigma * v.transpose());

    // let mat = image_to_matrix(&original_img);
    // println!("mat =\n{}", mat);
    
    // let pxs = original_img.raw_pixels();
    // let (w, h) = (original_img.width(), original_img.height());
    // println!("pxs.len() = {}", pxs.len());
    // println!("h * w * 3 == pxs.len(): {}", 3 * w * h == (pxs.len() as u32));

    let (r, g, b) = img_rgb_mats(&original_img);
    println!("r.cols(): {}", r.cols());
    println!("r.rows(): {}", r.rows());
    println!("img dims: {:?}", (original_img.width(), original_img.height()));
}
