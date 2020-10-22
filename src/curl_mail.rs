use crate::util;
use sparkpost::transmission::{
    EmailAddress, Message, Options, Recipient, Transmission,
    TransmissionResponse,
};

/**
 * Struct used to indiciate the api key for sparkpost is and if it is the eu version or not. Also needs the email where you going to send from.
 * (api key, sender email, eu version of api)
 */
pub struct Mailer(String, String, bool);

impl Mailer {
    /**
     * Default mailer is made from a .env config file. Lines in the .env config file to be set.
     * SPARK_KEY: sparkpost api key
     * SENDER: email from where the mail will be send
     * USE_EU: Boolean to indicate if you want to use the EU version, or if not set false to use the USA version.
    */
    pub fn default() -> Result<Mailer,&'static str> {
        let api : String = util::get_value_from_key("SPARK_KEY").expect("SPARK_KEY cannot be empty, cannot send emails without api key.");
        let email_sender : String = util::get_value_from_key("SENDER").expect("SENDER cannot be empty, email must have a name");
        let eu_version : bool = util::get_value_from_key("USE_EU").unwrap_or_else(|| "false".to_owned()).eq("true");
        if !util::control_email(&email_sender) {
            return Err("Email is not correctly built.");
        }
        Ok(
            Mailer(api, email_sender,eu_version)
        )
    }

    pub fn send_mail(&self, email : &str, subject : &str, msg : &str, html_msg : &str) -> Result<(),&'static str> {
        let tm = if self.2 {
            Transmission::new_eu(&self.0.to_owned())
        } else {
            Transmission::new(&self.0.to_owned())
        };
        let mut email_struct = Message::new(EmailAddress::new(&self.1.to_owned(),"",));    
        let recipient = Recipient::from(email.to_owned());
        match tm.send(&email_struct.add_recipient(recipient).options(Options::default()).subject(subject).html(html_msg).text(msg)) {
            Ok(res) => {
                match res {
                    TransmissionResponse::ApiResponse(api_res) => {
                        debug!("API Response: \n {:#?}", api_res);
                    }
                    TransmissionResponse::ApiError(errors) => {
                        error!("Response Errors: \n {:#?}", &errors);
                    }
                }
            }
            Err(error) => {
                error!("error \n {:#?}", error);
            }
        }
        Ok(())
    }
}

#[test]
fn test_send() {
    let mailer = Mailer::default();
    assert!(mailer.unwrap().send_mail("michael28072000@outlook.com", "Test mail", "Test", "<h1>Test</h1>").is_ok())
}