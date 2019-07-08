use std::result;
use std::path::Path;

use failure::Error;

#[derive(Default)]
pub struct PkvStore {
}

pub type Result<T> = result::Result<T, Error>;

impl PkvStore {
    pub fn open(path: &Path) -> Result<PkvStore> {

        Ok(PkvStore {
        })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        unimplemented!();
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        unimplemented!();
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        unimplemented!();
    }
}
