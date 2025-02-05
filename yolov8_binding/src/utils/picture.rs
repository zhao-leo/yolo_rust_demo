// #![allow(unused)]
// use image::{self, DynamicImage, GenericImageView, ImageBuffer, Rgba};
// use std::error::Error;
// use std::path::Path;
// use tch::{vision, Tensor};
// use ab_glyph;
// use image::{self, DynamicImage, GenericImageView, Rgba};
// use imageproc::drawing::{draw_line_segment_mut, draw_text_mut};
// use serde_json::Value;
// use std::{
//     env,
//     error::Error,
//     ffi::{c_char, CString},
//     fs,
//     path::Path,
//     str::FromStr,
//     time::Instant,
// };
// use tch::{CModule, Device, Tensor};
// use winapi::um::libloaderapi::LoadLibraryA;

// fn image_process(
//     picture: &mut DynamicImage,
//     xywh: Tensor,
//     type_name: &str,
// ) -> Result<(), Box<dyn Error>> {
//     //x,y,w,h is based on 640*640 image size
//     let (mut x, mut y, mut w, mut h) = (
//         xywh.double_value(&[0, 0]),
//         xywh.double_value(&[0, 1]),
//         xywh.double_value(&[0, 2]),
//         xywh.double_value(&[0, 3]),
//     );
//     println!("x: {:.2}, y: {:.2}, w: {:.2}, h: {:.2}", x, y, w, h);
//     (x, y, w, h) = (x / 640., y / 640., w / 640., h / 640.); //Normalize to 1
//     let (width, height) = picture.dimensions();
//     (x, y, w, h) = (
//         x * width as f64,
//         y * height as f64,
//         w * width as f64,
//         h * height as f64,
//     );
//     let (x1, y1, x2, y2) = (x - w / 2., y - h / 2., x + w / 2., y + h / 2.);
//     draw_line_segment_mut(
//         picture,
//         (x1 as f32, y1 as f32),
//         (x2 as f32, y1 as f32),
//         Rgba([255, 0, 0, 255]),
//     );
//     draw_line_segment_mut(
//         picture,
//         (x2 as f32, y1 as f32),
//         (x2 as f32, y2 as f32),
//         Rgba([255, 0, 0, 255]),
//     );
//     draw_line_segment_mut(
//         picture,
//         (x2 as f32, y2 as f32),
//         (x1 as f32, y2 as f32),
//         Rgba([255, 0, 0, 255]),
//     );
//     draw_line_segment_mut(
//         picture,
//         (x1 as f32, y2 as f32),
//         (x1 as f32, y1 as f32),
//         Rgba([255, 0, 0, 255]),
//     );
//     let font = ab_glyph::FontRef::try_from_slice(include_bytes!("./arial.ttf"))?;
//     let scale = ab_glyph::PxScale::from(20.0);
//     draw_text_mut(
//         picture,
//         Rgba([255, 0, 0, 255]),
//         x1 as i32,
//         y1 as i32, // 调整文本位置
//         scale,
//         &font,
//         type_name,
//     );
//     Ok(())
// }
use image::{DynamicImage, GenericImageView, ImageReader, Rgba};
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut};
use std::{collections::HashMap, error::Error, fs, ops::Index, path::Path};

fn export(
    tags: &HashMap<i64, String>,
    image: Vec<(i64, i64, i64, i64, i64, f64)>,
    mut picture: DynamicImage,
) -> Result<DynamicImage, Box<dyn Error>> {
    for (x, y, w, h, type_id, _) in image {
        let type_name = tags.get(&type_id).unwrap();
        let (x, y, w, h) = (x as f64, y as f64, w as f64, h as f64);
        let (width, height) = picture.dimensions();
        let (x1, y1, x2, y2) = (
            (x - w / 2.) / 640. * width as f64,
            (y - h / 2.) / 640. * height as f64,
            (x + w / 2.) / 640. * width as f64,
            (y + h / 2.) / 640. * height as f64,
        );
        let (x1, y1, x2, y2) = (x1 as f32, y1 as f32, x2 as f32, y2 as f32);
        draw_line_segment_mut(&mut picture, (x1, y1), (x2, y1), Rgba([255, 0, 0, 255]));
        draw_line_segment_mut(&mut picture, (x2, y1), (x2, y2), Rgba([255, 0, 0, 255]));
        draw_line_segment_mut(&mut picture, (x2, y2), (x1, y2), Rgba([255, 0, 0, 255]));
        draw_line_segment_mut(&mut picture, (x1, y2), (x1, y1), Rgba([255, 0, 0, 255]));
        let font = ab_glyph::FontRef::try_from_slice(include_bytes!("arial.ttf")).unwrap();
        let scale = ab_glyph::PxScale::from(20.0);
        draw_text_mut(
            &mut picture,
            Rgba([255, 0, 0, 255]),
            x1 as i32,
            y1 as i32,
            scale,
            &font,
            type_name,
        );
    }
    Ok(picture)
}
pub fn export_images(
    tags: &HashMap<i64, String>,
    image: Vec<Vec<(i64, i64, i64, i64, i64, f64)>>,
    image_dir: &str,
    output_dir: &str,
) -> Result<(), Box<dyn Error>> {
    let output_dir = Path::new(output_dir);
    if !output_dir.exists() {
        fs::create_dir(&Path::new(output_dir))?;
    }
    for (index, entry) in fs::read_dir(image_dir)?.enumerate() {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let image_reader = ImageReader::open(Path::new(&path))?
                .with_guessed_format()
                .unwrap();
            let img = image_reader.decode().unwrap();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let output_img = export(tags, image.index(index).to_vec(), img)?;
            let output_path = Path::new(output_dir).join(file_name);
            println!("{:?}", output_path);
            output_img.save(output_path)?;
        }
    }
    Ok(())
}

pub fn export_one_image(
    tags: &HashMap<i64, String>,
    image: Vec<(i64, i64, i64, i64, i64, f64)>,
    image_path: &str,
) -> Result<DynamicImage, Box<dyn Error>> {
    let image_reader = ImageReader::open(Path::new(image_path))?
        .with_guessed_format()
        .unwrap();
    let picture = image_reader.decode().unwrap();
    export(tags, image, picture)
}
