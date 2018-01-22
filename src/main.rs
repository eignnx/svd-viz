#[macro_use]
extern crate rulinalg;
extern crate image;
extern crate imageproc;

use image::DynamicImage;
use rulinalg::matrix::{Matrix, BaseMatrix};

fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // let filename = args.get(1).unwrap();
    // let original_img = image::open(filename).unwrap();

    let a = Matrix::new(3, 2, vec![
        1.0, 2.0,
        3.0, 4.0,
        5.0, 6.0
    ]);

    let b = &a + &a;
    let (sigma, u, v) = a.svd().expect("SVD failed!");

    println!("{}", b);
    println!("\nsigma =\n{}\nu =\n{}\nv =\n{}", sigma, u, v);
    println!("u . sigma . v^T =\n{}", &u * &sigma * v.transpose());
}
