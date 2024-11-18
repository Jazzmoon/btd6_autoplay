use bitflags::bitflags;
use image::{imageops, DynamicImage, GenericImage, Rgba};
use rusty_tesseract::Image;

use crate::lib::global::CURRENT_WINDOW;
use crate::models::coords::CoordsArea;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ImageProcessingType: u32 {
        const None          = 0b0;
        const Resize        = 0b1;
        const Grayscale     = 0b10;
        const Contrast      = 0b100;
        const FloodFill     = 0b1000;
        const SheerVertical = 0b10000;
        const Invert        = 0b100000;
    }
}

fn capture_screenshot() -> DynamicImage {
  let current_window_read_lock = CURRENT_WINDOW.read().unwrap();
  let current_window = current_window_read_lock.as_ref().unwrap().get();
  let monitor = current_window.current_monitor();
  match monitor.capture_image() {
    Ok(screenshot) => {
      let monitor_img = DynamicImage::ImageRgba8(screenshot);
      let window_img = imageops::crop_imm(
        &monitor_img,
        current_window.x().try_into().unwrap(),
        current_window.y().try_into().unwrap(),
        current_window.width(),
        current_window.height()
      );

      DynamicImage::ImageRgba8(window_img.to_image())
    },
    Err(err) => {
        panic!("Screenshot failed to capture properly: {:?}", err);
    }
  }
}

pub fn capture_area(ca: CoordsArea, image_processing: ImageProcessingType, grayscale_threshold: Option<i32>, contrast_value: Option<f32>) -> DynamicImage {
  let screenshot = capture_screenshot();
  let cropped_img = imageops::crop_imm(
    &screenshot,
    ca.x.try_into().unwrap(),
    ca.y.try_into().unwrap(),
    ca.w.try_into().unwrap(),
    ca.h.try_into().unwrap()
  );

  let mut image_buffer = cropped_img.to_image();

  if image_processing.contains(ImageProcessingType::Resize) {
    // Resize the image
    image_buffer = imageops::resize(
      &image_buffer,
      image_buffer.width() * 4,
      image_buffer.height() * 4,
      imageops::FilterType::Nearest
    );
  }

  if image_processing.contains(ImageProcessingType::Contrast) {
    // Contrast the image
    if let Some(contrast_value) = contrast_value {
      image_buffer = imageops::contrast(&image_buffer, contrast_value);
    }
  }

  if image_processing.contains(ImageProcessingType::FloodFill) {
    // Flood fill the image
  }

  if image_processing.contains(ImageProcessingType::SheerVertical) {
    // Sheer the image vertically
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
            let intensity = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;
            let new_pixel = if intensity > threshold as u8 {
              Rgba([255, 255, 255, pixel[3]])
            } else {
              Rgba([0, 0, 0, pixel[3]])
            };
            dyn_image.put_pixel(x, y, new_pixel);
          }
        }
      },
      None => {
        dyn_image = DynamicImage::ImageLuma8(imageops::grayscale(&image_buffer));
      }
    }
  } else {
    dyn_image = DynamicImage::ImageRgba8(image_buffer);
  };

  // Save the image for debugging purposes
  let _ = dyn_image.save("debug/screenshot.png");
  return dyn_image;
}

pub fn convert_to_rusty_image(img: DynamicImage) -> Image {
  Image::from_dynamic_image(&img).unwrap()
}