use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Modloader {
    LiteLoader(String),
    Forge(String),
    Fabric(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModData {
    pub unique_name: String,
    pub display_name: String,
    pub mod_loader: Modloader,
    pub version: String,
    pub description: String,
    pub author: String,
    pub jar_links: Vec<ModJarData>,
    pub src_link: String,
    pub dependencies: Vec<DependencyData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DependencyData {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModJarData {
    pub link: String,
    pub minecraft_version: String,
}

impl ModData {
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModpackData {
    pub unique_name: String,
    pub display_name: String,
    pub minecraft_version: String,
    pub mod_loader: Modloader,
}
