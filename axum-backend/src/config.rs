use dotenv_codegen::dotenv;
use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct Config {
    #[validate(range(min = 1, max = 65535, message = "port must be between 1 and 65535"))]
    pub port: i32,
    #[validate(length(min = 1, message = "database_url cannot be empty"))]
    pub database_url: String,
    #[validate(length(min = 1, message = "jwt_secret cannot be empty"))]
    pub jwt_secret: String,
    #[validate(length(min = 1, message = "jwt_expires_in cannot be empty"))]
    pub jwt_expires_in: String,
    #[validate(range(
        min = 1,
        max = 65535,
        message = "jwt_maxage must be between 1 and 65535"
    ))]
    pub jwt_maxage: i32,
    #[validate(length(min = 1, message = "smtp_username cannot be empty"))]
    pub smtp_username: String,
    #[validate(length(min = 1, message = "smtp_password cannot be empty"))]
    pub smtp_password: String,
    #[validate(length(min = 1, message = "smtp_host cannot be empty"))]
    pub smtp_host: String,
    #[validate(range(min = 1, max = 65535))]
    pub smtp_port: u16,
    #[validate(length(min = 1, message = "frontend_origin cannot be empty"))]
    pub frontend_origin: String,
}

// AT COMPILE TIME (common)
const PORT: &str = dotenv!("PORT");
const JWT_EXPIRES_IN: &str = dotenv!("JWT_EXPIRES_IN");
const JWT_MAXAGE: &str = dotenv!("JWT_MAXAGE");
const SMTP_HOST: &str = dotenv!("SMTP_HOST");
const SMTP_PORT: &str = dotenv!("SMTP_PORT");

impl Config {
    pub fn init() -> Result<Config, validator::ValidationErrors> {
        // AT RUN TIME (sensitive)
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
        let frontend_origin =
            std::env::var("FRONTEND_ORIGIN").expect("FRONTEND_ORIGIN must be set");

        let config = Config {
            port: PORT.parse::<i32>().expect("Invalid PORT"),
            database_url,
            jwt_secret,
            jwt_expires_in: JWT_EXPIRES_IN.to_owned(),
            jwt_maxage: JWT_MAXAGE.parse::<i32>().expect("Invalid JWT_MAXAGE"),
            smtp_username,
            smtp_password,
            smtp_host: SMTP_HOST.to_owned(),
            smtp_port: SMTP_PORT.parse::<u16>().expect("Invalid SMTP_PORT"),
            frontend_origin,
        };
        config.validate()?;
        return Ok(config);
    }
}
