use std::net::TcpStream;

use native_tls::TlsStream;

/// Mailbox represents an email account.
pub struct Mailbox {
    domain: String,
    username: String,
    password: String,
}

pub struct Session {
    imap_session: imap::Session<TlsStream<TcpStream>>,
}
impl Session {
    pub fn print_box_status(&mut self, mail_box_name: &str) -> imap::error::Result<()> {
        let mail_box = self.imap_session.select(mail_box_name).unwrap();
        let exists = mail_box.exists;
        let unseen = self.imap_session.search("UNSEEN").unwrap().len();
        let seen = self.imap_session.search("SEEN").unwrap().len();
        println!(
            "Status of inbox \n\
             ===============\n\
            * Total emails = {}\n\
            * Unseen emails = {}\n\
            * Seen emails = {}",
            exists, unseen, seen
        );
        Ok(())
    }

    pub fn close(&mut self) -> imap::error::Result<()> {
        self.imap_session.logout()?;
        Ok(())
    }
}

impl Mailbox {
    pub fn new(domain: String, username: String, password: String) -> Mailbox {
        Mailbox {
            domain,
            username,
            password,
        }
    }
    /// Returns a mailbox session
    pub fn connect(&self) -> imap::error::Result<Session> {
        let tls = native_tls::TlsConnector::builder().build().unwrap();
        let client =
            imap::connect((self.domain.as_str(), 993), self.domain.as_str(), &tls).unwrap();
        let imap_session = client
            .login(self.username.as_str(), self.password.as_str())
            .map_err(|e| e.0)?;
        Ok(Session { imap_session })
    }
}
