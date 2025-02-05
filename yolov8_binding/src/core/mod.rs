pub mod export;
pub mod load;
pub mod predict;
use std::{error::Error, collections::HashMap};
use tch::{CModule, Tensor};
pub enum YOLODevice {
    Cpu,
    Gpu,
}
pub struct YOLO {
    pub yolo_model: CModule,
    cuda: YOLODevice,
    pub types:HashMap<i64,String>,
}

impl YOLO {
    pub fn new(model_path: &str, cuda_is_available: bool) -> Self {
        load::load_model_from_path(model_path, cuda_is_available).unwrap()
    }
    pub fn load_one_image(&self, image_path: &str) -> Result<Tensor, Box<dyn Error>> {
        let image = load::load_one_image(image_path)?;
        Ok(image)
    }
    pub fn load_images_from_dir(&self, image_dir: &str) -> Result<Tensor, Box<dyn Error>> {
        let images = load::load_images_from_dir(image_dir)?;
        Ok(images)
    }
    pub fn load_one_image_from_memory(&self, image_bytes: &[u8]) -> Result<Tensor, Box<dyn Error>> {
        let image = load::load_one_image_from_memory(image_bytes)?;
        Ok(image)
    }
    pub fn predict(&self, input: &Tensor) -> Result<Tensor, Box<dyn Error>> {
        predict::pred(self, input)
    }
    pub fn get_result(&self, input: &Tensor, confidence: f64,threshold:f64) -> Result<Vec<(i64, i64, i64, i64, i64, f64)>, Box<dyn Error>> {
        export::post_process(input, confidence,threshold)
    }
}
