use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MojangVersionManifest {
    pub latest: MojangVersionManifestLatest,
    pub versions: Vec<MojangReleaseProfile>
}

impl MojangVersionManifest {
    pub fn look_up_version(&self, version: String) -> Option<&MojangReleaseProfile> {
        self.versions
            .iter()
            .filter(|v| v.id == version).next()
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct MojangVersionManifestLatest {
    pub release: String,
    pub snapshot: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MojangReleaseProfile {
    pub id: String,
    #[serde(rename = "type")]
    pub release_type: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String
}


pub mod mojang_version_data {
    use crate::types::{Or, OrVec};
    use serde::{Serialize, Deserialize};


    #[derive(Serialize, Deserialize, Debug)]
    pub struct MojangVersionData {
        pub arguments: Option<Arguments>,
        #[serde(rename = "assetIndex")]
        pub asset_index: AssetIndex,
        pub assets: String,
        pub downloads: Downloads,
        pub id: String,
        pub libraries: Vec<Library>,
        pub logging: Logging,
        #[serde(rename = "mainClass")]
        pub main_class: String,
        #[serde(rename = "minecraftArguments")]
        pub minecraft_arguments: Option<String>,
        #[serde(rename = "minimumLauncherVersion")]
        pub minimum_launcher_version: u16,
        #[serde(rename = "releaseTime")]
        pub release_time: String,
        pub time: String,
        #[serde(rename = "type")]
        pub release_type: String
    }
    
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Arguments {
        game: Vec<Or<String, Argument>>,
        jvm: Vec<Or<String, Argument>>
    }
    
    
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Argument {
        rules: Option<Vec<Rule>>,
        value: OrVec<String>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Logging {
        client: LoggingConfig
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct LoggingConfig {
        argument: String,
        file: File,
        #[serde(rename = "type")]
        file_type: String
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Library {
        pub downloads: LibraryDownload,
        pub extract: Option<Extract>,
        pub name: String,
        pub natives: Option<Natives>,
        pub rules: Option<Vec<Rule>>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct LibraryDownload {
        pub classifiers: Option<Classifiers>,
        pub artifact: Option<Artifact>
    }
    
    #[derive(Serialize, Deserialize, Debug)]
   pub struct Extract {
        exclude: Vec<String>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Natives {
        linux: Option<String>,
        osx: Option<String>,
        windows: Option<String>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Rule {
        action: String,
        os: Option<Os>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Os {
        name: Option<String>,
        arch: Option<String>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Classifiers {
        #[serde(rename = "natives-linux")]
        natives_linux: Option<Artifact>,
        #[serde(rename = "natives-osx")]
        natives_osx: Option<Artifact>,
        #[serde(rename = "natives-windows")]
        natives_windows: Option<Artifact>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Downloads {
        pub client: Artifact,
        pub server: Artifact
    }
    
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Artifact {
        pub path: Option<String>,
        pub sha1: String,
        pub size: u32,
        pub url: String
    }

    
    #[derive(Serialize, Deserialize, Debug)]
    pub struct File {
        pub id: String,
        pub sha1: String,
        pub size: u32,
        pub url: String
    }

    
    #[derive(Serialize, Deserialize, Debug)]
    pub struct AssetIndex {
        pub id: String,
        pub sha1: String,
        pub size: u32,
        #[serde(rename = "totalSize")]
        pub total_size: u32,
        pub url: String
    }
}


//https://launchermeta.mojang.com/mc/game/version_manifest.json


//https://adfoc.us/serve/sitelinks/?id=271228&url=https://files.minecraftforge.net/maven/net/minecraftforge/forge/1.15.2-31.2.33/forge-1.15.2-31.2.33-universal.jar