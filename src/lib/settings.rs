pub use config::Config;
use config::{File, Environment, ConfigError};
use std::path::Path;
use shellexpand::tilde;

#[derive(Debug, Deserialize)]
pub struct LimitSettings {
    pub forms: u64,
}

#[derive(Debug, Deserialize)]
pub struct RocketSettings {
    pub workers: u16,
    pub log: String,
    pub limits: LimitSettings,
    pub template_dir: String,
    pub address: String,
    pub port: u16,
    pub secret_key: String,
}

#[derive(Debug, Deserialize)]
pub struct DBSettings {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct PathSettings {
    pub data_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: RocketSettings,
    pub database: DBSettings,
    pub path: PathSettings,
}

impl Settings {
    pub fn new(basename: &str) -> Result<Self, ConfigError> {
        let mut settings = Config::default();
        let config_filename_tilde = tilde("~/.config/").into_owned();
        let home_filename_tilde = tilde(&format!("~/.{}", basename)).into_owned();

        let config_filename = Path::new(&config_filename_tilde).join(&basename);
        let home_filename = Path::new(&home_filename_tilde);
        let etc_filename = Path::new("/etc/").join(&basename);

        settings
            .merge(File::from(etc_filename).required(false)).unwrap()
            .merge(File::from(home_filename).required(false)).unwrap()
            .merge(File::from(config_filename).required(false)).unwrap()
            // Add in settings from the environment (with a prefix of BOOKRSS)
            // Eg.. `BOOKRSS_DEBUG=1 ./target/app` would set the `debug` key
            .merge(Environment::with_prefix("BOOKRSS")).unwrap();

        settings.try_into()
    }
}
