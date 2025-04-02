# Image Splitting Library

A Rust library for splitting images into smaller sub-images. This library provides two main functions:
1. Splitting an image into 9 equal parts (3x3 grid)
2. Splitting an image into sub-images of any specified size

## Features

- Split images into 9 equal parts (3x3 grid)
- Split images into custom-sized sub-images
- Handle edge cases where image dimensions aren't perfectly divisible
- Support for various image formats (PNG, JPEG, etc.)
- Preserve image quality and format

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
image_splitting = "0.1.0"
```

## Usage

### Splitting into 9 equal parts

```rust
use image_splitting::split_image;

let sub_images = split_image("path/to/image.png")?;
assert_eq!(sub_images.len(), 9);
```

### Splitting into custom-sized parts

```rust
use image_splitting::split_image_with_size;

// Split into 100x100 pixel sub-images
let sub_images = split_image_with_size("path/to/image.png", 100, 100)?;
```

## Examples

### Basic 3x3 Split
```rust
use image_splitting::split_image;

fn main() -> Result<(), image::ImageError> {
    let sub_images = split_image("input.png")?;
    
    // Save each sub-image
    for (i, sub_image) in sub_images.iter().enumerate() {
        sub_image.save(format!("sub_image_{}.png", i))?;
    }
    
    Ok(())
}
```

### Custom Size Split
```rust
use image_splitting::split_image_with_size;

fn main() -> Result<(), image::ImageError> {
    let sub_images = split_image_with_size("input.png", 200, 150)?;
    
    // Save each sub-image
    for (i, sub_image) in sub_images.iter().enumerate() {
        sub_image.save(format!("sub_image_{}.png", i))?;
    }
    
    Ok(())
}
```

## Dependencies

- `image` crate (version 0.24) for image processing

## Author

Created by Raisfeld Ori on February 4, 2025

## License

This project is licensed under the MIT License - see the LICENSE file for details.
