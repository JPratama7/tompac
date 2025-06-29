use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct Configuration {
    pub config: Config,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct Config {
    pub packages: Vec<String>,
    pub repositories: Vec<Repository>,
    pub pacman: PacmanConfiguration,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[non_exhaustive]
#[serde(rename_all = "PascalCase")]
pub struct Repository {
    pub name: String,
    pub sig_level: Option<String>,
    pub server: Vec<String>,
    pub include: Option<String>,
    pub usage: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[non_exhaustive]
#[serde(rename_all = "PascalCase")]
pub struct PacmanConfiguration {
    pub hold_pkg: Vec<String>,
    pub clean_method: String,
    pub architecture: String,

    pub ignore_pkg: Vec<String>,
    pub ignore_group: Vec<String>,
    pub no_upgrade: Vec<String>,
    pub no_extract: Vec<String>,

    pub use_syslog: bool,
    pub color: bool,
    pub no_progress_bar: bool,
    pub check_space: bool,
    pub verbose_pkg_lists: bool,
    pub parallel_downloads: u32,
    pub download_user: String,
    pub disable_sandbox: bool,

    pub sig_level: String,
    pub local_file_sig_level: String,
    pub remote_file_sig_level: String,
}

impl Configuration {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let text = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&text)?)
    }
}
