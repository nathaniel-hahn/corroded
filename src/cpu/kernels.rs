/*
 * Nathaniel Hahn
 * corroded
 * kernel defs
 */



/// Edge detection kernel (Sobel operator)
pub const EDGE_DETECTION: [f32; 9] = [
    -1.0, -1.0, -1.0,
    -1.0,  8.0, -1.0,
    -1.0, -1.0, -1.0,
];

/// Gaussian blur kernel (3x3)
pub const GAUSSIAN_BLUR: [f32; 9] = [
    1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0,
    2.0 / 16.0, 4.0 / 16.0, 2.0 / 16.0,
    1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0,
];

/// Sharpening kernel
pub const SHARPEN: [f32; 9] = [
    0.0, -1.0, 0.0,
    -1.0,  5.0, -1.0,
    0.0, -1.0, 0.0,
];

/// Emboss kernel
pub const EMBOSS: [f32; 9] = [
    -2.0, -1.0, 0.0,
    -1.0,  1.0, 1.0,
    0.0,  1.0, 2.0,
];
