use super::{YOLODevice, YOLO};
use std::{error::Error, time::Instant};
use tch::{Device, Tensor};

pub fn pred(model: &YOLO, input: &Tensor) -> Result<Tensor, Box<dyn Error>> {
    let device = match model.cuda {
        YOLODevice::Cpu => Device::Cpu,
        YOLODevice::Gpu => Device::cuda_if_available(),
    };
    let input = input.to_device(device);
    let time_start = Instant::now();
    let output = model
        .yolo_model
        .forward_ts(&[input])
        .expect("forward failed");
    let time_end = Instant::now();
    println!("Inference time: {:?}", time_end.duration_since(time_start));
    Ok(output)
}
