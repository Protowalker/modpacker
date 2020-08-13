mod data;
mod launcher;

extern crate serde;
extern crate reqwest;
extern crate tempfile;
extern crate sha1;
extern crate cpuprofiler;
extern crate downloader;

use downloader::{download, mc_data};

const VER_MANIFEST: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
       println!("syntax: modpacker <version>");
       return Ok(());
    }

    let result = Box::new(reqwest::blocking::get(VER_MANIFEST)?
        .json::<mc_data::MojangVersionManifest>()?);
    
    let version = result.look_up_version(String::from(&*args[1])).unwrap();
    
    println!("downloading from {}", &version.url);

    let result = Box::new(reqwest::blocking::get(&version.url[..])?
        .json::<mc_data::mojang_version_data::MojangVersionData>()?);
    
    let instance_path = std::path::Path::new("./installations").join(&*args[1]);

    let download_successful = match download::install_to_directory(&result, &instance_path) {
        Err(e) => {println!("{:?}", e); false},
        _ => true
    };
    
    if download_successful {
        match launcher::launch_instance(&instance_path) {
            Ok(_) => println!("Launch Successful!"),
            Err(e) => println!("Error! {:#?}", e)
        }
    }
    println!("Exiting..");
    Ok(())
}
