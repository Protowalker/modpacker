use super::mc_data::mojang_version_data::{Arguments, MojangVersionData};
use std::path::Path;
use std::ffi::{OsStr, OsString};
use std::process::Command;
use path_clean::{PathClean};
use rayon::prelude::*;

#[derive(Debug)]
pub enum LaunchError {
    IOError(std::io::Error),
    JSONError(serde_json::Error)
}

pub fn launch_instance(instance_dir: &std::path::Path) -> Result<(), LaunchError> {
    let version_data = std::fs::read_to_string(&instance_dir.join("version_info.json"))?;
    let version_data: MojangVersionData = serde_json::from_str(&version_data[..])?;

    let _java_arguments = if let Some(Arguments {jvm, ..}) = version_data.arguments.clone() {
       Some(jvm)
    } else { None };

    let mut args: Vec<&OsStr> = vec![ 
                                        OsStr::new("-Xms512m"),
                                        OsStr::new("-Xmx2048m"),
                                        OsStr::new("-Duser.language=en"),
                                    ];

    let instance_dir = get_absolute_path(instance_dir)?;

    let mut logging_string = version_data.logging.client.argument.clone();
    let offset = logging_string.find('$').unwrap();
    logging_string.replace_range(offset.., &instance_dir.join("client.xml").to_string_lossy());
    args.push(OsStr::new(&logging_string));

    let natives_path = &instance_dir.join("natives");
    std::fs::create_dir_all(natives_path)?;
    let mut natives_path_arg = OsString::from("-Djava.libaries.path=");
    natives_path_arg.push(natives_path.clone().into_os_string());
    args.push(&*natives_path_arg);

    //if let Some(args) = java_arguments {
    //    args.iter().for_each(|arg| args.push());
    //}


    let lib_dir = &get_absolute_path(Path::new("./libraries"))?;
    let (mut lib_artifacts, nat_artifacts) = downloader::download::get_needed_libraries(&version_data);
    lib_artifacts.append(&mut nat_artifacts.clone());


    nat_artifacts.par_iter()
        .for_each(|nat| {
            let file = std::fs::File::open(lib_dir.join(nat.path.clone().unwrap())).unwrap();
            let mut jar = zip::ZipArchive::new(file).unwrap();

            for i in 0..jar.len() {
                let mut file = jar.by_index(i).unwrap();
                println!("{}", file.name());
                if !file.name().contains("META-INF/") {
                    let mut path = file.sanitized_name().clone();
                    path.pop();
                    let path = natives_path.join(path);
                    std::fs::create_dir_all(&path).unwrap();
                    let mut out = std::fs::File::create(natives_path.join(file.name())).unwrap();
                    std::io::copy(&mut file, &mut out).unwrap();
                }
            }

        });

    let lib_paths: Vec<OsString> = lib_artifacts.iter()
                                 .map(|lib| 
                                      lib_dir.join(lib.path.clone()
                                                           .unwrap())
                                      .into_os_string())
                                 .collect();

    let lib_paths = lib_paths.iter();
    
    
    args.push(OsStr::new("-cp"));
    let mut lib_string = OsString::new();
    lib_paths.for_each(|cur| { lib_string.push(cur); lib_string.push(OsStr::new(":"));}); 

    lib_string.push(instance_dir.join("client.jar"));

    args.push(&*lib_string);
    
    args.push(OsStr::new("net.minecraft.client.main.Main"));

    let game_dir = instance_dir.join("minecraft");
    args.push(OsStr::new("--gameDir"));
    args.push(game_dir.as_os_str());

    let assets_dir = get_absolute_path(std::path::Path::new("./assets"))?;
    args.push(OsStr::new("--assetsDir"));
    args.push(assets_dir.as_os_str());

    args.push(OsStr::new("--assetIndex"));
    args.push(OsStr::new(&version_data.assets));

    args.push(OsStr::new("--version"));
    args.push(OsStr::new(&version_data.id));
    
    args.push(OsStr::new("--accessToken"));
    args.push(OsStr::new("eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhMmY1YTA1OTMxYzY0NDNmOGRmZTJjNTlkYTFkNTQ0ZSIsInlnZ3QiOiJjNWVlYTU5MWZkZWY0MTBlYjIxMmIxMzAzYWFkY2Q4MCIsInNwciI6IjM5MTdkZDczNjc3ZDQzOTg5YTBhYWJlY2Y2NDBjOGI5IiwiaXNzIjoiWWdnZHJhc2lsLUF1dGgiLCJleHAiOjE1OTc0Mzc5OTcsImlhdCI6MTU5NzI2NTE5N30.124RZjpyn2u1IQ1qVQXMGYBO_OZh0L0kdL9KPoT48L0"));

    println!("{:#?}", args);

    let output = Command::new("java")
             .args(args)
             .output()?;
    
    println!("{:?}", output);

    Ok(())
}

fn get_absolute_path<P: AsRef<Path>>(path: P) -> std::io::Result<std::path::PathBuf> {
    let path = path.as_ref();
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    }.clean();
    Ok(absolute_path)
}

impl From<std::io::Error> for LaunchError {
    fn from(error: std::io::Error) -> LaunchError {
        LaunchError::IOError(error)
    }
}

impl From<serde_json::Error> for LaunchError {
    fn from(error: serde_json::Error) -> LaunchError {
        LaunchError::JSONError(error)
    }
}

