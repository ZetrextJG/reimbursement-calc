use handlebars::Handlebars;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

use crate::{config::Config, models::User};

pub struct Email {
    config: Config,
    from: String,
    user: User,
    redirect_url: String,
}

impl Email {
    pub fn new(user: User, url: String, config: Config) -> Self {
        let from = format!("ReCalc <{}>", config.smtp_username.to_owned());

        Email {
            config,
            from,
            user,
            redirect_url: url,
        }
    }

    fn new_transport(&self) -> Result<AsyncSmtpTransport<Tokio1Executor>, anyhow::Error> {
        let creds = Credentials::new(
            self.config.smtp_username.to_owned(),
            self.config.smtp_password.to_owned(),
        );

        let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(
            &self.config.smtp_host.to_owned(),
        )?
        .port(self.config.smtp_port)
        .credentials(creds)
        .build();

        Ok(transport)
    }

    fn render_template(
        &self,
        subject: &str,
        template_name: &str,
    ) -> Result<String, handlebars::RenderError> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_file(
            template_name,
            &format!("./templates/{}.html", template_name),
        )?;
        handlebars.register_template_file("styles", "./templates/partials/styles.html")?;
        handlebars.register_template_file("base", "./templates/layouts/base.html")?;

        let data = serde_json::json!({
            "username": &self.user.username.split_whitespace().next().unwrap(),
            "subject": subject,
            "url": &self.redirect_url
        });

        let content_template = handlebars.render(template_name, &data)?;

        Ok(content_template)
    }

    async fn send_email(
        &self,
        template_name: &str,
        subject: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let html_template = self.render_template(template_name, subject)?;
        let mail_target = format!(
            "{} <{}>",
            self.user.username.as_str(),
            self.user.mail.as_str()
        );
        let email = Message::builder()
            .to(mail_target.parse().unwrap())
            .reply_to(self.from.parse().unwrap())
            .from(self.from.parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(html_template)?;

        let transport = self.new_transport()?;

        transport.send(email).await?;
        Ok(())
    }

    pub async fn send_verification_code(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.send_email("verification_code", "Your account verification code")
            .await
    }
}
