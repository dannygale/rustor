
pub mod object;
pub mod objstore;
pub mod keystore;
pub mod freelist;
pub mod blockstore;
pub mod keygen;

//pub use objstore::filestore::FileStore;
pub use objstore::objstore::ObjectStore;
pub use blockstore::blockstore::{BlockStore, BlockDevice};
//use keystore::keystore::SQLiteKeyStore;

pub use freelist::*;

use std::error::Error;
type RResult<T> = Result<T, Box<dyn Error>>;

pub struct GeneralError(String);

use std::fmt;

impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}
impl fmt::Debug for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}


impl GeneralError {
    pub fn new(msg: &str) -> RResult<()> {
        return Err(Box::new(GeneralError(msg.to_string())));
    }
    pub fn from(err: impl Error) -> RResult<()> {
        return GeneralError::new(&err.to_string());
    }
}

impl Error for GeneralError {}
