use crate::{GmailnatorInbox, Error};

use std::time::Instant;

const BULK_SIZE:u32 = 1000;

pub struct AddressQueue{

    bulk_vec:Vec<String>,
    
    expiration:u64,
    last_updated:Option<Instant>,

}

impl AddressQueue {

    pub fn new() -> Self { 

        Self { bulk_vec: Vec::new(), last_updated:None, expiration:0 }

    }

    pub fn set_expiration(&mut self, expiration:u64) {

        self.expiration = expiration;

    }

    pub fn pop(&mut self) -> Result<String, Error> {

        if self.should_renew() { self.renew()? }

        Ok(self.bulk_vec.pop().unwrap())

    }

    fn should_renew(&self) -> bool {

        match self.last_updated {

            Some(instant) => { 

                let empty = self.bulk_vec.is_empty(); 
                let expired = instant.elapsed().as_secs() >= self.expiration;
                           
                let renew = empty || expired;

                if renew {

                    print!("Renewing queue. ");
                    
                    if empty { print!("[empty]") }
                    if expired { print!("[expired]") }

                    println!();

                }

                renew

            }
            
            None => true,

        }

    }

    fn renew(&mut self) -> Result<(), Error> {
        
        self.bulk_vec.clear();

        GmailnatorInbox::new_bulk(BULK_SIZE)?
            .iter()
            .for_each(|inbox| self.bulk_vec.push(inbox.get_address().to_string()));

        self.last_updated = Some(Instant::now());

        Ok(())
        
    }

}