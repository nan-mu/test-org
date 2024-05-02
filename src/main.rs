//! An example of template matching in a greyscale image.

use image::{GrayImage, Luma};
use imageproc::definitions::Image;
use imageproc::map::map_colors;
use imageproc::template_matching::{self, MatchTemplateMethod};

fn main() {
    let image = image::open("./img.png").unwrap();
    let tmp = image::open("./tmp.png").unwrap();
    let image = image.into_luma8();
    let tmp = tmp.into_luma8();
    let res = template_matching::match_template_parallel(
        &image,
        &tmp,
        MatchTemplateMethod::SumOfSquaredErrorsNormalized,
    );
    let res = convert_to_gray_image(&res);
    res.save("./SumOfSquaredErrorsNormalized.png").unwrap();
    // let mut result_padded = GrayImage::new(res.width(), res.height());
}

fn convert_to_gray_image(image: &Image<Luma<f32>>) -> GrayImage {
    let mut lo = f32::INFINITY;
    let mut hi = f32::NEG_INFINITY;

    for p in image.iter() {
        lo = if *p < lo { *p } else { lo };
        hi = if *p > hi { *p } else { hi };
    }

    let range = hi - lo;
    let scale = |x| (255.0 * (x - lo) / range) as u8;
    map_colors(image, |p| Luma([scale(p[0])]))
}
