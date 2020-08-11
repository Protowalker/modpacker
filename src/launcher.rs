use super::mc_data::mojang_version_data::MojangVersionData;


pub enum LaunchError {
    IOError(std::io::Error),
    JSONError(serde_json::Error)
}

pub fn launch_instance(instance_dir: &std::path::Path) -> Result<(), LaunchError> {
    let version_data = std::fs::read_to_string(&instance_dir.join("version_info.json"))?;
    let version_data: MojangVersionData = serde_json::from_str(&version_data[..])?;

    let java_arguments = if let Some(args) = version_data.arguments {
       Some(args.jvm)
    } else { None };

    Ok(())
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
