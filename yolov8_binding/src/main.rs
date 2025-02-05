// use std::env;
// fn main() -> Result<(), Box<dyn std::error::Error>> {

//     // 从命令行获取模型路径
//     let args: Vec<String> = env::args().collect();
//     if args.len() < 4 {
//         println!("用法: {} <模型路径> <图片路径> <类型描述路径> optional[-o <图片输出路径> -c <置信度>]", args[0]);
//         println!("默认情况下：\n标记过的图片将会被输出到输入路径\n置信度为0.5");
//         return Ok(());
//     }
//     let model_path = &args[1];
//     let image_path = &args[2];
//     let type_path  = &args[3];
//     let mut output_path = &args[2];
//     let mut confidence = 0.5;
//     for i in 4..args.len() {
//         if args[i] == "-o" {
//             output_path = &args[i + 1];
//         }
//         if args[i] == "-c" {
//             confidence = args[i + 1].trim().parse::<f64>().unwrap();
//         }
//     }
//     println!("模型路径: {}  图片路径: {} 输出路径: {} 置信度: {}", model_path, image_path, output_path, confidence);
//     use yolo_binding::yolo;
//     let image_tensor = yolo::preprocess_image(image_path)?;  //预处理图片

//     let output = yolo::predict(model_path, image_tensor)?;  //模型预测
//     println!("output: {:?}", output);
//     yolo::post_process(output,confidence,type_path,image_path,output_path)?;
//     Ok(())
// }
#![allow(unused_imports)]
use image::open;
use std::io;
use yolo_binding::core::YOLO;
fn main() {
    // let mut buf = String::new();
    // io::stdin().read_line(&mut buf).unwrap();
    let model_path = "waste_detection.jit";
    let image_path = "1.png";
    let model = YOLO::new(model_path, true);
    let image = model.load_one_image(image_path).unwrap();
    // let images = model.load_images_from_dir("images").unwrap();
    // let bytes = std::fs::read(image_path).unwrap();
    // let image2 = model.load_one_image_from_memory(&bytes).unwrap();
    let res = model.predict(&image).unwrap();
    print!("{:?}\n", res);
    // let res = model.predict(&images).unwrap();
    // print!("{:?}\n", res);
    let res = res.squeeze();
    println!("{:#?}", res.size());
    println!("{:#?}", model.types);
    let res = model.get_result(&res, 0.5,0.8).unwrap();
    println!("{:#?}", res);
}
