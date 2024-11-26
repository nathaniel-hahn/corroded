/*
 * Nathaniel Hahn
 * corroded
 *
 */
use crate::cpu::kernels;


pub fn apply_convolution(
    image: &[u8], // image array
    width: usize,
    height: usize,
    kernel: &[f32],
    kernel_size: usize,
) -> Vec<u8> {
    assert!(kernel_size % 2 == 1, "Kernel must be odd");

    let mut output = vec![0u8; width * height];

    let radius = kernel_size / 2;

    for y in 0..height{
        for x in 0..width{
            let mut sum = 0.0;


            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let px = x as isize + ky as isize - radius as isize;
                    let py = y as isize + ky as isize - radius as isize;

                    if px >= 0 && px < width as isize && py >= 0 && py < height as isize{
                        let pixel = image[py as usize * width +px as usize] as f32;
                        let weight = kernel[ky * kernel_size + kx];
                        sum += pixel * weight; 
                    }
                }
            }

            output[y* width + x] = sum.clamp(0.0, 255.0) as u8;
        }
    }

    output
}

pub fn apply_predefined_kernel(
    image: &[u8],
    width: usize,
    height: usize,
    kernel_name: &str,
) -> Vec<u8> {
    match kernel_name {
        "edge_detection" => apply_convolution(image, width, height, &kernels::EDGE_DETECTION, 3),
        "gaussian_blur" => apply_convolution(image, width, height, &kernels::GAUSSIAN_BLUR, 3),
        "sharpen" => apply_convolution(image, width, height, &kernels::SHARPEN, 3),
        "emboss" => apply_convolution(image, width, height, &kernels::EMBOSS, 3),
        _ => panic!("Kernel not found"),
    }
}

// Test Module
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::kernels;

    // Test for edge detection
    #[test]
    fn test_edge_detection() {
        let image: Vec<u8> = vec![
        10, 10, 10, 10, 10,
        10, 50, 50, 50, 10,
        10, 50, 100, 50, 10,
        10, 50, 50, 50, 10,
        10, 10, 10, 10, 10,
    ];
        let result = apply_predefined_kernel(&image, 5, 5, "edge_detection");
        let expected = vec![0, 0, 0, 30, 60, 0, 0, 120, 240, 30, 0, 120, 255, 120, 0, 30, 240, 120, 0, 0, 60, 30, 0, 0, 0];
        assert_eq!(result, expected);
    }

    // Test for gaussian blur
    #[test]
    fn test_gaussian_blur() {
        let image: Vec<u8> =  vec![
        10, 10, 10, 10, 10,
        10, 50, 50, 50, 10,
        10, 50, 100, 50, 10,
        10, 50, 50, 50, 10,
        10, 10, 10, 10, 10,
    ];  
        let result = apply_predefined_kernel(&image, 5, 5, "gaussian_blur");
        let expected = vec![17, 17, 17, 7, 5, 17, 52, 40, 30, 7, 17, 40, 75, 40, 17, 7, 30, 40, 52, 17, 5, 7, 17, 17, 17];
        assert_eq!(result, expected);
    }

    // Test for sharpening
    #[test]
    fn test_sharpen() {
        let image: Vec<u8> = vec![
        10, 10, 10, 10, 10,
        10, 50, 50, 50, 10,
        10, 50, 100, 50, 10,
        10, 50, 50, 50, 10,
        10, 10, 10, 10, 10,
    ];
        let result = apply_predefined_kernel(&image, 5, 5, "sharpen");
        let expected = vec![0, 0, 0, 20, 30, 0, 40, 90, 130, 20, 0, 90, 200, 90, 0, 20, 130, 90, 40, 0, 30, 20, 0, 0, 0];
        assert_eq!(result, expected);
    }

    // Test for embossing
    #[test]
    fn test_emboss() {
        let image: Vec<u8> = vec![
        10, 10, 10, 10, 10,
        10, 50, 50, 50, 10,
        10, 50, 100, 50, 10,
        10, 50, 50, 50, 10,
        10, 10, 10, 10, 10,
    ];
        let result = apply_predefined_kernel(&image, 5, 5, "emboss");
        let expected = vec![160, 160, 160, 40, 10, 160, 255, 170, 50, 0, 160, 170, 100, 0, 0, 40, 50, 0, 0, 0, 10, 0, 0, 0, 0];
        assert_eq!(result, expected);
    }
}
