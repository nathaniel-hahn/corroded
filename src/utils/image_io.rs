/*
 * Nathaniel Hahn
 * corroded
 * image io
 *
 */

use image::{DynamicImage, GrayImage};

pub fn load_image(path: &str) -> GrayImage {
    let img = image::open(path).expect("Failed to load image");
    img.to_luma8()
}
