# drash - Image Hash Distance

*drash* is a Rust-based command-line tool for calculating perceptual hashes of images and comparing them to detect duplicates. This tool uses difference hashing (dHash) to generate a hash for each image, and then calculates the Hamming distance between these hashes to determine the similarity.

Usage
To use ImageHashComparator, you need to have Rust installed on your system. You can install Rust from here.

## Building the Project
Clone the repository and build the project using Cargo:
```sh
cargo build --release
```
## Running the Project
To run the project, provide two image file paths as arguments:
```sh
cargo run --release <path_to_image_1> <path_to_image_2>
```
For example:

```sh
cargo run --release images/image1.png images/image2.png
```
## Example Output
```sh
5 bit differs out of 128 (3.9%) in 12ms
```


## Acknowledgments

This code was inspired by Dr. Neal Krawetz [blogpost](https://www.hackerfactor.com/blog/index.php?/archives/529-Kind-of-Like-That.html)
