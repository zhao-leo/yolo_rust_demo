use image::{self, DynamicImage, GenericImageView, Rgba};
use imageproc::drawing::{draw_line_segment_mut,draw_text_mut};
use serde_json::Value;
use std::{error::Error, fs, path::Path, time::Instant};
use tch::{CModule, Device, Tensor};
use ab_glyph;
pub fn predict(model_path: &str, input: Tensor) -> Result<Tensor, Box<dyn Error>> {
    //Tensor should be on the same device as the model [1, 3, 640, 640]
    let device = Device::cuda_if_available();
    println!("Device: {:?}", device);
    let model = CModule::load_on_device(Path::new(model_path), device)?;
    // start inference
    let time_start = Instant::now();
    let output = model.forward_ts(&[input])?;
    let time_end = Instant::now();
    println!("Inference time: {:?}", time_end.duration_since(time_start));
    Ok(output)
}
pub fn preprocess_image(image_path: &str) -> Result<Tensor, Box<dyn Error>> {
    let origin_image = tch::vision::image::load(image_path).unwrap();
    println!("Origin Image size: {:?}", origin_image);
    let image = tch::vision::image::resize(&origin_image, 640, 640)
        .unwrap()
        .unsqueeze(0)
        .to_kind(tch::Kind::Float)
        / 255.;
    println!("Resized Image size: {:?}", image);
    Ok(image)
}
pub fn post_process(
    output: Tensor,
    conf: f64,
    type_path: &str,
    image_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    //Tensor [1, 84, 8400]
    let pred = output.transpose(2, 1).squeeze();
    let (npreds, pred_size) = pred.size2().unwrap();
    println!("npreds: {:?}, pred_size: {:?}", npreds, pred_size);

    let json_buffer = fs::read_to_string(Path::new(type_path))?;

    let types: Value = serde_json::from_str(&json_buffer)?;
    println!("All types loaded");
    let full_xywh = pred.slice(1, 0, 4, 1);
    let mut picture = image::open(Path::new(image_path))?;
    println!("Input picture: {}", image_path);
    for index in 0..npreds {
        // 遍历所有预测框
        let pred = pred.get(index);
        let mut max_conf = 0.0;
        let mut max_conf_index = 0;
        for i in 4..pred_size {
            // 遍历所有类别
            if pred.double_value(&[i]) > max_conf {
                max_conf = pred.double_value(&[i]);
                max_conf_index = i;
            }
        }
        if max_conf > conf {
            let class_index = max_conf_index - 4;
            let class_name = &types[class_index.to_string()];
            print!("max_conf: {:.4}, class_name: {} ", max_conf, class_name);
            image_process(&mut picture, full_xywh.slice(0, index, index + 1, 1),class_name.as_str().unwrap())?;
        }
    }
    if let Ok(_) = picture.save(Path::new(output_path)){
        println!("Output picture: {}", output_path);
    }
    Ok(())
}

fn image_process(picture: &mut DynamicImage, xywh: Tensor,type_name: &str) -> Result<(), Box<dyn Error>> {
    //x,y,w,h is based on 640*640 image size
    let (mut x, mut y, mut w, mut h) = (
        xywh.double_value(&[0, 0]),
        xywh.double_value(&[0, 1]),
        xywh.double_value(&[0, 2]),
        xywh.double_value(&[0, 3]),
    );
    println!("x: {:.2}, y: {:.2}, w: {:.2}, h: {:.2}", x, y, w, h);
    (x, y, w, h) = (x / 640., y / 640., w / 640., h / 640.);  //Normalize to 1
    let (width, height) = picture.dimensions();
    (x, y, w, h) = (
        x * width as f64,
        y * height as f64,
        w * width as f64,
        h * height as f64,
    );
    let (x1, y1, x2, y2) = (x - w / 2., y - h / 2., x + w / 2., y + h / 2.);
    draw_line_segment_mut(
        picture,
        (x1 as f32, y1 as f32),
        (x2 as f32, y1 as f32),
        Rgba([255, 0, 0, 255]),
    );
    draw_line_segment_mut(
        picture,
        (x2 as f32, y1 as f32),
        (x2 as f32, y2 as f32),
        Rgba([255, 0, 0, 255]),
    );
    draw_line_segment_mut(
        picture,
        (x2 as f32, y2 as f32),
        (x1 as f32, y2 as f32),
        Rgba([255, 0, 0, 255]),
    );
    draw_line_segment_mut(
        picture,
        (x1 as f32, y2 as f32),
        (x1 as f32, y1 as f32),
        Rgba([255, 0, 0, 255]),
    );
    let font = ab_glyph::FontRef::try_from_slice(include_bytes!("./arial.ttf"))?;
    let scale = ab_glyph::PxScale::from(20.0);
    draw_text_mut(
        picture,
        Rgba([255, 0, 0, 255]),
        x1 as i32,
        y1 as i32, // 调整文本位置
        scale,
        &font,
        type_name,
    );
    Ok(())
}
