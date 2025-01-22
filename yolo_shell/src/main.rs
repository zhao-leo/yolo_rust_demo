use reqwest::blocking::Client;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::{exit,Stdio,Command};
use std::{env, fs};
use zip::ZipArchive;
use indicatif::{ProgressBar, ProgressStyle};


fn main() {
    let args: Vec<String> = env::args().collect();
    dll_inject();    
    // println!("Running command: \n{:?}", args);
    let mut child = Command::new(Path::new(&args[1]).as_os_str())
        .args(&args[2..])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute command");
    let status = child.wait().expect("failed to wait on child");
    println!("Process exited with: {}", status);
}

fn download_and_extract(url: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut response = client.get(url).send()?;

    let total_size = response
        .content_length()
        .ok_or("Failed to get content length")?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-")
    );
    let mut file = File::create("libtorch.zip")?;
    let mut downloaded: u64 = 0;
    let mut buffer = [0; 1024];
    while let Ok(n) = response.read(&mut buffer) {
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
        downloaded += n as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");

    let file = File::open("libtorch.zip")?;
    let mut archive = ZipArchive::new(file)?;
    archive.extract(output_dir)?;
    fs::remove_file("libtorch.zip")?;
    Ok(())
}
fn dll_inject() {
    let libtorch_path = Path::new("libtorch");
    match env::var("LIBTORCH") {
        Ok(path) => {
            println!("LIBTORCH environment variable found in {}", path);
        }
        Err(_) => {
            println!("No LIBTORCH environment variable found");
            if libtorch_path.exists() {
                let libtorch_path = Path::new("./libtorch").canonicalize().unwrap();
                let path_env = libtorch_path.join("lib");
                println!("libtorch directory found in current directory");
                env::set_var("LIBTORCH", libtorch_path.as_os_str());
                env::set_var("PATH", path_env.as_os_str());
                println!(
                    "LIBTORCH environment variable set to {:?}",
                    libtorch_path.as_os_str()
                );
                println!(
                    "PATH environment variable set to {:?}",
                    path_env.as_os_str()
                );
            } else {
                println!("libtorch directory not found in current directory");
                println!("Downloading libtorch-2.5.1 cpu version...");
                let download_url = "https://download.pytorch.org/libtorch/cpu/libtorch-win-shared-with-deps-2.5.1%2Bcpu.zip";
                match download_and_extract(download_url, "./") {
                    Ok(_) => {
                        println!("Downloaded and extracted libtorch successfully");
                        let libtorch_path = Path::new("./libtorch").canonicalize().unwrap();
                        let path_env = libtorch_path.join("lib");
                        env::set_var("LIBTORCH", libtorch_path.as_os_str());
                        env::set_var("PATH", path_env.as_os_str());
                        println!(
                            "LIBTORCH environment variable set to {:?}",
                            libtorch_path.as_os_str()
                        );
                        println!(
                            "PATH environment variable set to {:?}",
                            path_env.as_os_str()
                        );
                    }
                    Err(e) => {
                        println!("Failed to download and extract libtorch: {:?}", e);
                        exit(1);
                    }
                }
            }
        }
    }
}
