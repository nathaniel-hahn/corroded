/*
 * Nathaniel Hahn
 * corroded
 * main
 *
 */



use corroded::cpu::filters;
use corroded::utils::image_io;

fn main() {
    let image = vec![
        10, 10, 10, 10, 10,
        10, 50, 50, 50, 10,
        10, 50, 100, 50, 10,
        10, 50, 50, 50, 10,
        10, 10, 10, 10, 10,
    ];
    let width = 5;
    let height = 5;

    // Define the kernel
    let kernel = [
        -1.0, -1.0, -1.0,
        -1.0,  8.0, -1.0,
        -1.0, -1.0, -1.0,
    ];

    // Apply convolution
    let result = filters::apply_convolution(&image, width, height, &kernel, 3);


    let edge_result = filters::apply_predefined_kernel(&image, width, height, "edge_detection");
    let blur_result = filters::apply_predefined_kernel(&image, width, height, "gaussian_blur");
    let sharpen_result = filters::apply_predefined_kernel(&image, width, height, "sharpen");
    let emboss_result = filters::apply_predefined_kernel(&image, width, height, "emboss");


    // Print the resulting image
    println!("Result: {:?}", result);
    println!("Edge detection result: {:?}", edge_result);
    println!("Gaussian blur result: {:?}", blur_result);
    println!("Sharpen result: {:?}", sharpen_result);
    println!("Emboss result: {:?}", emboss_result);
}
