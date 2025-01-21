mod yolo;

use std::env;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 从命令行获取模型路径
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("用法: {} <模型路径> <图片路径> <类型描述路径>", args[0]);
        return Ok(());
    }
    let model_path = &args[1];
    let image_path = &args[2];
    let type_path = &args[3];
    println!("模型路径: {}  图片路径: {} ", model_path, image_path);
    let image_tensor = yolo::preprocess_image(image_path)?;

    let output = yolo::predict(model_path, image_tensor)?;
    println!("output: {:?}", output);
    yolo::post_process(output,0.6,type_path)?;
    Ok(())
}

