use crate::models::state::State;
use std::fs;
use std::path::PathBuf;

pub struct AppHandler {
    config_path: PathBuf,
}

impl AppHandler {
    pub fn new() -> color_eyre::Result<Self> {
        let config_path = Self::get_config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        Ok(Self { config_path })
    }

    fn get_config_path() -> color_eyre::Result<PathBuf> {
        let app_name = "pseugo";
        let filename = "state.json";

        #[cfg(target_os = "windows")]
        {
            // Windows: %APPDATA%\todo\state.json
            let appdata =
                std::env::var("APPDATA").map_err(|_| color_eyre::eyre::eyre!("APPDATA environment variable not found"))?;
            Ok(PathBuf::from(appdata).join(app_name).join(filename))
        }

        #[cfg(target_os = "macos")]
        {
            // macOS: ~/Library/Application Support/todo/state.json
            let home = std::env::var("HOME").map_err(|_| "HOME environment variable not found")?;
            Ok(PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join(app_name)
                .join(filename))
        }

        #[cfg(target_os = "linux")]
        {
            // Linux: ~/.config/todo/state.json (XDG Base Directory)
            let config_dir = std::env::var("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    let home = std::env::var("HOME").expect("HOME environment variable not found");
                    PathBuf::from(home).join(".config")
                });
            Ok(config_dir.join(app_name).join(filename))
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            // Fallback for other systems
            Ok(PathBuf::from(".").join(filename))
        }
    }

    pub fn save(&self, state: &State) -> color_eyre::Result<()> {
        let json = serde_json::to_string_pretty(state)?;
        fs::write(&self.config_path, json)?;
        Ok(())
    }

    pub fn load(&self, state: &mut State) -> color_eyre::Result<()> {
        if self.config_path.exists() {
            let json = fs::read_to_string(&self.config_path)?;
            let loaded_state: State = serde_json::from_str(&json)?;
            *state = loaded_state;
        } else {
            self.save(state)?;
        }
        Ok(())
    }
}
