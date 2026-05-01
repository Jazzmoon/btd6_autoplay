use bitflags::bitflags;
use image::{imageops, DynamicImage, GenericImage, Rgba, RgbaImage};
use rusty_tesseract::Image;

use crate::models::coords::CoordsArea;

use super::global::CURRENT_WINDOW;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ImageProcessingType: u32 {
        const None          = 0b0;
        const Resize        = 0b1;
        const Grayscale     = 0b10;
        const Contrast      = 0b100;
        const SkewVertical  = 0b1000;
        const Invert        = 0b100000;
    }
}

pub fn capture_screenshot(debug: bool) -> DynamicImage {
    let current_window_read_lock = CURRENT_WINDOW.read().unwrap();
    let current_window = current_window_read_lock.as_ref().unwrap().get();
    match current_window.capture_image() {
        Ok(window_img) => {
            let dyn_image = DynamicImage::ImageRgba8(window_img);
            if debug {
                println!("Window position: ({}, {})", current_window.x(), current_window.y());
                println!("Window size: {}x{}", current_window.width(), current_window.height());
                let _ = dyn_image.save("debug/bloons_window_capture.png");
            }
            dyn_image
        }
        Err(err) => {
            panic!("Screenshot failed to capture properly: {:?}", err);
        }
    }
}

pub fn capture_area(
    screenshot: DynamicImage,
    ca: CoordsArea,
    image_processing: ImageProcessingType,
    grayscale_threshold: Option<i32>,
    contrast_value: Option<f32>,
    debug_image_name: Option<String>,
) -> DynamicImage {
    let cropped_img = imageops::crop_imm(
        &screenshot,
        ca.x.try_into().unwrap(),
        ca.y.try_into().unwrap(),
        ca.w.try_into().unwrap(),
        ca.h.try_into().unwrap(),
    );

    let mut image_buffer = cropped_img.to_image();

    if image_processing.contains(ImageProcessingType::Resize) {
        // Resize the image
        image_buffer = imageops::resize(
            &image_buffer,
            image_buffer.width() * 4,
            image_buffer.height() * 4,
            imageops::FilterType::Nearest,
        );
    }

    if image_processing.contains(ImageProcessingType::Contrast) {
        // Contrast the image
        if let Some(contrast_value) = contrast_value {
            image_buffer = imageops::contrast(&image_buffer, contrast_value);
        }
    }

    if image_processing.contains(ImageProcessingType::SkewVertical) {
        image_buffer = skew_vertical(&image_buffer, -5.0);
    }

    if image_processing.contains(ImageProcessingType::Invert) {
        // Invert the image
        let mut inverted = image_buffer.clone();
        imageops::invert(&mut inverted);
        image_buffer = inverted;
    }

    let mut dyn_image: DynamicImage;
    if image_processing.contains(ImageProcessingType::Grayscale) {
        match grayscale_threshold {
            Some(threshold) => {
                // Using the custom threshold, black and white the image pixel by pixel
                let (width, height) = image_buffer.dimensions();
                dyn_image = DynamicImage::new_luma8(width, height);
                for x in 0..width {
                    for y in 0..height {
                        let pixel = image_buffer.get_pixel(x, y);
                        let intensity = (0.299 * pixel[0] as f32
                            + 0.587 * pixel[1] as f32
                            + 0.114 * pixel[2] as f32)
                            as u8;
                        let new_pixel = if intensity > threshold as u8 {
                            Rgba([255, 255, 255, pixel[3]])
                        } else {
                            Rgba([0, 0, 0, pixel[3]])
                        };
                        dyn_image.put_pixel(x, y, new_pixel);
                    }
                }
            }
            None => {
                dyn_image = DynamicImage::ImageLuma8(imageops::grayscale(&image_buffer));
            }
        }
    } else {
        dyn_image = DynamicImage::ImageRgba8(image_buffer);
    };

    if let Some(name) = debug_image_name {
        let _ = dyn_image.save(format!("debug/post_processing_{name}.png"));
    }

    return dyn_image;
}

pub fn convert_to_rusty_image(img: DynamicImage) -> Image {
    match Image::from_dynamic_image(&img) {
        Ok(image) => image,
        Err(err) => panic!("Failed to convert image: {}", err),
    }
}

pub fn skew_vertical(image: &RgbaImage, angle_degrees: f32) -> RgbaImage {
    let width = image.width();
    let height = image.height();

    // Convert angle to radians and calculate skew factor
    let angle_rad = angle_degrees.to_radians();
    let skew_factor = angle_rad.tan();

    // Calculate new dimensions
    let new_height = height;
    let new_width = width + (height as f32 * skew_factor.abs()).ceil() as u32;

    // Create new buffer
    let mut skewed = RgbaImage::new(new_width, new_height);

    // Apply vertical skew transformation
    for y in 0..new_height {
        for x in 0..new_width {
            // Calculate source coordinates using inverse skew
            let src_y = y;
            let src_x = (x as f32 - ((height - y) as f32 * skew_factor)) as i32;

            // Only copy pixels if source coordinates are within bounds
            if src_x >= 0 && src_x < width as i32 {
                let pixel = image.get_pixel(src_x as u32, src_y);
                skewed.put_pixel(x, y, *pixel);
            }
        }
    }

    skewed
}
