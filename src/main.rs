use std::{
    env,
    ops::BitXor,
    path::{Path, PathBuf},
};

use imageproc::image::{
    imageops::{resize, FilterType},
    open, ImageBuffer, Luma,
};

fn load_resize_grayscale(image_path: &Path) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    if !image_path.is_file() {
        panic!("Input file does not exits");
    }

    let grayscale = open(image_path)
        .unwrap_or_else(|_| panic!("Could not load image at {:?}", image_path))
        .to_luma8();

    resize(&grayscale, 9, 8, FilterType::Nearest)
}

// Calculate row and column difference hash for given image and return
// hashes combined as a single 2*size*size bit integer (row_hash in most
// significant bits, col_hash in least).
fn hash_diff(pixels: &[u8], size: u16) -> (u16, u16) {
    let width: usize = size as usize;

    (0..size)
        .flat_map(|y| (0..size).map(move |x| (x, y)))
        .reduce(|(acc_row_hash, acc_col_hash), (x, y)| {
            let offset = y as usize * width + x as usize;
            let row_bit = pixels[offset] < pixels[offset + 1];
            let col_bit = pixels[offset] < pixels[offset + width];

            (
                (acc_row_hash << 1) | u16::from(row_bit),
                (acc_col_hash << 1) | u16::from(col_bit),
            )
        })
        .unwrap_or((0, 0)) // In case the iterator is empty
}

fn distance_str(d: u32) -> String {
    match d {
        d if d < 1 => "Likely a similar picture".into(),
        d if d < 11 => "Potentially a variation".into(),
        d if d > 10 => "Likely a different image".into(),
        _ => panic!("No distance provided"),
    }
}

// Calculate difference hash (perceptual hash) for a given image, useful for detecting duplicates.
fn main() {
    if env::args().len() != 3 {
        panic!("Please enter an two input images")
    }

    let images: Vec<PathBuf> = env::args().skip(1).map(PathBuf::from).collect();

    let image_path_1 = Path::new(&images[0]);
    let image_path_2 = Path::new(&images[1]);

    let image_a = load_resize_grayscale(image_path_1);
    let image_b = load_resize_grayscale(image_path_2);

    let pixels_image_a = image_a.iter().as_slice();
    let pixels_image_b = image_b.iter().as_slice();

    let size = 8;

    // Distance Hash
    let (row_hash_image_a, col_hash_image_a) = hash_diff(pixels_image_a, size);
    let (row_hash_image_b, col_hash_image_b) = hash_diff(pixels_image_b, size);

    let dhash_int_a: u128 =
        ((row_hash_image_a as u128) << (size * size)) | (col_hash_image_a as u128);

    let dhash_int_b: u128 =
        ((row_hash_image_b as u128) << (size * size)) | (col_hash_image_b as u128);

    // Hamming distance
    let hamming_distance = dhash_int_a.bitxor(dhash_int_b).count_ones();

    println!(
        "{} bit differs out of {} ({:0.1}%)\n{}",
        hamming_distance,
        size * size * 2,
        100 * hamming_distance / (size * size * 2) as u32,
        distance_str(hamming_distance)
    );
}
