pub mod object;
pub mod objstore;
pub mod keystore;

use objstore::filestore::FileStore;
use keystore::keystore::SQLiteKeyStore;

use std::env;

fn main() {

    // PUT an object:
    //  get object size
    //  hash object data
    //  determine object placement
    //  generate uuid
    //  store key in keystore
    //  put data into object store
    //  return uuid
    //
    // GET an object by uuid:
    //  query keystore for uuid
    //  determine object location and size
    //  read object from object store
    //  return object
    //

}
