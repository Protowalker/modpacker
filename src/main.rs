mod data;
mod mc_data;
mod types;
mod downloader;

extern crate serde;
extern crate reqwest;
extern crate tokio;
extern crate tempfile;
extern crate sha1;
extern crate cpuprofiler;

const VER_MANIFEST: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let result = reqwest::get(VER_MANIFEST)
        .await?
        .json::<mc_data::MojangVersionManifest>()
        .await?;
    
    let version = result.look_up_version(String::from("1.16.1")).unwrap();
    
    println!("downloading from {}", &version.url);

    let result = reqwest::get(&version.url[..])
        .await?
        .json::<mc_data::mojang_version_data::MojangVersionData>()
        .await?;
    
    match downloader::install_to_directory(&result, &std::path::Path::new("./installations/")).await {
        Err(e) => println!("{:?}", e),
        _ => ()
    }

    Ok(())
}
