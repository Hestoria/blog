use dotenv::dotenv;

pub struct Config {
    pub api_port: i32,
    pub api_host: String,
    pub db_url: String,
}

impl Config {
    pub fn from_env() -> Config {
        dotenv().ok();
        let api_host = std::env::var("API_HOST").expect("api host not valid");
        let api_port = std::env::var("API_PORT")
            .expect("api port not valid")
            .parse()
            .expect("api port incorrect format");
        let db_url = std::env::var("DATABASE_URL").expect("DB url not found");
        Config {
            api_port,
            api_host,
            db_url,
        }
    }
}
