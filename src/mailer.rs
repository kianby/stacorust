// --------------------------------------------------------------------------
//  stacorust.lib.mailer
// --------------------------------------------------------------------------

extern crate imap;
extern crate native_tls;
use crate::config::ImapConfig;
use std::borrow::Borrow;

pub fn fetch_inbox(config: ImapConfig) -> Option<imap::types::Fetch> {
    let host_port = format!("{}:{}", config.host, config.port);
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client_result = imap::connect(host_port, config.host, &tls);
    let client = match client_result {
        Ok(imap_client) => imap_client,
        Err(error) => panic!("Problem opening imap client: {:?}", error),
    };

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = match client.login(config.username, config.password) {
        Ok(session) => session,
        Err((e, _)) => {
            panic!("error logging in: {}", e);
        }
    };

    // we want to fetch the first email in the INBOX mailbox
    match imap_session.select("INBOX") {
        Ok(mailbox) => println!("inbox selected: {}", mailbox),
        Err(error) => panic!("Problem opening inbox: {:?}", error),
    };

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    let messages = imap_session.fetch("1", "RFC822").unwrap();
    
    //let message: Option<&imap::types::Fetch> = messages.iter().next();

    let message = match messages.iter().next() {
        Some(m) => Some(m.borrow()),
         None => None    
    };
    

    // be nice to the server and log out
    match imap_session.logout() {
        Ok(_) => println!("logout success"),
        Err(error) => eprintln!("Problem closing session: {:?}", error),
    }

    return message;

    /*
    // extract the message's body
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();



    Ok(Some(body))
    */
}
