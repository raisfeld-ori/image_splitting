//! Image Splitting Library
//! 
//! This library provides functionality to split images into smaller sub-images.
//! It supports both equal division (3x3 grid) and custom-sized sub-images.

use image::{GenericImageView, ImageBuffer, Rgba};
use std::path::Path;

/// Splits an image into 9 equal parts (3x3 grid).
/// 
/// # Arguments
/// 
/// * `image_path` - Path to the input image file
/// 
/// # Returns
/// 
/// * `Result<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>, image::ImageError>` - A vector of 9 sub-images or an error if the operation fails
/// 
/// # Example
/// 
/// ```
/// use image_splitting::split_image;
/// 
/// let sub_images = split_image("path/to/image.png")?;
/// assert_eq!(sub_images.len(), 9);
/// ```
pub fn split_image<P: AsRef<Path>>(image_path: P) -> Result<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>, image::ImageError> {
    // Load the image
    let img = image::open(image_path)?;
    
    // Get image dimensions
    let (width, height) = img.dimensions();
    
    // Calculate dimensions for each sub-image
    let sub_width = width / 3;
    let sub_height = height / 3;
    
    let mut sub_images = Vec::new();
    
    // Split the image into 9 parts
    for y in 0..3 {
        for x in 0..3 {
            let sub_img = img.crop_imm(
                x * sub_width,
                y * sub_height,
                sub_width,
                sub_height
            );
            
            sub_images.push(sub_img.to_rgba8());
        }
    }
    
    Ok(sub_images)
}

/// Splits an image into sub-images of specified dimensions.
/// 
/// This function allows splitting an image into sub-images of any size. The last row and column
/// may be smaller if the image dimensions aren't perfectly divisible by the specified sub-image size.
/// 
/// # Arguments
/// 
/// * `image_path` - Path to the input image file
/// * `sub_width` - Desired width of each sub-image
/// * `sub_height` - Desired height of each sub-image
/// 
/// # Returns
/// 
/// * `Result<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>, image::ImageError>` - A vector of sub-images or an error if the operation fails
/// 
/// # Example
/// 
/// ```
/// use image_splitting::split_image_with_size;
/// 
/// let sub_images = split_image_with_size("path/to/image.png", 100, 100)?;
/// ```
pub fn split_image_with_size<P: AsRef<Path>>(
    image_path: P,
    sub_width: u32,
    sub_height: u32,
) -> Result<Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>, image::ImageError> {
    // Load the image
    let img = image::open(image_path)?;
    
    // Get image dimensions
    let (width, height) = img.dimensions();
    
    // Calculate number of sub-images in each dimension using ceiling division
    let num_cols = (width + sub_width - 1) / sub_width;
    let num_rows = (height + sub_height - 1) / sub_height;
    
    let mut sub_images = Vec::new();
    
    // Split the image into parts
    for y in 0..num_rows {
        for x in 0..num_cols {
            let x_pos = x * sub_width;
            let y_pos = y * sub_height;
            
            // Calculate actual dimensions for the last row/column
            let actual_width = (width - x_pos).min(sub_width);
            let actual_height = (height - y_pos).min(sub_height);
            
            let sub_img = img.crop_imm(
                x_pos,
                y_pos,
                actual_width,
                actual_height
            );
            
            sub_images.push(sub_img.to_rgba8());
        }
    }
    
    Ok(sub_images)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_image() {
        // This test assumes there's a test image in the tests directory
        let result = split_image("tests/test_image.png");
        assert!(result.is_ok());
        let sub_images = result.unwrap();
        assert_eq!(sub_images.len(), 9);
    }

    #[test]
    fn test_split_image_with_size() {
        let result = split_image_with_size("tests/test_image.png", 100, 100);
        assert!(result.is_ok());
        let sub_images = result.unwrap();
        assert!(sub_images.len() > 0);
    }
}
