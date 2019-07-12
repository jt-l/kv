use std::result;
use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::OpenOptions;
use std::io;
use std::process;

use failure::Error;

use serde::{Serialize, Deserialize};
use serde_json;


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
// The command will be serialized before being written to the log, and deseralized upon being read
enum Command {
    Get(String),
    Set(String, String),
    Remove(String),
}

pub struct PkvStore {
    // map contains a mapping from key to a pointer to the command in the log
    map: HashMap<String, String>,    

    // log file
    file: File,
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
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open("log.txt")?;

        // load the db to memory 
        let map = load_db(&f)?;

        Ok(PkvStore {
            map: map,
            file: f,
        })
    }

    /* Set the value of a string key to a string
     * 1. Store the command in the log
     * 2. Store the key and pointer to command in map
     */
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        // check if the key already exists in memory
        if let Some(key) = self.map.get(&key) {
            eprintln!("Failed to set provided key: Key already exists!");
            process::exit(1);
        };

        // create command
        let command = Command::Set(key, value);

        // serialize to a json string
        let serialized_command = serde_json::to_string(&command).unwrap() + "\n";

        // write the serialized command to the log
        self.file.write_all(&serialized_command.into_bytes())?;

        // write the key and value to the hash map
        if let Command::Set(key, value) = command {
            self.map.insert(key, value);
        }

        Ok(())
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

// load the db from the log into memory
fn load_db(f: &File) -> Result<(HashMap<String, String>)> {

    // create buf reader
    let reader = BufReader::new(f);

    // create hash map
    let mut map = HashMap::new();

    for line in reader.lines() {
        // deseralize, line returns a Result so need to specify the type of deseralized_command
        let deserialized_command: Command = serde_json::from_str(&line?)?;

        // execute the command on the map
        match deserialized_command {
            Command::Set(key, value) => map.insert(key, value),
            Command::Remove(key) => map.remove(&key),
            _ => None,
        };
    }

    Ok(map)
}

