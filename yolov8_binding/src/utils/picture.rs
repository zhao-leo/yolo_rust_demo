// #![allow(unused)]
// use image::{self, DynamicImage, GenericImageView, ImageBuffer, Rgba};
// use std::error::Error;
// use std::path::Path;
// use tch::{vision, Tensor};

// pub fn load_one_image(image_path: &str) -> Result<Tensor, Box<dyn Error>> {
//     let image_tensor = vision::image::load(Path::new(&image_path))?;
//     let image = tch::vision::image::resize(&image_tensor, 640, 640)
//         .unwrap()
//         .unsqueeze(0)
//         .to_kind(tch::Kind::Float)
//         / 255.;
//     println!("Resized Image size: {:?}", image);
//     Ok(image)
// }
// pub fn load_images_from_dir(image_dir: &str) -> Result<Tensor, Box<dyn Error>> {
//     let image_tensor = vision::image::load_dir(Path::new(&image_dir), 640, 640)
//         .unwrap()
//         .to_kind(tch::Kind::Float)
//         / 255.;
//     println!("Resized Image size: {:?}", image_tensor);
//     Ok(image_tensor)
// }

// pub fn load_one_image_from_memory(image_bytes: &[u8]) -> Result<Tensor, Box<dyn Error>> {
//     let image_tensor = vision::image::load_from_memory(image_bytes)?;
//     let image = tch::vision::image::resize(&image_tensor, 640, 640)
//         .unwrap()
//         .unsqueeze(0)
//         .to_kind(tch::Kind::Float)
//         / 255.;
//     println!("Resized Image size: {:?}", image);
//     Ok(image)
// }

// // fn add_gray_background_to_square(image_path: &str) -> Result<DynamicImage, Box<dyn Error>> {
// //     let img = match image::ImageReader::open(Path::new(image_path))?.with_guessed_format() {
// //         Ok(reader) => reader.decode()?,
// //         Err(_) => return Err("Unknown image type".into()),
// //     };
// //     let (width, height) = img.dimensions();
// //     let side_length = width.max(height);
// //     // generate a square image with gray background
// //     let mut square_img =
// //         ImageBuffer::from_pixel(side_length, side_length, Rgba([128, 128, 128, 255]));
// //     // overlay the original image on the square image
// //     let x_offset = ((side_length - width) / 2) as i64;
// //     let y_offset = ((side_length - height) / 2) as i64;
// //     image::imageops::overlay(&mut square_img, &img, x_offset, y_offset);
// //     Ok(DynamicImage::ImageRgba8(square_img))
// // }

// // fn add_gray_background_to_square_from_memory(
// //     image_bytes: &[u8],
// // ) -> Result<DynamicImage, Box<dyn Error>> {
// //     let img = image::load_from_memory(image_bytes)?;
// //     let (width, height) = img.dimensions();
// //     let side_length = width.max(height);
// //     // generate a square image with gray background
// //     let mut square_img =
// //         ImageBuffer::from_pixel(side_length, side_length, Rgba([128, 128, 128, 255]));
// //     // overlay the original image on the square image
// //     let x_offset = ((side_length - width) / 2) as i64;
// //     let y_offset = ((side_length - height) / 2) as i64;
// //     image::imageops::overlay(&mut square_img, &img, x_offset, y_offset);
// //     Ok(DynamicImage::ImageRgba8(square_img))
// // }
