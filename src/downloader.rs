use super::mc_data::{mojang_version_data::MojangVersionData};
use std::path::Path;
use std::fs::File;
use serde::{Serialize, Deserialize};
use super::types::{Or};

const RESOURCE_URL: &str = "https://resources.download.minecraft.net";


#[derive(Debug)]
pub enum InstallError {
    WebError(reqwest::Error),
    URLError(url::ParseError),
    IOError(std::io::Error),
    HashError(String),
    JSONError(serde_json::error::Error)
}


pub async fn install_to_directory(version: &MojangVersionData, directory: &Path) -> Result<(), InstallError> {
    let resource_url = url::Url::parse(RESOURCE_URL)?;
    //Need a builder for temporary files so that we don't leave half an installation
    let tmp_dir = tempfile::Builder::new().prefix("modpacker").tempdir()?;
    ///////////////First step: make sure assets folders exist
    let assets_path = Path::new("./assets");
    std::fs::create_dir_all(&assets_path.join("indexes"))?;
    std::fs::create_dir_all(&assets_path.join("objects"))?;
    ///////////////

    let assets_filename = String::from(format!("{}{}", &version.assets, ".json"));

    //if asset index doesn't exist, create and populate it. Otherwise, carry on
    if let Err(_) = std::fs::read(Path::new("./assets/indexes/").join(&version.assets).with_extension("json")) {
        try_download_and_write(&version.asset_index.url, &version.asset_index.sha1, &*assets_path.join("indexes"), &assets_filename, None).await?;
    }
    
    //if the objects folder isn't properly populated, create it
    {
        let client = reqwest::Client::new();
        
        let assets = std::fs::read_to_string(format!("./assets/indexes/{}", assets_filename))?;
        let objects: ResourceObjectData = serde_json::from_str::<ResourceData>(&assets[..])?.objects;
        for (name, hash_data) in objects.extra.iter() {
            //Files are stored in folders named with the first two characters in a hash.
            let dir = &assets_path.join("objects").join(&hash_data.hash[..2]);
            std::fs::create_dir_all(dir)?;
            
            //Does the file exist? If so, is the hash correct?
            let download_necessary = match std::fs::read(&dir.join(&hash_data.hash[..])) {
                Err(_) => true,
                Ok(f) => sha1::Sha1::from(f).digest().to_string() != &hash_data.hash[..]
            };
            if download_necessary
            {
                try_download_and_write(&resource_url
                                       .join(&format!("{}/{}", &hash_data.hash[..2], &hash_data.hash[..])[..])?
                                       .to_string(),
                                       &hash_data.hash,
                                       &dir,
                                       &hash_data.hash,
                                       Some(&client)
                                       ).await?;
    
                println!("downloaded {}", name);
    
            }
        }
    }
    //////
    //Next phase: installing libraries
    let lib_path = Path::new("./libraries");
    std::fs::create_dir_all(&lib_path)?;
    
    for lib in &version.libraries {
        let download = lib.downloads.artifact.as_ref().unwrap();
        let mut path: Vec<&str>= download.path.as_ref().unwrap().split("/").collect();
        
        let file_name = path.pop().unwrap().to_string();
        let path: std::path::PathBuf = path.iter().scan(lib_path, |state, &x| {Some(state.join(x))}).collect();
        
        std::fs::create_dir_all(&path)?;

        try_download_and_write(&download.url, &download.sha1, &path, &file_name, None).await?;
        println!("downloaded {}", file_name);
    }

    Ok(())

}

#[derive(Serialize, Deserialize)]
struct ResourceData {
    objects: ResourceObjectData
}

#[derive(Serialize, Deserialize)]
struct ResourceObjectData {
    #[serde(flatten)]
    extra: std::collections::HashMap<String, HashData>
}

#[derive(Serialize, Deserialize)]
struct HashData {
    hash: String,
    size: u32
}


async fn try_download_and_write(url: &String, hash: &String, dir: &Path, name: &String, client: Option<&reqwest::Client>) -> Result<(), InstallError> {
    let result = download_and_check(url, hash, client).await?;
    std::fs::write(dir.join(name), result)?;
    Ok(())        
}

async fn download_and_check(url: &String, hash: &String, client: Option<&reqwest::Client>) -> Result<Vec<u8>, InstallError> {
    let result = match client {
        None => reqwest::get(&url[..])
                         .await?
                         .bytes()
                         .await?,
        Some(client) => client.get(url)
                        .send()
                        .await?
                        .bytes()
                        .await?
    };

    
    if sha1::Sha1::from(&result).digest().to_string() != *hash {
        return Err(InstallError::HashError(url.to_string()));
    }
    Ok(result.iter().map(|v|->u8 {*v}).collect())
}

impl From<std::io::Error> for InstallError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<reqwest::Error> for InstallError {
    fn from(error: reqwest::Error) -> Self {
        Self::WebError(error)
    }
}

impl From<serde_json::Error> for InstallError {
    fn from(error: serde_json::Error) -> Self {
        Self::JSONError(error)
    }
}
impl From<url::ParseError> for InstallError {
    fn from(error: url::ParseError) -> Self {
        Self::URLError(error)
    }
}
