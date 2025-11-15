use std::path::Path;
use imgcmp_rs::{ahash, dhash, phash, load_resize_grayscale};

#[test]
fn test_dhash() {
    let image = load_resize_grayscale(Path::new("tests/images/black.png")).unwrap();
    let pixels = image.iter().as_slice();
    let hash = dhash(pixels, 8);
    assert_eq!(hash, 0);

    let image = load_resize_grayscale(Path::new("tests/images/white.png")).unwrap();
    let pixels = image.iter().as_slice();
    let hash = dhash(pixels, 8);
    assert_eq!(hash, 0);

    let image = load_resize_grayscale(Path::new("tests/images/checkerboard.png")).unwrap();
    let pixels = image.iter().as_slice();
    let hash = dhash(pixels, 8);
    assert_ne!(hash, 0);
}

#[test]
fn test_ahash() {
    let hash = ahash(Path::new("tests/images/black.png")).unwrap();
    assert_eq!(hash, 0);

    let hash = ahash(Path::new("tests/images/white.png")).unwrap();
    assert_eq!(hash, 0);

    let hash = ahash(Path::new("tests/images/checkerboard.png")).unwrap();
    assert_ne!(hash, 0);
    assert_ne!(hash, u64::MAX);
}

#[test]
fn test_phash() {
    let hash = phash(Path::new("tests/images/black.png")).unwrap();
    assert_eq!(hash, 0);

    let hash = phash(Path::new("tests/images/white.png")).unwrap();
    assert_ne!(hash, 0);

    let hash = phash(Path::new("tests/images/checkerboard.png")).unwrap();
    assert_ne!(hash, 0);
}
