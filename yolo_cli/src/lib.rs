use std::{error::Error, ops::Index};
use yolo_binding::{core::*, utils::*};

enum ArgState {
    FileInput,
    DirInput,
}

fn arg_prase() -> Result<(ArgState, String, String, String, f64, f64), String> {
    //! Usage: yolo_binding [model_path] [-p input_image_path output_image_path | -d input_dir_path output_dir_path] < [-C confidence] [-T threshold] >
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 5 || args.len() > 9 {
        return Err(format!(
            "Usage: {} [model_path] [-p input_image_path output_image_path | -d input_dir_path output_dir_path] < [-C confidence] [-T threshold] >",
            args[0]
        ));
    }

    let mut confidence: f64 = 0.5;
    let mut threshold: f64 = 0.8;

    let mut i = 5;
    while i < args.len() {
        match args[i].as_str() {
            "-C" => {
                if i + 1 < args.len() {
                    confidence = args[i + 1].parse::<f64>().unwrap();
                    i += 2;
                } else {
                    return Err("Missing value for -C".to_string());
                }
            }
            "-T" => {
                if i + 1 < args.len() {
                    threshold = args[i + 1].parse::<f64>().unwrap();
                    i += 2;
                } else {
                    return Err("Missing value for -T".to_string());
                }
            }
            _ => return Err(format!("Unknown argument: {}", args[i])),
        }
    }

    match args[2].as_str() {
        "-p" => Ok((
            ArgState::FileInput,
            args[1].clone(),
            args[3].clone(),
            args[4].clone(),
            confidence,
            threshold,
        )),
        "-d" => Ok((
            ArgState::DirInput,
            args[1].clone(),
            args[3].clone(),
            args[4].clone(),
            confidence,
            threshold,
        )),
        _ => Err("Invalid option. Use -p for image paths or -d for directory paths.".to_string()),
    }
}

fn predict_image(args: (ArgState, String, String, String, f64, f64)) -> Result<(), Box<dyn Error>> {
    let (args, model_path, input, output, confidence, threshold) = args;
    let model = YOLO::new(&model_path, true);
    match args {
        ArgState::FileInput => {
            let image = load_one_image(&input).unwrap();
            let res = model.predict(&image).unwrap();
            let res = get_results(&res, confidence as f64, threshold as f64).unwrap();
            let picture =
                picture::export_one_image(&model.types, res.index(0).to_vec(), &input).unwrap();
            picture.save(output)?;
            Ok(())
        }
        ArgState::DirInput => {
            let images = load_images_from_dir(&input).unwrap();
            let res = model.predict(&images).unwrap();
            let res = get_results(&res, confidence as f64, threshold as f64).unwrap();
            picture::export_images(&model.types, res, &input, &output).unwrap();
            Ok(())
        }
    }
}

pub fn run() -> () {
    let args = arg_prase().unwrap();
    predict_image(args).unwrap()
}