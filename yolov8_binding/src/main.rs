use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 从命令行获取模型路径
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("用法: {} <模型路径> <图片路径> <类型描述路径> optional[-o <图片输出路径> -c <置信度>]", args[0]);
        return Ok(());
    }
    let model_path = &args[1];
    let image_path = &args[2];
    let type_path = &args[3];
    let mut output_path = &args[2];
    let mut confidence = 0.5;
    for i in 4..args.len() {
        if args[i] == "-o" {
            output_path = &args[i + 1];
        }
        if args[i] == "-c" {
            confidence = args[i + 1].trim().parse::<f64>().unwrap();
        }
    }
    println!("模型路径: {}  图片路径: {} 输出路径: {} 置信度: {}", model_path, image_path, output_path, confidence);
    use yolo_binding::yolo;
    let image_tensor = yolo::preprocess_image(image_path)?;  //预处理图片

    let output = yolo::predict(model_path, image_tensor)?;  //模型预测
    println!("output: {:?}", output);
    yolo::post_process(output,confidence,type_path,image_path,output_path)?;
    Ok(())
}

