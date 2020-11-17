#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]    extern crate rocket;
#[macro_use]    extern crate lazy_static;
                extern crate rocket_contrib;
                extern crate gmailnator;

mod mail;

use mail::AddressQueue;

use gmailnator::{GmailnatorInbox, MailMessage, Error};

use rocket_contrib::json::Json;
use rocket::response::Debug;

use configparser::ini::Ini;
use std::sync::Mutex;

const SETTINGS_FILE:&'static str = "settings.ini";
const DEFAULT_SETTINGS:&'static str = "[server]\nPORT=80\nADDRESS_EXPIRATION=82800";
const INVALID_FILE_MESSAGE:&'static str = "Invalid configuration file, delete it and try again.";

lazy_static! {

    static ref MAIL_QUEUE:Mutex<AddressQueue> = Mutex::new(AddressQueue::new());

}

fn main() -> Result<(), ()> {

    let mut config = Ini::new();

    if config.load(SETTINGS_FILE).is_err() {

        println!("Failed to read default configuation file.");

        config.read(DEFAULT_SETTINGS.to_owned()).expect("Could not read default configuration.");
        config.write(SETTINGS_FILE).expect("Could not write/create the default configuation file."); 

        println!("Created default configuration file with filename {}", SETTINGS_FILE);

        main()?;

    };

    let port = config.get("server", "PORT").expect(INVALID_FILE_MESSAGE);

    let expiration_delay = config.getint("server", "ADDRESS_EXPIRATION").expect(INVALID_FILE_MESSAGE)
                                                                        .expect(INVALID_FILE_MESSAGE);

    MAIL_QUEUE.lock().unwrap().set_expiration(expiration_delay as u64);
                                                                        
    std::env::set_var("ROCKET_PORT", port);
                                                                        
    rocket::ignite().mount("/", routes![new_address, get_messages]).launch();

    Ok(())

}

#[get("/new_address")]
fn new_address() -> Result<String, Debug<Error>> {

    get_debuggable_error(MAIL_QUEUE.lock().unwrap().pop())

}

#[get("/get_messages/<address>")]
fn get_messages(address:String) -> Result<Json<Vec<MailMessage>>, Debug<Error>> {

    let inbox = get_debuggable_error(GmailnatorInbox::from_address(&address))?;

    let messages:Vec<MailMessage> = get_debuggable_error(inbox.get_messages_iter())?.collect();

    Ok(Json(messages))

}

fn get_debuggable_error<T, E>(result:Result<T, E>) -> Result<T, Debug<E>> {

    match result {

        Ok(value) => Ok(value),
        Err(e) => Err(Debug::from(e)),

    }

}