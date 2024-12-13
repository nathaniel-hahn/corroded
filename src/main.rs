/*
 * Nathaniel Hahn
 * corroded
 * main
 *
 */

// itty bitty update for git testing

use corroded::cpu::filters;
use corroded::utils::image_io;

fn main() {
    
    let image_path = "tests/tester.jpg";
    // this should be a match conditional with debug info
    let gray_image = image_io::load_image(image_path).expect("Failed to load image"); 
                                                                
    
    let width = gray_image.width();
    let height = gray_image.height();

    let edge_result = filters::apply_predefined_kernel(&gray_image, width.try_into().unwrap(), height.try_into().unwrap(), "edge_detection");
    let blur_result = filters::apply_predefined_kernel(&gray_image, width.try_into().unwrap(), height.try_into().unwrap(), "gaussian_blur");
    let sharpen_result = filters::apply_predefined_kernel(&gray_image, width.try_into().unwrap(), height.try_into().unwrap(), "sharpen");
    let emboss_result = filters::apply_predefined_kernel(&gray_image, width.try_into().unwrap(), height.try_into().unwrap(), "emboss");


    // Print the resulting image
  /*
    println!("Edge detection result: {:?}", edge_result);
    println!("Gaussian blur result: {:?}", blur_result);
    println!("Sharpen result: {:?}", sharpen_result);
    println!("Emboss result: {:?}", emboss_result);
  */
    image_io::create_image_from_vector(edge_result, width, height, "edgeout.png");
}
