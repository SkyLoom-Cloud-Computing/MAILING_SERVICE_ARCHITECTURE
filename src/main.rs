use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};


fn main() {
    let smtp_key = "xsmtpsib-ca732fba61fe306291965130c51fcb29b9060ad4e5225778ea17ea3a879d479d-DqFba9tYIhEKSkJB";
    let from_email = "stacknewbie@gmail.com";
    let host = "smtp-relay.brevo.com";
    let to_email = "irfanghat@gmail.com";

    let email: Message = Message::builder()
        .from(from_email.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("Server mail, RUST")
        .body("Message body...".to_string())
        .unwrap();

    let mailer: SmtpTransport = SmtpTransport::relay(&host)
        .unwrap()
        .credentials(Credentials::new(
            from_email.to_string(),
            smtp_key.to_string(),
        ))
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
