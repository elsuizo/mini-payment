use serde_aux::field_attributes::deserialize_number_from_string;
#[derive(serde::Deserialize, Clone)]
pub struct ServiceSettings {
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

pub fn get_configuration() -> Result<ServiceSettings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");

    let configuration_directory = base_path.join("configuration");
    // detectamos como esta seteada la variable `APP_ENVIRONMENT` hacemos que sea default a `local`
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse `APP_ENVIRONMENT`");

    let environment_filename = format!("{}.yaml", environment.as_str());
    // inicializamos el lector de configuracion
    let settings = config::Config::builder()
        // agregamos los valores desde un archivo de configuracion llamado `configuration.yaml`
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .build()?;
    // tratamos de convertir los valores que leimos en el type de `Settings`
    settings.try_deserialize::<ServiceSettings>()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported enviroment use either `local` or `production`",
                other
            )),
        }
    }
}
