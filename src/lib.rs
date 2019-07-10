use std::result;
use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io;

use failure::Error;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
// The command will be serialized before being written to the log, and deseralized upon being read
enum Command {
    Set(String, String),
}

pub struct PkvStore {
    // map contains a mapping from key to a pointer to the command in the log
    map: HashMap<String, String>,    

    // reference to the log
    log: BufReader<File>, 
}

pub type Result<T> = result::Result<T, Error>;

impl PkvStore {

    /* open the PkvStore
     * 1. On start up, if a log exists, the commands in the log are traversed from oldest to newest, and the in
     * memory map is rebuilt
     * 2. A file handler to the log is opened
     * 3. The PkvStore struct is returned
    */
    pub fn open(path: &Path) -> Result<PkvStore> {

        // open the log, if it does not exist create it
        let f = File::open("log.txt")?;

        Ok(PkvStore {
            map: HashMap::new(),
            log: BufReader::new(f),
        })
    }

    /* Set the value of a string key to a string
     * 1. Store the command in the log
     * 2. Store the key and pointer to command in map
     */
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        unimplemented!();
    }

    // Get the string value of a given string key
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        unimplemented!();
    }

    // Remove a given key
    pub fn remove(&mut self, key: String) -> Result<()> {
        unimplemented!();
    }
}
