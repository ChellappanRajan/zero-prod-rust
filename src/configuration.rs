use config;
#[derive(serde::Deserialize)]
pub struct  Settings{
    pub database:DataBaseSettings,
    pub port:u16
}

#[derive(serde::Deserialize)]
pub struct DataBaseSettings{
pub username:String,
pub password:String,
pub port:u16,
pub host:String,
pub database_name:String
}


pub fn configurations() -> Result<Settings,config::ConfigError>{
    //Init Configuration readers
    let mut settings = config::Config::default();

    settings.merge(config::File::with_name("configuration"))?;  


    settings.try_into()
}