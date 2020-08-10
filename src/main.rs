mod data;
mod downloader;
mod mc_data;
mod types;

extern crate reqwest;
extern crate serde;
extern crate sha1;
extern crate tempfile;
extern crate tokio;

use data::ModData;

const VER_MANIFEST: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = reqwest::get(VER_MANIFEST)
        .await?
        .json::<mc_data::MojangVersionManifest>()
        .await?;

    let version = result.look_up_version(String::from("1.12.2")).unwrap();

    let result = reqwest::get(&version.url[..])
        .await?
        .json::<mc_data::mojang_version_data::MojangVersionData>()
        .await?;

    match downloader::install_to_directory(&result, &std::path::Path::new("./installations/")).await
    {
        Err(e) => println!("{:?}", e),
        _ => (),
    }

    Ok(())
}
