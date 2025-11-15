use std::path::PathBuf;

use clap::Parser;
use imgcmp_rs::{ahash, dhash, hamming_distance, load_resize_grayscale, phash, distance_str, ImgCmpError};

#[derive(Parser)]
#[clap(version = "1.0", author = "Your Name")]
struct Opts {
    #[clap(short, long, default_value = "dhash")]
    algorithm: String,
    #[clap(required = true)]
    image1: PathBuf,
    #[clap(required = true)]
    image2: PathBuf,
}

fn main() -> Result<(), ImgCmpError> {
    let opts: Opts = Opts::parse();

    let hash1 = match opts.algorithm.as_str() {
        "dhash" => {
            let image = load_resize_grayscale(&opts.image1)?;
            let pixels = image.iter().as_slice();
            Ok(dhash(pixels, 8))
        }
        "ahash" => ahash(&opts.image1).map(|h| h as u128),
        "phash" => phash(&opts.image1).map(|h| h as u128),
        _ => Err(ImgCmpError::UnsupportedAlgorithm(opts.algorithm.clone())),
    }?;

    let hash2 = match opts.algorithm.as_str() {
        "dhash" => {
            let image = load_resize_grayscale(&opts.image2)?;
            let pixels = image.iter().as_slice();
            Ok(dhash(pixels, 8))
        }
        "ahash" => ahash(&opts.image2).map(|h| h as u128),
        "phash" => phash(&opts.image2).map(|h| h as u128),
        _ => Err(ImgCmpError::UnsupportedAlgorithm(opts.algorithm.clone())),
    }?;

    let distance = hamming_distance(hash1, hash2);
    let size = match opts.algorithm.as_str() {
        "dhash" => 128,
        "ahash" => 64,
        "phash" => 64,
        _ => return Err(ImgCmpError::UnsupportedAlgorithm(opts.algorithm.clone())),
    };

    println!(
        "{} bit differs out of {} ({:0.1}%)\n{}",
        distance,
        size,
        100 * distance / size as u32,
        distance_str(distance)
    );

    Ok(())
}
