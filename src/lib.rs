use std::{
    error::Error,
    fmt,
    ops::BitXor,
    path::Path,
};

use imageproc::image::{
    imageops::{resize, FilterType},
    open, ImageBuffer, Luma, ImageError,
};
use rustdct::DctPlanner;

#[derive(Debug)]
pub enum ImgCmpError {
    ImageError(ImageError),
    UnsupportedAlgorithm(String),
}

impl fmt::Display for ImgCmpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImgCmpError::ImageError(e) => write!(f, "Image error: {}", e),
            ImgCmpError::UnsupportedAlgorithm(algo) => write!(f, "Unsupported algorithm: {}", algo),
        }
    }
}

impl From<ImageError> for ImgCmpError {
    fn from(err: ImageError) -> ImgCmpError {
        ImgCmpError::ImageError(err)
    }
}

impl Error for ImgCmpError {}

pub fn load_resize_grayscale(image_path: &Path) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, ImgCmpError> {
    let grayscale = open(image_path)?.to_luma8();
    Ok(resize(&grayscale, 9, 8, FilterType::Nearest))
}

pub fn hash_diff(pixels: &[u8], size: u16) -> (u16, u16) {
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
        .unwrap_or((0, 0))
}

pub fn distance_str(d: u32) -> String {
    match d {
        d if d < 1 => "Likely a similar picture".into(),
        d if d < 11 => "Potentially a variation".into(),
        d if d > 10 => "Likely a different image".into(),
        _ => "No distance provided".into(),
    }
}

pub fn dhash(pixels: &[u8], size: u16) -> u128 {
    let (row_hash, col_hash) = hash_diff(pixels, size);
    ((row_hash as u128) << (size * size)) | (col_hash as u128)
}

pub fn hamming_distance(hash1: u128, hash2: u128) -> u32 {
    hash1.bitxor(hash2).count_ones()
}

pub fn ahash(image_path: &Path) -> Result<u64, ImgCmpError> {
    let image = open(image_path)?.to_luma8();
    let resized = resize(&image, 8, 8, FilterType::Triangle);
    let pixels = resized.as_raw();
    let avg = pixels.iter().map(|&p| p as u32).sum::<u32>() / (8 * 8);
    Ok(pixels.iter().fold(0, |acc, &p| (acc << 1) | (p > avg as u8) as u64))
}

fn transpose(pixels: &[f32], width: usize, height: usize) -> Vec<f32> {
    let mut transposed = vec![0.0; pixels.len()];
    for y in 0..height {
        for x in 0..width {
            transposed[x * height + y] = pixels[y * width + x];
        }
    }
    transposed
}

pub fn phash(image_path: &Path) -> Result<u64, ImgCmpError> {
    let image = open(image_path)?.to_luma8();
    let resized = resize(&image, 32, 32, FilterType::Triangle);
    let mut pixels: Vec<f32> = resized.as_raw().iter().map(|&p| p as f32).collect();

    let mut planner = DctPlanner::new();
    let dct = planner.plan_dct2(32);

    // Apply DCT to rows
    for row in pixels.chunks_mut(32) {
        dct.process_dct2(row);
    }

    // Transpose
    pixels = transpose(&pixels, 32, 32);

    // Apply DCT to columns (which are now rows)
    for row in pixels.chunks_mut(32) {
        dct.process_dct2(row);
    }

    let mut top_left = Vec::new();
    for y in 0..8 {
        for x in 0..8 {
            top_left.push(pixels[y * 32 + x]);
        }
    }

    let median = {
        let mut sorted = top_left.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted[sorted.len() / 2]
    };

    Ok(top_left.iter().fold(0, |acc, &p| (acc << 1) | (p > median) as u64))
}
