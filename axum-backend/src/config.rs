use dotenv_codegen::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: i32,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub frontend_origin: String,
}

// AT COMPILE TIME (common)
const PORT: &str = dotenv!("PORT");
const JWT_EXPIRES_IN: &str = dotenv!("JWT_EXPIRES_IN");
const JWT_MAXAGE: &str = dotenv!("JWT_MAXAGE");
const SMTP_HOST: &str = dotenv!("SMTP_HOST");
const SMTP_PORT: &str = dotenv!("SMTP_PORT");
const FRONTEND_ORIGIN: &str = dotenv!("FRONTEND_ORIGIN");

impl Config {
    pub fn init() -> Config {
        // AT RUN TIME (sensitive)
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

        Config {
            port: PORT.parse::<i32>().expect("Invalid PORT"),
            database_url,
            jwt_secret,
            jwt_expires_in: JWT_EXPIRES_IN.to_owned(),
            jwt_maxage: JWT_MAXAGE.parse::<i32>().expect("Invalid JWT_MAXAGE"),
            smtp_username,
            smtp_password,
            smtp_host: SMTP_HOST.to_owned(),
            smtp_port: SMTP_PORT.parse::<u16>().expect("Invalid SMTP_PORT"),
            frontend_origin: FRONTEND_ORIGIN.to_owned(),
        }
    }
}
