use std::error::Error;
use std::ops::Index;
use yolo_binding::core::*;
use yolo_binding::utils;
fn main() -> Result<(), Box<dyn Error>> {
    let model_path = "waste_detection.jit";
    let model = YOLO::new(model_path, true);
    println!("{:?}", model.types);
    let images = load_images_from_dir("images").unwrap();
    let res = model.predict(&images).unwrap();
    print!("{:?}\n", res);
    let res = get_results(&res, 0.5, 0.8).unwrap();
    println!("{:?}", res);
    utils::picture::export_images(&model.types, res, "images", "output")?;
    let image = load_one_image("1.png").unwrap();
    let res = model.predict(&image).unwrap();
    print!("{:?}\n", res);
    let res = get_results(&res, 0.5, 0.8).unwrap();
    println!("{:?}", res);

    let pic =
        utils::picture::export_one_image(&model.types, res.index(0).to_vec(), "1.png").unwrap();
    pic.save("123.png").unwrap();
    Ok(())
}
