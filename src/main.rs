extern crate native_tls;
mod mailbox;

#[macro_use]
extern crate clap;
extern crate imap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let domain = std::env::var("IMAP_DOMAIN").expect("IMAP_DOMAIN env var not found");
    let username = std::env::var("IMAP_USERNAME").expect("IMAP_USERNAME env var not found");
    let password = std::env::var("IMAP_PASSWORD").expect("IMAP_PASSWORD env var not found");
    let mail_box = mailbox::Mailbox::new(domain, username, password);
    let mut session = mail_box.connect().expect("failed to connect to mail box");

    if let Some(matches) = matches.subcommand_matches("status") {
        let mail_box_name = matches.value_of("box").unwrap();
        session
            .print_box_status(mail_box_name.to_string())
            .expect("Failed to print mail box status");
    }

    session.close().expect("Failed to logout from server");
}
