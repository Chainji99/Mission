use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use std::env;

#[derive(Clone)]
pub struct EmailService;

impl EmailService {
    pub fn new() -> Self {
        Self
    }

    pub async fn send_welcome_email(&self, to_email: &str, username: &str) -> anyhow::Result<()> {
        let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string());
        let smtp_user = env::var("SMTP_USER").unwrap_or_else(|_| "test@example.com".to_string());
        let smtp_pass = env::var("SMTP_PASS").unwrap_or_else(|_| "password".to_string());
        let from_email = env::var("SMTP_FROM").unwrap_or_else(|_| "no-reply@nebula.com".to_string());

        // If credentials are defaults/dummy, just log and return (mock mode)
        if smtp_user == "test@example.com" {
            println!("--------------------------------------------------");
            println!("(Mock) Sending Welcome Email to: {}", to_email);
            println!("Subject: Welcome to Nebula!");
            println!("Body: Hello {}, welcome to our platform!", username);
            println!("--------------------------------------------------");
            return Ok(());
        }

        let email = Message::builder()
            .from(from_email.parse()?)
            .to(to_email.parse()?)
            .subject("Welcome to Nebula!")
            .body(format!("Hello {}, welcome to Nebula! We are glad to have you.", username))?;

        let creds = Credentials::new(smtp_user, smtp_pass);

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(&smtp_host)?
            .credentials(creds)
            .build();

        // Send the email
        // Note: SmtpTransport::send is blocking, but we are in async. 
        // In a real app, use AsyncSmtpTransport or spawn_blocking.
        // For now, spawn_blocking is safer.
        let mailer = std::sync::Arc::new(mailer);
        let email = std::sync::Arc::new(email);
        
        let mailer_clone = mailer.clone();
        let email_clone = email.clone();

        tokio::task::spawn_blocking(move || {
            match mailer_clone.send(&email_clone) {
                Ok(_) => println!("Email sent successfully!"),
                Err(e) => println!("Could not send email: {:?}", e),
            }
        }).await?;

        Ok(())
    }

    pub async fn send_password_reset_email(&self, to_email: &str, username: &str, token: &str) -> anyhow::Result<()> {
        let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string());
        let smtp_user = env::var("SMTP_USER").unwrap_or_else(|_| "test@example.com".to_string());
        let smtp_pass = env::var("SMTP_PASS").unwrap_or_else(|_| "password".to_string());
        let from_email = env::var("SMTP_FROM").unwrap_or_else(|_| "no-reply@nebula.com".to_string());
        let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:4200".to_string());

        let reset_url = format!("{}/reset-password?token={}", frontend_url, token);

        if smtp_user == "test@example.com" {
            println!("--------------------------------------------------");
            println!("(Mock) Sending Password Reset Email to: {}", to_email);
            println!("Subject: Reset your password");
            println!("Body: Hello {}, click here to reset your password: {}", username, reset_url);
            println!("--------------------------------------------------");
            return Ok(());
        }

        let email = Message::builder()
            .from(from_email.parse()?)
            .to(to_email.parse()?)
            .subject("Reset your Nebula Password")
            .body(format!(
                "Hello {},\n\nYou requested a password reset. Please click the link below to reset your password:\n\n{}\n\nIf you did not request this, please ignore this email.",
                username, reset_url
            ))?;

        let creds = Credentials::new(smtp_user, smtp_pass);
        let mailer = SmtpTransport::relay(&smtp_host)?
            .credentials(creds)
            .build();

        let mailer = std::sync::Arc::new(mailer);
        let email = std::sync::Arc::new(email);
        
        let mailer_clone = mailer.clone();
        let email_clone = email.clone();

        tokio::task::spawn_blocking(move || {
            match mailer_clone.send(&email_clone) {
                Ok(_) => println!("Reset email sent successfully!"),
                Err(e) => println!("Could not send reset email: {:?}", e),
            }
        }).await?;

        Ok(())
    }
}
