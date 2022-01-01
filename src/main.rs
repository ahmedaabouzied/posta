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
        let mail_box_name = matches
            .value_of("box")
            .expect("error: status expects a box name as argument");
        session
            .print_box_status(mail_box_name)
            .expect("Failed to print mail box status");
    }

    if let Some(matches) = matches.subcommand_matches("list") {
        let mail_box_name = matches
            .value_of("box")
            .expect("error: list expects a box name as argument");

        let short = match matches.value_of("short") {
            Some(_) => true,
            None => false,
        };
        session
            .list_emails(mail_box_name, short)
            .expect("Failed to list mailbox emails");
    }

    session.close().expect("Failed to logout from server");
}
