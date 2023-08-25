use axum_macros::FromRef;


#[derive(Clone, FromRef)]
pub struct EmailService {

}

impl EmailService {
    pub fn new() -> Self {
        Self{}
    }

    pub fn send_activation_mail(&self, email: &str, activation_key: &str) {
        let text = format!("Sending activation mail to {email}, the code is {activation_key}.");
        println!("{text}");
    }
}