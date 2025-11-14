# drash - Image Hash Distance

*drash* is a Rust-based command-line tool and library for calculating perceptual hashes of images and comparing them to detect duplicates. This tool supports three common perceptual hashing algorithms:

*   **dHash (Difference Hash):** This is the default algorithm. It's fast and effective, but less accurate than pHash.
*   **aHash (Average Hash):** This algorithm is even faster than dHash, but also less accurate.
*   **pHash (Perceptual Hash):** This is the most accurate of the three, but also the slowest. It's based on the Discrete Cosine Transform (DCT) and is more robust to image modifications.

## Usage

To use drash, you need to have Rust installed on your system. You can install Rust from [here](https://www.rust-lang.org/tools/install).

### Building the Project

Clone the repository and build the project using Cargo:

```sh
cargo build --release
```

### Running the Project

To run the project, provide two image file paths as arguments. You can also specify the hashing algorithm to use with the `--algorithm` flag.

```sh
cargo run --release -- --algorithm <algorithm> <path_to_image_1> <path_to_image_2>
```

For example:

```sh
cargo run --release -- --algorithm phash images/image1.png images/image2.png
```

### Example Output

```sh
5 bit differs out of 64 (7.8%)
Potentially a variation
```

### Library API

This crate can also be used as a library. To use it, add the following to your `Cargo.toml`:

```toml
[dependencies]
drash = "0.1.0"
```

You can then use the `dhash`, `ahash`, and `phash` functions in your own code:

```rust
use drash::{ahash, dhash, phash, load_resize_grayscale};
use std::path::Path;

fn main() {
    let image_path = Path::new("path/to/your/image.png");

    let image = load_resize_grayscale(image_path).unwrap();
    let pixels = image.iter().as_slice();
    let dhash_hash = dhash(pixels, 8);

    let ahash_hash = ahash(image_path).unwrap();
    let phash_hash = phash(image_path).unwrap();
}
```

## Acknowledgments

This code was inspired by Dr. Neal Krawetz [blogpost](https://www.hackerfactor.com/blog/index.php?/archives/529-Kind-of-Like-That.html)
