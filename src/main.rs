mod yolo;

use std::{env, io::Write, process::exit};
use reqwest::blocking::get;
use std::fs::File;
use zip::ZipArchive;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("初始化...");
    dll_inject();
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
fn download_and_extract(url: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?;
    let bytes = response.bytes()?;
    let mut file = File::create("libtorch.zip")?;
    file.write_all(&bytes)?;
    let file = File::open("libtorch.zip")?;
    let mut archive = ZipArchive::new(file)?;
    archive.extract(output_dir)?;
    Ok(())
}
fn dll_inject() {
    match env::var("LIBTORCH") {
        Ok(path) => {
            println!("LIBTORCH environment variable found in {}", path);
        }
        Err(_) => {
            println!("No LIBTORCH environment variable found");
            println!("Downloading libtorch cpu version...");
            let download_url = "https://download.pytorch.org/libtorch/cpu/libtorch-win-shared-with-deps-2.5.1%2Bcpu.zip";
            match download_and_extract(download_url, "./") {
                Ok(_) => {
                    println!("Downloaded and extracted libtorch successfully");
                    env::set_var("LIBTORCH", Path::new("./libtorch").as_os_str());
                    env::set_var("PATH", Path::new("./libtorch/lib").as_os_str());
                    println!("LIBTORCH environment variable set to {:?}", Path::new("./libtorch").as_os_str());
                    println!("PATH environment variable set to {:?}", Path::new("./libtorch/lib").as_os_str());
                }
                Err(e) => {
                    println!("Failed to download and extract libtorch: {:?}", e);
                    exit(1);
                }
            }
        }
    }
}
