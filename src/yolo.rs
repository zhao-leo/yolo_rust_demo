use serde_json::Value;
use std::{error::Error, fs, path::Path, time::Instant};
use tch::{CModule, Device, Tensor};
pub fn predict(model_path: &str, input: Tensor) -> Result<Tensor, Box<dyn Error>> {
    //Tensor should be on the same device as the model [1, 3, 640, 640]
    let device = Device::cuda_if_available();
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
pub fn post_process(output: Tensor, conf: f64, type_path: &str) -> Result<(), Box<dyn Error>> {
    //Tensor [1, 84, 8400]
    let pred = output.transpose(2, 1).squeeze();
    let (npreds, pred_size) = pred.size2().unwrap();
    println!("npreds: {:?}, pred_size: {:?}", npreds, pred_size);

    let json_buffer = fs::read_to_string(Path::new(type_path))?;

    let types: Value = serde_json::from_str(&json_buffer)?;
    println!("All types loaded");

    for index in 0..npreds {
        // 遍历所有预测框
        let pred = pred.get(index);
        let mut max_conf = 0.0;
        let mut max_conf_index = 0;
        for i in 4..pred_size {
            if pred.double_value(&[i]) > max_conf {
                max_conf = pred.double_value(&[i]);
                max_conf_index = i;
            }
        }
        if max_conf > conf {
            let class_index = max_conf_index - 4;
            let class_name = &types[class_index.to_string()];
            println!("max_conf: {:?}, class_name: {}", max_conf, class_name);
        }
    }
    Ok(())
}
