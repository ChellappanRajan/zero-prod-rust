use config;

#[derive(serde::Deserialize)]
pub struct  Settings{
    pub database:DataBaseSettings,
    pub application_port:u16
}

#[derive(serde::Deserialize)]
pub struct DataBaseSettings{
pub username:String,
pub password:String,
pub port:u16,
pub host:String,
pub database_name:String
}

impl DataBaseSettings {
    pub fn connection_string(&self)->String{
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}


pub fn get_configurations() -> Result<Settings,config::ConfigError>{
    //Init Configuration readers
    let mut settings = config::Config::default();

    settings.merge(config::File::with_name("configuration"))?;  


    settings.try_into()
}